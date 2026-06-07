use rustc_target::spec::{CfgAbi, Env, Os, Target};

pub fn is_mingw_gnu_toolchain(target: &Target) -> bool {
    target.os == Os::Windows && target.env == Env::Gnu && target.cfg_abi == CfgAbi::Unspecified
}
