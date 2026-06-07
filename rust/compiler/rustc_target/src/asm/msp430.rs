use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    Msp430 Msp430InlineAsmRegClass {
        reg,
    }
}

impl Msp430InlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<ModifierInfo> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<ModifierInfo> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        &[]
    }
}

def_regs! {
    Msp430 Msp430InlineAsmReg Msp430InlineAsmRegClass {
        r5: reg = ["r5"],
        r6: reg = ["r6"],
        r7: reg = ["r7"],
        r8: reg = ["r8"],
        r9: reg = ["r9"],
        r10: reg = ["r10"],
        r11: reg = ["r11"],
        r12: reg = ["r12"],
        r13: reg = ["r13"],
        r14: reg = ["r14"],
        r15: reg = ["r15"],

    }
}
