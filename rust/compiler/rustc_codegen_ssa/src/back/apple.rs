pub(super) use rustc_target::spec::apple::OSVersion;
use rustc_target::spec::{Env, Os, Target};

pub(super) fn macho_platform(target: &Target) -> u32 {
    match (&target.os, &target.env) {
        (Os::MacOs, _) => object::macho::PLATFORM_MACOS,
        (Os::IOs, Env::MacAbi) => object::macho::PLATFORM_MACCATALYST,
        (Os::IOs, Env::Sim) => object::macho::PLATFORM_IOSSIMULATOR,
        (Os::IOs, _) => object::macho::PLATFORM_IOS,
        (Os::WatchOs, Env::Sim) => object::macho::PLATFORM_WATCHOSSIMULATOR,
        (Os::WatchOs, _) => object::macho::PLATFORM_WATCHOS,
        (Os::TvOs, Env::Sim) => object::macho::PLATFORM_TVOSSIMULATOR,
        (Os::TvOs, _) => object::macho::PLATFORM_TVOS,
        (Os::VisionOs, Env::Sim) => object::macho::PLATFORM_XROSSIMULATOR,
        (Os::VisionOs, _) => object::macho::PLATFORM_XROS,
        (os, env) => unreachable!("invalid os '{os}' / env '{env}' combination for Apple target"),
    }
}

pub(super) fn add_version_to_llvm_target(
    llvm_target: &str,
    deployment_target: OSVersion,
) -> String {
    let mut components = llvm_target.split("-");
    let arch = components.next().expect("apple target should have arch");
    let vendor = components.next().expect("apple target should have vendor");
    let os = components.next().expect("apple target should have os");
    let environment = components.next();
    assert_eq!(components.next(), None, "too many LLVM triple components");

    assert!(
        !os.contains(|c: char| c.is_ascii_digit()),
        "LLVM target must not already be versioned"
    );

    let version = deployment_target.fmt_full();
    if let Some(env) = environment {
        // Insert version into OS, before environment
        format!("{arch}-{vendor}-{os}{version}-{env}")
    } else {
        format!("{arch}-{vendor}-{os}{version}")
    }
}
