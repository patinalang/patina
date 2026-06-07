use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    RiscV RiscVInlineAsmRegClass {
        reg,
        freg,
        vreg,
    }
}

impl RiscVInlineAsmRegClass {
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
    RiscV RiscVInlineAsmReg RiscVInlineAsmRegClass {
        x1: reg = ["x1", "ra"],
        x5: reg = ["x5", "t0"],
        x6: reg = ["x6", "t1"],
        x7: reg = ["x7", "t2"],
        x10: reg = ["x10", "a0"],
        x11: reg = ["x11", "a1"],
        x12: reg = ["x12", "a2"],
        x13: reg = ["x13", "a3"],
        x14: reg = ["x14", "a4"],
        x15: reg = ["x15", "a5"],
        x16: reg = ["x16", "a6"],
        x17: reg = ["x17", "a7"],
        x18: reg = ["x18", "s2"],
        x19: reg = ["x19", "s3"],
        x20: reg = ["x20", "s4"],
        x21: reg = ["x21", "s5"],
        x22: reg = ["x22", "s6"],
        x23: reg = ["x23", "s7"],
        x24: reg = ["x24", "s8"],
        x25: reg = ["x25", "s9"],
        x26: reg = ["x26", "s10"],
        x27: reg = ["x27", "s11"],
        x28: reg = ["x28", "t3"],
        x29: reg = ["x29", "t4"],
        x30: reg = ["x30", "t5"],
        x31: reg = ["x31", "t6"],
        f0: freg = ["f0", "ft0"],
        f1: freg = ["f1", "ft1"],
        f2: freg = ["f2", "ft2"],
        f3: freg = ["f3", "ft3"],
        f4: freg = ["f4", "ft4"],
        f5: freg = ["f5", "ft5"],
        f6: freg = ["f6", "ft6"],
        f7: freg = ["f7", "ft7"],
        f8: freg = ["f8", "fs0"],
        f9: freg = ["f9", "fs1"],
        f10: freg = ["f10", "fa0"],
        f11: freg = ["f11", "fa1"],
        f12: freg = ["f12", "fa2"],
        f13: freg = ["f13", "fa3"],
        f14: freg = ["f14", "fa4"],
        f15: freg = ["f15", "fa5"],
        f16: freg = ["f16", "fa6"],
        f17: freg = ["f17", "fa7"],
        f18: freg = ["f18", "fs2"],
        f19: freg = ["f19", "fs3"],
        f20: freg = ["f20", "fs4"],
        f21: freg = ["f21", "fs5"],
        f22: freg = ["f22", "fs6"],
        f23: freg = ["f23", "fs7"],
        f24: freg = ["f24", "fs8"],
        f25: freg = ["f25", "fs9"],
        f26: freg = ["f26", "fs10"],
        f27: freg = ["f27", "fs11"],
        f28: freg = ["f28", "ft8"],
        f29: freg = ["f29", "ft9"],
        f30: freg = ["f30", "ft10"],
        f31: freg = ["f31", "ft11"],
        v0: vreg = ["v0"],
        v1: vreg = ["v1"],
        v2: vreg = ["v2"],
        v3: vreg = ["v3"],
        v4: vreg = ["v4"],
        v5: vreg = ["v5"],
        v6: vreg = ["v6"],
        v7: vreg = ["v7"],
        v8: vreg = ["v8"],
        v9: vreg = ["v9"],
        v10: vreg = ["v10"],
        v11: vreg = ["v11"],
        v12: vreg = ["v12"],
        v13: vreg = ["v13"],
        v14: vreg = ["v14"],
        v15: vreg = ["v15"],
        v16: vreg = ["v16"],
        v17: vreg = ["v17"],
        v18: vreg = ["v18"],
        v19: vreg = ["v19"],
        v20: vreg = ["v20"],
        v21: vreg = ["v21"],
        v22: vreg = ["v22"],
        v23: vreg = ["v23"],
        v24: vreg = ["v24"],
        v25: vreg = ["v25"],
        v26: vreg = ["v26"],
        v27: vreg = ["v27"],
        v28: vreg = ["v28"],
        v29: vreg = ["v29"],
        v30: vreg = ["v30"],
        v31: vreg = ["v31"],
    }
}
