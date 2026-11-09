//! The OTP application resource file, `<otp_app_name>.app`.

use rustc_metadata::creader::CStore;
use rustc_middle::bug;
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::LOCAL_CRATE;
use rustc_span::sym;
use serde::Serialize;

use crate::erlang::{self, Charlist, abstract_format as eaf};

/// `{application, Name, Properties}`
#[derive(Serialize)]
#[serde(rename = "application")]
struct Application<'a> {
    name: eaf::Symbol,
    properties: [Property<'a>; 5],
}

/// `{variant_snake_case_name, value}`
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum Property<'a> {
    Description { value: Charlist<'a> },
    Vsn { value: Charlist<'a> },
    Modules { value: Vec<eaf::Symbol> },
    Registered { value: Vec<eaf::Symbol> },
    Applications { value: Vec<eaf::Symbol> },
}

/// The application specification of the local package, as an Erlang's External Term.
/// See https://www.erlang.org/doc/apps/kernel/app.html .
pub(super) fn app_term(
    tcx: TyCtxt<'_>,
    version: &str,
    description: &str,
) -> erlang::Result<Vec<u8>> {
    let modules = tcx.beam_modules(LOCAL_CRATE).iter().copied().map(eaf::Symbol).collect();
    let applications = [sym::kernel, sym::stdlib]
        .into_iter()
        .chain(dependency_applications(tcx))
        .map(eaf::Symbol)
        .collect();

    erlang::to_bytes(&Application {
        name: eaf::Symbol(tcx.otp_app_name(LOCAL_CRATE)),
        properties: [
            Property::Description { value: Charlist(description) },
            Property::Vsn { value: Charlist(version) },
            Property::Modules { value: modules },
            Property::Registered { value: Vec::new() },
            Property::Applications { value: applications },
        ],
    })
}

/// The OTP applications of the direct dependencies, the packages given with `--extern`.
fn dependency_applications(tcx: TyCtxt<'_>) -> impl Iterator<Item = rustc_span::Symbol> {
    let cstore = CStore::from_tcx(tcx);
    tcx.sess.opts.externs.iter().map(move |(name, _)| {
        // Every extern is forced, and so resolved even if unused.
        let crate_num = cstore
            .resolved_extern_crate(rustc_span::Symbol::intern(name))
            .unwrap_or_else(|| bug!("package dependency `{name}` was not resolved"));

        tcx.otp_app_name(crate_num)
    })
}
