use rustc_apfloat::Float;
use rustc_apfloat::ieee::Double;
use rustc_ast::{self as ast};
use rustc_hir::LangItem;
use rustc_middle::bug;
use rustc_middle::ty::{self, LitToConstInput, ScalarInt, Ty, TyCtxt, TypeVisitableExt as _};
use rustc_span::Symbol;
use tracing::trace;

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
        let width = tcx.data_layout.pointer_size();
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
        (ast::LitKind::Byte(n), _) => (ty::ValTree::from_scalar_int(tcx, n.into()), tcx.types.int),
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
