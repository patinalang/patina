use std::fmt;

use rustc_abi::Size;
use rustc_data_structures::fx::{FxHashMap, FxIndexSet};
use rustc_macros::{Decodable, Encodable, StableHash};
use rustc_span::Symbol;

use crate::spec::{Arch, RelocModel, Target};

pub struct ModifierInfo {
    pub modifier: char,
    pub result: &'static str,
    pub size: u16,
}

impl From<(char, &'static str, u16)> for ModifierInfo {
    fn from((modifier, result, size): (char, &'static str, u16)) -> Self {
        Self { modifier, result, size }
    }
}

macro_rules! def_reg_class {
    ($arch:ident $arch_regclass:ident {
        $(
            $class:ident,
        )*
    }) => {
        #[derive(Copy, Clone, rustc_macros::Encodable, rustc_macros::Decodable, Debug, Eq, PartialEq, PartialOrd, Hash, rustc_macros::StableHash)]
        #[allow(non_camel_case_types)]
        pub enum $arch_regclass {
            $($class,)*
        }

        impl $arch_regclass {
            pub fn name(self) -> rustc_span::Symbol {
                match self {
                    $(Self::$class => rustc_span::sym::$class,)*
                }
            }

            pub fn parse(name: rustc_span::Symbol) -> Result<Self, &'static [rustc_span::Symbol]> {
                match name {
                    $(
                        rustc_span::sym::$class => Ok(Self::$class),
                    )*
                    _ => Err(&[$(rustc_span::sym::$class),*]),
                }
            }
        }
    }
}

macro_rules! def_regs {
    ($arch:ident $arch_reg:ident $arch_regclass:ident {
        $(
            $reg:ident: $class:ident $(, $extra_class:ident)* = [$reg_name:literal $(, $alias:literal)*] $(% $filter:ident)*,
        )*
        $(
            #error = [$($bad_reg:literal),+] => $error:literal,
        )*
    }) => {
        #[allow(unreachable_code)]
        #[derive(Copy, Clone, rustc_macros::Encodable, rustc_macros::Decodable, Debug, Eq, PartialEq, PartialOrd, Hash, rustc_macros::StableHash)]
        #[allow(non_camel_case_types)]
        pub enum $arch_reg {
            $($reg,)*
        }

        impl $arch_reg {
            pub fn name(self) -> &'static str {
                match self {
                    $(Self::$reg => $reg_name,)*
                }
            }

            pub fn reg_class(self) -> $arch_regclass {
                match self {
                    $(Self::$reg => $arch_regclass::$class,)*
                }
            }

            pub fn parse(name: &str) -> Result<Self, &'static str> {
                match name {
                    $(
                        $($alias)|* | $reg_name => Ok(Self::$reg),
                    )*
                    $(
                        $($bad_reg)|* => Err($error),
                    )*
                    _ => Err("unknown register"),
                }
            }

            pub fn validate(
                self,
                _arch: super::InlineAsmArch,
                _reloc_model: crate::spec::RelocModel,
                _target_features: &rustc_data_structures::fx::FxIndexSet<rustc_span::Symbol>,
                _target: &crate::spec::Target,
                _is_clobber: bool,
            ) -> Result<(), &'static str> {
                let _ = self;
                Err("unsupported")
            }
        }
    }
}

mod aarch64;
mod arm;
mod avr;
mod bpf;
mod csky;
mod hexagon;
mod loongarch;
mod m68k;
mod mips;
mod msp430;
mod nvptx;
mod powerpc;
mod riscv;
mod s390x;
mod sparc;
mod spirv;
mod wasm;
mod x86;

pub use aarch64::{AArch64InlineAsmReg, AArch64InlineAsmRegClass};
pub use arm::{ArmInlineAsmReg, ArmInlineAsmRegClass};
pub use avr::{AvrInlineAsmReg, AvrInlineAsmRegClass};
pub use bpf::{BpfInlineAsmReg, BpfInlineAsmRegClass};
pub use csky::{CSKYInlineAsmReg, CSKYInlineAsmRegClass};
pub use hexagon::{HexagonInlineAsmReg, HexagonInlineAsmRegClass};
pub use loongarch::{LoongArchInlineAsmReg, LoongArchInlineAsmRegClass};
pub use m68k::{M68kInlineAsmReg, M68kInlineAsmRegClass};
pub use mips::{MipsInlineAsmReg, MipsInlineAsmRegClass};
pub use msp430::{Msp430InlineAsmReg, Msp430InlineAsmRegClass};
pub use nvptx::{NvptxInlineAsmReg, NvptxInlineAsmRegClass};
pub use powerpc::{PowerPCInlineAsmReg, PowerPCInlineAsmRegClass};
pub use riscv::{RiscVInlineAsmReg, RiscVInlineAsmRegClass};
pub use s390x::{S390xInlineAsmReg, S390xInlineAsmRegClass};
pub use sparc::{SparcInlineAsmReg, SparcInlineAsmRegClass};
pub use spirv::{SpirVInlineAsmReg, SpirVInlineAsmRegClass};
pub use wasm::{WasmInlineAsmReg, WasmInlineAsmRegClass};
pub use x86::{X86InlineAsmReg, X86InlineAsmRegClass};

#[derive(Copy, Clone, Encodable, Decodable, Debug, Eq, PartialEq, Hash)]
pub enum InlineAsmArch {
    X86,
    X86_64,
    Arm,
    AArch64,
    Arm64EC,
    RiscV32,
    RiscV64,
    Nvptx64,
    Hexagon,
    LoongArch32,
    LoongArch64,
    Mips,
    Mips64,
    PowerPC,
    PowerPC64,
    S390x,
    Sparc,
    Sparc64,
    SpirV,
    Wasm32,
    Wasm64,
    Bpf,
    Avr,
    Msp430,
    M68k,
    CSKY,
}

impl InlineAsmArch {
    pub fn from_arch(arch: &Arch) -> Option<Self> {
        match arch {
            Arch::X86 => Some(Self::X86),
            Arch::X86_64 => Some(Self::X86_64),
            Arch::Arm => Some(Self::Arm),
            Arch::Arm64EC => Some(Self::Arm64EC),
            Arch::AArch64 => Some(Self::AArch64),
            Arch::RiscV32 => Some(Self::RiscV32),
            Arch::RiscV64 => Some(Self::RiscV64),
            Arch::Nvptx64 => Some(Self::Nvptx64),
            Arch::Hexagon => Some(Self::Hexagon),
            Arch::LoongArch32 => Some(Self::LoongArch32),
            Arch::LoongArch64 => Some(Self::LoongArch64),
            Arch::Mips | Arch::Mips32r6 => Some(Self::Mips),
            Arch::Mips64 | Arch::Mips64r6 => Some(Self::Mips64),
            Arch::PowerPC => Some(Self::PowerPC),
            Arch::PowerPC64 => Some(Self::PowerPC64),
            Arch::S390x => Some(Self::S390x),
            Arch::Sparc => Some(Self::Sparc),
            Arch::Sparc64 => Some(Self::Sparc64),
            Arch::SpirV => Some(Self::SpirV),
            Arch::Wasm32 => Some(Self::Wasm32),
            Arch::Wasm64 => Some(Self::Wasm64),
            Arch::Bpf => Some(Self::Bpf),
            Arch::Avr => Some(Self::Avr),
            Arch::Msp430 => Some(Self::Msp430),
            Arch::M68k => Some(Self::M68k),
            Arch::CSky => Some(Self::CSKY),
            Arch::AmdGpu | Arch::Xtensa | Arch::Other(_) => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
#[derive(StableHash, Encodable, Decodable)]
pub enum InlineAsmReg {
    X86(X86InlineAsmReg),
    Arm(ArmInlineAsmReg),
    AArch64(AArch64InlineAsmReg),
    RiscV(RiscVInlineAsmReg),
    Nvptx(NvptxInlineAsmReg),
    PowerPC(PowerPCInlineAsmReg),
    Hexagon(HexagonInlineAsmReg),
    LoongArch(LoongArchInlineAsmReg),
    Mips(MipsInlineAsmReg),
    S390x(S390xInlineAsmReg),
    Sparc(SparcInlineAsmReg),
    SpirV(SpirVInlineAsmReg),
    Wasm(WasmInlineAsmReg),
    Bpf(BpfInlineAsmReg),
    Avr(AvrInlineAsmReg),
    Msp430(Msp430InlineAsmReg),
    M68k(M68kInlineAsmReg),
    CSKY(CSKYInlineAsmReg),
    // Placeholder for invalid register constraints for the current target
    Err,
}

impl InlineAsmReg {
    pub fn name(self) -> &'static str {
        match self {
            Self::X86(r) => r.name(),
            Self::Arm(r) => r.name(),
            Self::AArch64(r) => r.name(),
            Self::RiscV(r) => r.name(),
            Self::PowerPC(r) => r.name(),
            Self::Hexagon(r) => r.name(),
            Self::LoongArch(r) => r.name(),
            Self::Mips(r) => r.name(),
            Self::S390x(r) => r.name(),
            Self::Sparc(r) => r.name(),
            Self::Bpf(r) => r.name(),
            Self::Avr(r) => r.name(),
            Self::Msp430(r) => r.name(),
            Self::M68k(r) => r.name(),
            Self::CSKY(r) => r.name(),
            Self::Err => "<reg>",
        }
    }

    pub fn reg_class(self) -> InlineAsmRegClass {
        match self {
            Self::X86(r) => InlineAsmRegClass::X86(r.reg_class()),
            Self::Arm(r) => InlineAsmRegClass::Arm(r.reg_class()),
            Self::AArch64(r) => InlineAsmRegClass::AArch64(r.reg_class()),
            Self::RiscV(r) => InlineAsmRegClass::RiscV(r.reg_class()),
            Self::PowerPC(r) => InlineAsmRegClass::PowerPC(r.reg_class()),
            Self::Hexagon(r) => InlineAsmRegClass::Hexagon(r.reg_class()),
            Self::LoongArch(r) => InlineAsmRegClass::LoongArch(r.reg_class()),
            Self::Mips(r) => InlineAsmRegClass::Mips(r.reg_class()),
            Self::S390x(r) => InlineAsmRegClass::S390x(r.reg_class()),
            Self::Sparc(r) => InlineAsmRegClass::Sparc(r.reg_class()),
            Self::Bpf(r) => InlineAsmRegClass::Bpf(r.reg_class()),
            Self::Avr(r) => InlineAsmRegClass::Avr(r.reg_class()),
            Self::Msp430(r) => InlineAsmRegClass::Msp430(r.reg_class()),
            Self::M68k(r) => InlineAsmRegClass::M68k(r.reg_class()),
            Self::CSKY(r) => InlineAsmRegClass::CSKY(r.reg_class()),
            Self::Err => InlineAsmRegClass::Err,
        }
    }

    pub fn parse(_arch: InlineAsmArch, _name: Symbol) -> Result<Self, &'static str> {
        Err("unsupported")
    }

    pub fn validate(
        self,
        _arch: InlineAsmArch,
        _reloc_model: RelocModel,
        _target_features: &FxIndexSet<Symbol>,
        _target: &Target,
        _is_clobber: bool,
    ) -> Result<(), &'static str> {
        let _ = self;
        Err("unsupported")
    }

    // NOTE: This function isn't used at the moment, but is needed to support
    // falling back to an external assembler.
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        match self {
            Self::Err => unreachable!("Use of InlineAsmReg::Err"),
            _ => out.write_str(self.name()),
        }
    }

    pub fn overlapping_regs(self, mut cb: impl FnMut(InlineAsmReg)) {
        match self {
            Self::Err => unreachable!("Use of InlineAsmReg::Err"),
            r => cb(r),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
#[derive(StableHash, Encodable, Decodable)]
pub enum InlineAsmRegClass {
    X86(X86InlineAsmRegClass),
    Arm(ArmInlineAsmRegClass),
    AArch64(AArch64InlineAsmRegClass),
    RiscV(RiscVInlineAsmRegClass),
    Nvptx(NvptxInlineAsmRegClass),
    PowerPC(PowerPCInlineAsmRegClass),
    Hexagon(HexagonInlineAsmRegClass),
    LoongArch(LoongArchInlineAsmRegClass),
    Mips(MipsInlineAsmRegClass),
    S390x(S390xInlineAsmRegClass),
    Sparc(SparcInlineAsmRegClass),
    SpirV(SpirVInlineAsmRegClass),
    Wasm(WasmInlineAsmRegClass),
    Bpf(BpfInlineAsmRegClass),
    Avr(AvrInlineAsmRegClass),
    Msp430(Msp430InlineAsmRegClass),
    M68k(M68kInlineAsmRegClass),
    CSKY(CSKYInlineAsmRegClass),
    // Placeholder for invalid register constraints for the current target
    Err,
}

impl InlineAsmRegClass {
    pub fn name(self) -> Symbol {
        match self {
            Self::X86(r) => r.name(),
            Self::Arm(r) => r.name(),
            Self::AArch64(r) => r.name(),
            Self::RiscV(r) => r.name(),
            Self::Nvptx(r) => r.name(),
            Self::PowerPC(r) => r.name(),
            Self::Hexagon(r) => r.name(),
            Self::LoongArch(r) => r.name(),
            Self::Mips(r) => r.name(),
            Self::S390x(r) => r.name(),
            Self::Sparc(r) => r.name(),
            Self::SpirV(r) => r.name(),
            Self::Wasm(r) => r.name(),
            Self::Bpf(r) => r.name(),
            Self::Avr(r) => r.name(),
            Self::Msp430(r) => r.name(),
            Self::M68k(r) => r.name(),
            Self::CSKY(r) => r.name(),
            Self::Err => rustc_span::sym::reg,
        }
    }

    /// Returns a suggested register class to use for this type. This is called
    /// when `supported_types` fails to give a better error
    /// message to the user.
    pub fn suggest_class(self, arch: InlineAsmArch, ty: InlineAsmType) -> Option<Self> {
        match self {
            Self::X86(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::X86),
            Self::Arm(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::Arm),
            Self::AArch64(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::AArch64),
            Self::RiscV(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::RiscV),
            Self::Nvptx(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::Nvptx),
            Self::PowerPC(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::PowerPC),
            Self::Hexagon(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::Hexagon),
            Self::LoongArch(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::LoongArch),
            Self::Mips(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::Mips),
            Self::S390x(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::S390x),
            Self::Sparc(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::Sparc),
            Self::SpirV(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::SpirV),
            Self::Wasm(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::Wasm),
            Self::Bpf(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::Bpf),
            Self::Avr(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::Avr),
            Self::Msp430(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::Msp430),
            Self::M68k(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::M68k),
            Self::CSKY(r) => r.suggest_class(arch, ty).map(InlineAsmRegClass::CSKY),
            Self::Err => unreachable!("Use of InlineAsmRegClass::Err"),
        }
    }

    /// Returns a suggested template modifier to use for this type and an
    /// example of a register named formatted with it.
    ///
    /// Such suggestions are useful if a type smaller than the full register
    /// size is used and a modifier can be used to point to the subregister of
    /// the correct size.
    pub fn suggest_modifier(self, arch: InlineAsmArch, ty: InlineAsmType) -> Option<ModifierInfo> {
        match self {
            Self::X86(r) => r.suggest_modifier(arch, ty),
            Self::Arm(r) => r.suggest_modifier(arch, ty),
            Self::AArch64(r) => r.suggest_modifier(arch, ty),
            Self::RiscV(r) => r.suggest_modifier(arch, ty),
            Self::Nvptx(r) => r.suggest_modifier(arch, ty),
            Self::PowerPC(r) => r.suggest_modifier(arch, ty),
            Self::Hexagon(r) => r.suggest_modifier(arch, ty),
            Self::LoongArch(r) => r.suggest_modifier(arch, ty),
            Self::Mips(r) => r.suggest_modifier(arch, ty),
            Self::S390x(r) => r.suggest_modifier(arch, ty),
            Self::Sparc(r) => r.suggest_modifier(arch, ty),
            Self::SpirV(r) => r.suggest_modifier(arch, ty),
            Self::Wasm(r) => r.suggest_modifier(arch, ty),
            Self::Bpf(r) => r.suggest_modifier(arch, ty),
            Self::Avr(r) => r.suggest_modifier(arch, ty),
            Self::Msp430(r) => r.suggest_modifier(arch, ty),
            Self::M68k(r) => r.suggest_modifier(arch, ty),
            Self::CSKY(r) => r.suggest_modifier(arch, ty),
            Self::Err => unreachable!("Use of InlineAsmRegClass::Err"),
        }
    }

    /// Returns the default modifier for this register and an example of a
    /// register named formatted with it.
    ///
    /// This is only needed when the register class can suggest a modifier, so
    /// that the user can be shown how to get the default behavior without a
    /// warning.
    pub fn default_modifier(self, arch: InlineAsmArch) -> Option<ModifierInfo> {
        match self {
            Self::X86(r) => r.default_modifier(arch),
            Self::Arm(r) => r.default_modifier(arch),
            Self::AArch64(r) => r.default_modifier(arch),
            Self::RiscV(r) => r.default_modifier(arch),
            Self::Nvptx(r) => r.default_modifier(arch),
            Self::PowerPC(r) => r.default_modifier(arch),
            Self::Hexagon(r) => r.default_modifier(arch),
            Self::LoongArch(r) => r.default_modifier(arch),
            Self::Mips(r) => r.default_modifier(arch),
            Self::S390x(r) => r.default_modifier(arch),
            Self::Sparc(r) => r.default_modifier(arch),
            Self::SpirV(r) => r.default_modifier(arch),
            Self::Wasm(r) => r.default_modifier(arch),
            Self::Bpf(r) => r.default_modifier(arch),
            Self::Avr(r) => r.default_modifier(arch),
            Self::Msp430(r) => r.default_modifier(arch),
            Self::M68k(r) => r.default_modifier(arch),
            Self::CSKY(r) => r.default_modifier(arch),
            Self::Err => unreachable!("Use of InlineAsmRegClass::Err"),
        }
    }

    /// Returns a list of supported types for this register class, each with an
    /// options target feature required to use this type.
    ///
    /// At the codegen stage, it is fine to always pass true for `allow_experimental_reg`,
    /// since all the stability checking will have been done in prior stages.
    pub fn supported_types(
        self,
        arch: InlineAsmArch,
        allow_experimental_reg: bool,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            Self::X86(r) => r.supported_types(arch, allow_experimental_reg),
            Self::Arm(r) => r.supported_types(arch),
            Self::AArch64(r) => r.supported_types(arch),
            Self::RiscV(r) => r.supported_types(arch),
            Self::Nvptx(r) => r.supported_types(arch),
            Self::PowerPC(r) => r.supported_types(arch),
            Self::Hexagon(r) => r.supported_types(arch),
            Self::LoongArch(r) => r.supported_types(arch),
            Self::Mips(r) => r.supported_types(arch),
            Self::S390x(r) => r.supported_types(arch),
            Self::Sparc(r) => r.supported_types(arch),
            Self::SpirV(r) => r.supported_types(arch),
            Self::Wasm(r) => r.supported_types(arch),
            Self::Bpf(r) => r.supported_types(arch),
            Self::Avr(r) => r.supported_types(arch),
            Self::Msp430(r) => r.supported_types(arch),
            Self::M68k(r) => r.supported_types(arch),
            Self::CSKY(r) => r.supported_types(arch),
            Self::Err => unreachable!("Use of InlineAsmRegClass::Err"),
        }
    }

    pub fn parse(
        _arch: InlineAsmArch,
        _name: Symbol,
    ) -> Result<Self, &'static [rustc_span::Symbol]> {
        Err(&[])
    }

    /// Returns the list of template modifiers that can be used with this
    /// register class.
    pub fn valid_modifiers(self, arch: InlineAsmArch) -> &'static [char] {
        match self {
            Self::X86(r) => r.valid_modifiers(arch),
            Self::Arm(r) => r.valid_modifiers(arch),
            Self::AArch64(r) => r.valid_modifiers(arch),
            Self::RiscV(r) => r.valid_modifiers(arch),
            Self::Nvptx(r) => r.valid_modifiers(arch),
            Self::PowerPC(r) => r.valid_modifiers(arch),
            Self::Hexagon(r) => r.valid_modifiers(arch),
            Self::LoongArch(r) => r.valid_modifiers(arch),
            Self::Mips(r) => r.valid_modifiers(arch),
            Self::S390x(r) => r.valid_modifiers(arch),
            Self::Sparc(r) => r.valid_modifiers(arch),
            Self::SpirV(r) => r.valid_modifiers(arch),
            Self::Wasm(r) => r.valid_modifiers(arch),
            Self::Bpf(r) => r.valid_modifiers(arch),
            Self::Avr(r) => r.valid_modifiers(arch),
            Self::Msp430(r) => r.valid_modifiers(arch),
            Self::M68k(r) => r.valid_modifiers(arch),
            Self::CSKY(r) => r.valid_modifiers(arch),
            Self::Err => unreachable!("Use of InlineAsmRegClass::Err"),
        }
    }

    /// Returns whether registers in this class can only be used as clobbers
    /// and not as inputs/outputs.
    ///
    /// At the codegen stage, it is fine to always pass true for `allow_experimental_reg`,
    /// since all the stability checking will have been done in prior stages.
    pub fn is_clobber_only(self, arch: InlineAsmArch, allow_experimental_reg: bool) -> bool {
        self.supported_types(arch, allow_experimental_reg).is_empty()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
#[derive(StableHash, Encodable, Decodable)]
pub enum InlineAsmRegOrRegClass {
    Reg(InlineAsmReg),
    RegClass(InlineAsmRegClass),
}

impl InlineAsmRegOrRegClass {
    pub fn reg_class(self) -> InlineAsmRegClass {
        match self {
            Self::Reg(r) => r.reg_class(),
            Self::RegClass(r) => r,
        }
    }
}

impl fmt::Display for InlineAsmRegOrRegClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reg(r) => write!(f, "\"{}\"", r.name()),
            Self::RegClass(r) => write!(f, "{}", r.name()),
        }
    }
}

/// Set of types which can be used with a particular register class.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InlineAsmType {
    I8,
    I16,
    I32,
    I64,
    I128,
    F16,
    F32,
    F64,
    F128,
    VecI8(u64),
    VecI16(u64),
    VecI32(u64),
    VecI64(u64),
    VecI128(u64),
    VecF16(u64),
    VecF32(u64),
    VecF64(u64),
    VecF128(u64),
}

impl InlineAsmType {
    pub fn is_integer(self) -> bool {
        matches!(self, Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::I128)
    }

    pub fn size(self) -> Size {
        Size::from_bytes(match self {
            Self::I8 => 1,
            Self::I16 => 2,
            Self::I32 => 4,
            Self::I64 => 8,
            Self::I128 => 16,
            Self::F16 => 2,
            Self::F32 => 4,
            Self::F64 => 8,
            Self::F128 => 16,
            Self::VecI8(n) => n * 1,
            Self::VecI16(n) => n * 2,
            Self::VecI32(n) => n * 4,
            Self::VecI64(n) => n * 8,
            Self::VecI128(n) => n * 16,
            Self::VecF16(n) => n * 2,
            Self::VecF32(n) => n * 4,
            Self::VecF64(n) => n * 8,
            Self::VecF128(n) => n * 16,
        })
    }
}

impl fmt::Display for InlineAsmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::I8 => f.write_str("i8"),
            Self::I16 => f.write_str("i16"),
            Self::I32 => f.write_str("i32"),
            Self::I64 => f.write_str("i64"),
            Self::I128 => f.write_str("i128"),
            Self::F16 => f.write_str("f16"),
            Self::F32 => f.write_str("f32"),
            Self::F64 => f.write_str("f64"),
            Self::F128 => f.write_str("f128"),
            Self::VecI8(n) => write!(f, "i8x{n}"),
            Self::VecI16(n) => write!(f, "i16x{n}"),
            Self::VecI32(n) => write!(f, "i32x{n}"),
            Self::VecI64(n) => write!(f, "i64x{n}"),
            Self::VecI128(n) => write!(f, "i128x{n}"),
            Self::VecF16(n) => write!(f, "f16x{n}"),
            Self::VecF32(n) => write!(f, "f32x{n}"),
            Self::VecF64(n) => write!(f, "f64x{n}"),
            Self::VecF128(n) => write!(f, "f128x{n}"),
        }
    }
}

/// Returns the full set of allocatable registers for a given architecture.
///
/// The registers are structured as a map containing the set of allocatable
/// registers in each register class. A particular register may be allocatable
/// from multiple register classes, in which case it will appear multiple times
/// in the map.
// NOTE: This function isn't used at the moment, but is needed to support
// falling back to an external assembler.
pub fn allocatable_registers(
    _arch: InlineAsmArch,
    _reloc_model: RelocModel,
    _target_features: &FxIndexSet<Symbol>,
    _target: &crate::spec::Target,
) -> FxHashMap<InlineAsmRegClass, FxIndexSet<InlineAsmReg>> {
    FxHashMap::default()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
#[derive(StableHash, Encodable, Decodable)]
pub enum InlineAsmClobberAbi {
    X86,
    X86_64Win,
    X86_64SysV,
    Arm,
    AArch64,
    AArch64NoX18,
    Arm64EC,
    Avr,
    RiscV,
    RiscVE,
    LoongArch,
    PowerPC,
    PowerPCSPE,
    S390x,
    Bpf,
    Msp430,
}

impl InlineAsmClobberAbi {
    /// Parses a clobber ABI for the given target, or returns a list of supported
    /// clobber ABIs for the target.
    pub fn parse(
        _arch: InlineAsmArch,
        _target: &Target,
        _target_features: &FxIndexSet<Symbol>,
        _name: Symbol,
    ) -> Result<Self, &'static [&'static str]> {
        Err(&[])
    }

    /// Returns the set of registers which are clobbered by this ABI.
    pub fn clobbered_regs(self) -> &'static [InlineAsmReg] {
        &[]
    }
}
