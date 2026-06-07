use rustc_middle::ty::TyCtxt;

pub fn test_abi(tcx: TyCtxt<'_>) {
    if !tcx.features().rustc_attrs() {
        // if the `rustc_attrs` feature is not enabled, don't bother testing ABI
        return;
    }
}
