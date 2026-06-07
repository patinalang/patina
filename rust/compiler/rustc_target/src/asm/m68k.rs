use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    M68k M68kInlineAsmRegClass {
        reg,
        reg_addr,
        reg_data,
    }
}

impl M68kInlineAsmRegClass {
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
    M68k M68kInlineAsmReg M68kInlineAsmRegClass {
        d0: reg, reg_data = ["d0"],
        d1: reg, reg_data = ["d1"],
        d2: reg, reg_data = ["d2"],
        d3: reg, reg_data = ["d3"],
        d4: reg, reg_data = ["d4"],
        d5: reg, reg_data = ["d5"],
        d6: reg, reg_data = ["d6"],
        d7: reg, reg_data = ["d7"],
        a0: reg, reg_addr = ["a0"],
        a1: reg, reg_addr = ["a1"],
        a2: reg, reg_addr = ["a2"],
        a3: reg, reg_addr = ["a3"],
    }
}
