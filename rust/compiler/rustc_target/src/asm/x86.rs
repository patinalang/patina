use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    X86 X86InlineAsmRegClass {
        reg,
        reg_abcd,
        reg_byte,
        xmm_reg,
        ymm_reg,
        zmm_reg,
        kreg,
        kreg0,
        mmx_reg,
        x87_reg,
        tmm_reg,
    }
}

impl X86InlineAsmRegClass {
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
        _allow_experimental_reg: bool,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        &[]
    }
}

def_regs! {
    X86 X86InlineAsmReg X86InlineAsmRegClass {
        ax: reg, reg_abcd = ["ax", "eax", "rax"],
        bx: reg, reg_abcd = ["bx", "ebx", "rbx"],
        cx: reg, reg_abcd = ["cx", "ecx", "rcx"],
        dx: reg, reg_abcd = ["dx", "edx", "rdx"],
        si: reg = ["si", "esi", "rsi"],
        di: reg = ["di", "edi", "rdi"],
        r8: reg = ["r8", "r8w", "r8d"],
        r9: reg = ["r9", "r9w", "r9d"],
        r10: reg = ["r10", "r10w", "r10d"],
        r11: reg = ["r11", "r11w", "r11d"],
        r12: reg = ["r12", "r12w", "r12d"],
        r13: reg = ["r13", "r13w", "r13d"],
        r14: reg = ["r14", "r14w", "r14d"],
        r15: reg = ["r15", "r15w", "r15d"],
        al: reg_byte = ["al"],
        ah: reg_byte = ["ah"],
        bl: reg_byte = ["bl"],
        bh: reg_byte = ["bh"],
        cl: reg_byte = ["cl"],
        ch: reg_byte = ["ch"],
        dl: reg_byte = ["dl"],
        dh: reg_byte = ["dh"],
        sil: reg_byte = ["sil"],
        dil: reg_byte = ["dil"],
        r8b: reg_byte = ["r8b"],
        r9b: reg_byte = ["r9b"],
        r10b: reg_byte = ["r10b"],
        r11b: reg_byte = ["r11b"],
        r12b: reg_byte = ["r12b"],
        r13b: reg_byte = ["r13b"],
        r14b: reg_byte = ["r14b"],
        r15b: reg_byte = ["r15b"],
        xmm0: xmm_reg = ["xmm0"],
        xmm1: xmm_reg = ["xmm1"],
        xmm2: xmm_reg = ["xmm2"],
        xmm3: xmm_reg = ["xmm3"],
        xmm4: xmm_reg = ["xmm4"],
        xmm5: xmm_reg = ["xmm5"],
        xmm6: xmm_reg = ["xmm6"],
        xmm7: xmm_reg = ["xmm7"],
        xmm8: xmm_reg = ["xmm8"],
        xmm9: xmm_reg = ["xmm9"],
        xmm10: xmm_reg = ["xmm10"],
        xmm11: xmm_reg = ["xmm11"],
        xmm12: xmm_reg = ["xmm12"],
        xmm13: xmm_reg = ["xmm13"],
        xmm14: xmm_reg = ["xmm14"],
        xmm15: xmm_reg = ["xmm15"],
        ymm0: ymm_reg = ["ymm0"],
        ymm1: ymm_reg = ["ymm1"],
        ymm2: ymm_reg = ["ymm2"],
        ymm3: ymm_reg = ["ymm3"],
        ymm4: ymm_reg = ["ymm4"],
        ymm5: ymm_reg = ["ymm5"],
        ymm6: ymm_reg = ["ymm6"],
        ymm7: ymm_reg = ["ymm7"],
        ymm8: ymm_reg = ["ymm8"],
        ymm9: ymm_reg = ["ymm9"],
        ymm10: ymm_reg = ["ymm10"],
        ymm11: ymm_reg = ["ymm11"],
        ymm12: ymm_reg = ["ymm12"],
        ymm13: ymm_reg = ["ymm13"],
        ymm14: ymm_reg = ["ymm14"],
        ymm15: ymm_reg = ["ymm15"],
        zmm0: zmm_reg = ["zmm0"],
        zmm1: zmm_reg = ["zmm1"],
        zmm2: zmm_reg = ["zmm2"],
        zmm3: zmm_reg = ["zmm3"],
        zmm4: zmm_reg = ["zmm4"],
        zmm5: zmm_reg = ["zmm5"],
        zmm6: zmm_reg = ["zmm6"],
        zmm7: zmm_reg = ["zmm7"],
        zmm8: zmm_reg = ["zmm8"],
        zmm9: zmm_reg = ["zmm9"],
        zmm10: zmm_reg = ["zmm10"],
        zmm11: zmm_reg = ["zmm11"],
        zmm12: zmm_reg = ["zmm12"],
        zmm13: zmm_reg = ["zmm13"],
        zmm14: zmm_reg = ["zmm14"],
        zmm15: zmm_reg = ["zmm15"],
        zmm16: zmm_reg = ["zmm16", "xmm16", "ymm16"],
        zmm17: zmm_reg = ["zmm17", "xmm17", "ymm17"],
        zmm18: zmm_reg = ["zmm18", "xmm18", "ymm18"],
        zmm19: zmm_reg = ["zmm19", "xmm19", "ymm19"],
        zmm20: zmm_reg = ["zmm20", "xmm20", "ymm20"],
        zmm21: zmm_reg = ["zmm21", "xmm21", "ymm21"],
        zmm22: zmm_reg = ["zmm22", "xmm22", "ymm22"],
        zmm23: zmm_reg = ["zmm23", "xmm23", "ymm23"],
        zmm24: zmm_reg = ["zmm24", "xmm24", "ymm24"],
        zmm25: zmm_reg = ["zmm25", "xmm25", "ymm25"],
        zmm26: zmm_reg = ["zmm26", "xmm26", "ymm26"],
        zmm27: zmm_reg = ["zmm27", "xmm27", "ymm27"],
        zmm28: zmm_reg = ["zmm28", "xmm28", "ymm28"],
        zmm29: zmm_reg = ["zmm29", "xmm29", "ymm29"],
        zmm30: zmm_reg = ["zmm30", "xmm30", "ymm30"],
        zmm31: zmm_reg = ["zmm31", "xmm31", "ymm31"],
        k0: kreg0 = ["k0"],
        k1: kreg = ["k1"],
        k2: kreg = ["k2"],
        k3: kreg = ["k3"],
        k4: kreg = ["k4"],
        k5: kreg = ["k5"],
        k6: kreg = ["k6"],
        k7: kreg = ["k7"],
        mm0: mmx_reg = ["mm0"],
        mm1: mmx_reg = ["mm1"],
        mm2: mmx_reg = ["mm2"],
        mm3: mmx_reg = ["mm3"],
        mm4: mmx_reg = ["mm4"],
        mm5: mmx_reg = ["mm5"],
        mm6: mmx_reg = ["mm6"],
        mm7: mmx_reg = ["mm7"],
        st0: x87_reg = ["st(0)", "st"],
        st1: x87_reg = ["st(1)"],
        st2: x87_reg = ["st(2)"],
        st3: x87_reg = ["st(3)"],
        st4: x87_reg = ["st(4)"],
        st5: x87_reg = ["st(5)"],
        st6: x87_reg = ["st(6)"],
        st7: x87_reg = ["st(7)"],
        tmm0: tmm_reg = ["tmm0"],
        tmm1: tmm_reg = ["tmm1"],
        tmm2: tmm_reg = ["tmm2"],
        tmm3: tmm_reg = ["tmm3"],
        tmm4: tmm_reg = ["tmm4"],
        tmm5: tmm_reg = ["tmm5"],
        tmm6: tmm_reg = ["tmm6"],
        tmm7: tmm_reg = ["tmm7"],
    }
}
