use std::borrow::Cow;
use std::fmt;

use rustc_errors::{DiagArgValue, IntoDiagArg};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CguReuse {
    No,
    PreLto,
    PostLto,
}

impl fmt::Display for CguReuse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CguReuse::No => write!(f, "No"),
            CguReuse::PreLto => write!(f, "PreLto"),
            CguReuse::PostLto => write!(f, "PostLto"),
        }
    }
}

impl IntoDiagArg for CguReuse {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.to_string()))
    }
}
