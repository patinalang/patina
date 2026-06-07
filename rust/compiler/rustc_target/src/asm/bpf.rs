use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    Bpf BpfInlineAsmRegClass {
        reg,
        wreg,
    }
}

impl BpfInlineAsmRegClass {
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
    Bpf BpfInlineAsmReg BpfInlineAsmRegClass {
        r0: reg = ["r0"],
        r1: reg = ["r1"],
        r2: reg = ["r2"],
        r3: reg = ["r3"],
        r4: reg = ["r4"],
        r5: reg = ["r5"],
        r6: reg = ["r6"],
        r7: reg = ["r7"],
        r8: reg = ["r8"],
        r9: reg = ["r9"],
        w0: wreg = ["w0"],
        w1: wreg = ["w1"],
        w2: wreg = ["w2"],
        w3: wreg = ["w3"],
        w4: wreg = ["w4"],
        w5: wreg = ["w5"],
        w6: wreg = ["w6"],
        w7: wreg = ["w7"],
        w8: wreg = ["w8"],
        w9: wreg = ["w9"],

    }
}
