use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    Sparc SparcInlineAsmRegClass {
        reg,
        yreg,
    }
}

impl SparcInlineAsmRegClass {
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
    Sparc SparcInlineAsmReg SparcInlineAsmRegClass {
        // FIXME:
        // - LLVM has reserve-{g,o,l,i}N feature to reserve each general-purpose registers.
        // - g2-g4 are reserved for application (optional in both LLVM and GCC, and GCC has -mno-app-regs option to reserve them).
        // There are currently no builtin targets that use them, but in the future they may need to
        // be supported via options similar to AArch64's -Z fixed-x18.
        r2: reg = ["r2", "g2"], //
        r3: reg = ["r3", "g3"], //
        r4: reg = ["r4", "g4"], //
        r5: reg = ["r5", "g5"],
        r8: reg = ["r8", "o0"], //
        r9: reg = ["r9", "o1"], //
        r10: reg = ["r10", "o2"], //
        r11: reg = ["r11", "o3"], //
        r12: reg = ["r12", "o4"], //
        r13: reg = ["r13", "o5"], //
        r15: reg = ["r15", "o7"], //
        r16: reg = ["r16", "l0"], //
        r17: reg = ["r17", "l1"], //
        r18: reg = ["r18", "l2"], //
        r19: reg = ["r19", "l3"], //
        r20: reg = ["r20", "l4"], //
        r21: reg = ["r21", "l5"], //
        r22: reg = ["r22", "l6"], //
        r23: reg = ["r23", "l7"], //
        r24: reg = ["r24", "i0"], //
        r25: reg = ["r25", "i1"], //
        r26: reg = ["r26", "i2"], //
        r27: reg = ["r27", "i3"], //
        r28: reg = ["r28", "i4"], //
        r29: reg = ["r29", "i5"], //
        y: yreg = ["y"],
        // FIXME: %g1 is volatile in ABI, but used internally by LLVM.
        // https://github.com/llvm/llvm-project/blob/llvmorg-19.1.0/llvm/lib/Target/Sparc/SparcRegisterInfo.cpp#L55-L56
        // > FIXME: G1 reserved for now for large imm generation by frame code.
    }
}
