use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    Avr AvrInlineAsmRegClass {
        reg,
        reg_upper,
        reg_pair,
        reg_iw,
        reg_ptr,
    }
}

impl AvrInlineAsmRegClass {
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
    Avr AvrInlineAsmReg AvrInlineAsmRegClass {
        r2: reg = ["r2"],
        r3: reg = ["r3"],
        r4: reg = ["r4"],
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
        r16: reg, reg_upper = ["r16"],
        r17: reg, reg_upper = ["r17"],
        r18: reg, reg_upper = ["r18"],
        r19: reg, reg_upper = ["r19"],
        r20: reg, reg_upper = ["r20"],
        r21: reg, reg_upper = ["r21"],
        r22: reg, reg_upper = ["r22"],
        r23: reg, reg_upper = ["r23"],
        r24: reg, reg_upper = ["r24"],
        r25: reg, reg_upper = ["r25"],
        r26: reg, reg_upper = ["r26", "XL"],
        r27: reg, reg_upper = ["r27", "XH"],
        r30: reg, reg_upper = ["r30", "ZL"],
        r31: reg, reg_upper = ["r31", "ZH"],

        r3r2: reg_pair = ["r3r2"],
        r5r4: reg_pair = ["r5r4"],
        r7r6: reg_pair = ["r7r6"],
        r9r8: reg_pair = ["r9r8"],
        r11r10: reg_pair = ["r11r10"],
        r13r12: reg_pair = ["r13r12"],
        r15r14: reg_pair = ["r15r14"],
        r17r16: reg_pair = ["r17r16"],
        r19r18: reg_pair = ["r19r18"],
        r21r20: reg_pair = ["r21r20"],
        r23r22: reg_pair = ["r23r22"],

        r25r24: reg_iw, reg_pair = ["r25r24"],

        X: reg_ptr, reg_iw, reg_pair = ["r27r26", "X"],
        Z: reg_ptr, reg_iw, reg_pair = ["r31r30", "Z"],

            // If this changes within LLVM, the compiler might use the registers
            // in the future. This must be reflected in the set of clobbered
            // registers, else the clobber ABI implementation is *unsound*, as
            // this generates invalid code (register is not marked as clobbered
            // but may change the register content).
    }
}
