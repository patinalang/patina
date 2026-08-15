use rustc_apfloat::Float;
use rustc_apfloat::ieee::Double;
use rustc_ast::{self as ast};
use rustc_hir as hir;
use rustc_hir::LangItem;
use rustc_hir::def::DefKind;
use rustc_middle::mir::ConstValue;
use rustc_middle::mir::interpret::{
    EvalToConstValueResult, EvalToValTreeResult, GlobalId, ReportedErrorInfo, Scalar,
    ValTreeCreationError,
};
use rustc_middle::ty::{
    self, LitToConstInput, PseudoCanonicalInput, ScalarInt, Ty, TyCtxt, TypeVisitableExt as _,
};
use rustc_middle::{bug, error};
use rustc_span::Symbol;
use tracing::{debug, instrument, trace};

fn parse_float_into_scalar(num: Symbol, neg: bool) -> Option<ScalarInt> {
    let num = num.as_str();
    let Ok(rust_f) = num.parse::<f64>() else { return None };
    let mut f = num
        .parse::<Double>()
        .unwrap_or_else(|e| panic!("apfloat::ieee::Double failed to parse `{num}`: {e:?}"));

    assert!(
        u128::from(rust_f.to_bits()) == f.to_bits(),
        "apfloat::ieee::Double gave different result for `{}`: \
         {}({:#x}) vs Rust's {}({:#x})",
        rust_f,
        f,
        f.to_bits(),
        Double::from_bits(rust_f.to_bits().into()),
        rust_f.to_bits()
    );

    if neg {
        f = -f;
    }

    Some(ScalarInt::from(f))
}

pub(crate) fn lit_to_const<'tcx>(
    tcx: TyCtxt<'tcx>,
    lit_input: LitToConstInput<'tcx>,
) -> Option<ty::Value<'tcx>> {
    let LitToConstInput { lit, ty: expected_ty, neg } = lit_input;

    if expected_ty.error_reported().is_err() {
        return None;
    }

    let trunc = |n| {
        let width = tcx.types.int.primitive_size(tcx);
        trace!("trunc {} with size {} and shift {}", n, width.bits(), 128 - width.bits());
        let result = width.truncate(n);
        trace!("trunc result: {}", result);

        ScalarInt::try_from_uint(result, width)
            .unwrap_or_else(|| bug!("expected to create ScalarInt from uint {:?}", result))
    };

    let (valtree, valtree_ty) = match (lit, expected_ty.map(|ty| ty.kind())) {
        (ast::LitKind::Str(s, _), _) => {
            let str_bytes = s.as_str().as_bytes();
            let valtree_ty = tcx.types.str_;
            (ty::ValTree::from_raw_bytes(tcx, str_bytes), valtree_ty)
        }
        (ast::LitKind::ByteStr(byte_sym, _), Some(ty::Ref(_, inner_ty, _)))
            if let ty::Slice(ty) | ty::Array(ty, _) = inner_ty.kind()
                && let ty::Int = ty.kind() =>
        {
            (ty::ValTree::from_raw_bytes(tcx, byte_sym.as_byte_str()), expected_ty.unwrap())
        }
        (
            ast::LitKind::ByteStr(byte_sym, _),
            Some(ty::Slice(inner_ty) | ty::Array(inner_ty, _)),
        ) if tcx.features().deref_patterns()
            && let ty::Int = inner_ty.kind() =>
        {
            // Byte string literal patterns may have type `[u8]` or `[u8; N]` if `deref_patterns` is
            // enabled, in order to allow, e.g., `deref!(b"..."): Vec<u8>`.
            (ty::ValTree::from_raw_bytes(tcx, byte_sym.as_byte_str()), expected_ty.unwrap())
        }
        (ast::LitKind::ByteStr(byte_sym, _), _) => {
            let valtree = ty::ValTree::from_raw_bytes(tcx, byte_sym.as_byte_str());
            let valtree_ty = Ty::new_array(tcx, tcx.types.int, byte_sym.as_byte_str().len() as u64);
            (valtree, valtree_ty)
        }
        (ast::LitKind::Byte(n), _) => {
            (ty::ValTree::from_scalar_int(tcx, trunc(n.into())), tcx.types.int)
        }
        (ast::LitKind::CStr(byte_sym, _), _)
            if let Some(cstr_def_id) = tcx.lang_items().get(LangItem::CStr) =>
        {
            // A CStr is a newtype around a byte slice, so we create the inner slice here.
            // We need a branch for each "level" of the data structure.
            let cstr_ty = tcx.type_of(cstr_def_id).skip_binder();
            let bytes = ty::ValTree::from_raw_bytes(tcx, byte_sym.as_byte_str());
            let valtree =
                ty::ValTree::from_branches(tcx, [ty::Const::new_value(tcx, bytes, cstr_ty)]);
            let valtree_ty = Ty::new_imm_ref(tcx, tcx.lifetimes.re_static, cstr_ty);
            (valtree, valtree_ty)
        }
        (ast::LitKind::Int(n), _) => {
            let scalar_int = trunc(if neg { u128::wrapping_neg(n.get()) } else { n.get() });
            (ty::ValTree::from_scalar_int(tcx, scalar_int), tcx.types.int)
        }
        (ast::LitKind::Bool(b), _) => (ty::ValTree::from_scalar_int(tcx, b.into()), tcx.types.bool),
        (ast::LitKind::Float(n, _), _) => {
            let bits = parse_float_into_scalar(n, neg)?;
            (ty::ValTree::from_scalar_int(tcx, bits), Ty::new_float(tcx))
        }
        (ast::LitKind::Char(c), _) => (ty::ValTree::from_scalar_int(tcx, c.into()), tcx.types.char),
        (ast::LitKind::Err(_), _) => return None,
        _ => return None,
    };

    Some(ty::Value { ty: valtree_ty, valtree })
}

/// Limited `eval_to_valtree`: only named/anon consts whose body is a literal.
/// Full CTFE is not wired up yet, so `key.typing_env` cannot affect the supported subset.
#[instrument(level = "debug", skip(tcx), ret)]
pub(crate) fn eval_to_valtree<'tcx>(
    tcx: TyCtxt<'tcx>,
    key: PseudoCanonicalInput<'tcx, GlobalId<'tcx>>,
) -> EvalToValTreeResult<'tcx> {
    let cid = key.value;
    if cid.promoted.is_some() {
        return Err(ValTreeCreationError::Unevaluable);
    }

    let def_id = cid.instance.def_id();
    let Some(local_def_id) = def_id.as_local() else {
        // Cross-crate: encoded valtrees should already be available via metadata paths.
        // Until then, refuse.
        return Err(ValTreeCreationError::Unevaluable);
    };

    match tcx.def_kind(def_id) {
        DefKind::Const { .. }
        | DefKind::AssocConst { .. }
        | DefKind::AnonConst
        | DefKind::InlineConst => {}
        _ => bug!("eval_to_valtree: {def_id:?} is not a const"),
    }

    let ty =
        tcx.type_of(cid.instance.def_id()).instantiate(tcx, cid.instance.args).skip_normalization();

    let Some(body) = tcx.hir_maybe_body_owned_by(local_def_id) else {
        return Err(ValTreeCreationError::Unevaluable);
    };
    let expr = body.value;

    let (expr, neg) = match expr.kind {
        hir::ExprKind::Unary(hir::UnOp::Neg, inner) => (inner, true),
        _ => (expr, false),
    };
    let hir::ExprKind::Lit(lit) = expr.kind else {
        debug!(?expr.kind, "const body is not a literal; CTFE not available");
        return Err(ValTreeCreationError::Unevaluable);
    };
    let lit_input = LitToConstInput { lit: lit.node, ty: Some(ty), neg };

    match lit_to_const(tcx, lit_input) {
        Some(value) => Ok(value.valtree),
        None => match lit.node {
            ast::LitKind::Err(guar) => Err(ValTreeCreationError::ErrorHandled(
                ReportedErrorInfo::non_const_eval_error(guar).into(),
            )),
            _ => Err(ValTreeCreationError::Unevaluable),
        },
    }
}

/// Limited `eval_to_const_value_raw`: literal const bodies only, via `eval_to_valtree`.
#[instrument(level = "debug", skip(tcx), ret)]
pub(crate) fn eval_to_const_value_raw<'tcx>(
    tcx: TyCtxt<'tcx>,
    key: PseudoCanonicalInput<'tcx, GlobalId<'tcx>>,
) -> EvalToConstValueResult<'tcx> {
    match eval_to_valtree(tcx, key) {
        Ok(valtree) => {
            let ty = tcx
                .type_of(key.value.instance.def_id())
                .instantiate(tcx, key.value.instance.args)
                .skip_normalization();

            Ok(valtree_to_const_value(tcx, ty::Value { ty, valtree }))
        }
        Err(ValTreeCreationError::ErrorHandled(err)) => Err(err),
        Err(_) => {
            let span = tcx.def_span(key.value.instance.def_id());
            let guar = tcx.dcx().emit_err(error::UnevaluableConst { span });
            Err(ReportedErrorInfo::allowed_in_infallible(guar).into())
        }
    }
}

/// Convert a type-level value into a MIR `ConstValue` for the literal subset we support.
pub(crate) fn valtree_to_const_value<'tcx>(tcx: TyCtxt<'tcx>, cv: ty::Value<'tcx>) -> ConstValue {
    match *cv.ty.kind() {
        ty::Bool | ty::Char | ty::Int | ty::Float => ConstValue::Scalar(Scalar::Int(cv.to_leaf())),
        ty::FnDef(..) => ConstValue::ZeroSized,
        ty::Tuple(fields) if fields.is_empty() => ConstValue::ZeroSized,
        ty::Str | ty::Ref(..) if cv.ty.peel_refs().is_str() => {
            let bytes = cv
                .try_to_raw_bytes(tcx)
                .unwrap_or_else(|| bug!("str valtree did not lower to raw bytes: {cv:?}"));
            let alloc_id = tcx.allocate_bytes_dedup(bytes, /* salt */ 0);
            ConstValue::Slice { alloc_id, meta: bytes.len() as u64 }
        }
        _ => bug!("valtree_to_const_value: unsupported type {:?}", cv.ty),
    }
}
