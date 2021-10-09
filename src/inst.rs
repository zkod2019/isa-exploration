#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    Rsp,
    Rfp,
    Rip,
}

impl Register {
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => Self::R0,
            1 => Self::R1,
            2 => Self::R2,
            3 => Self::R3,
            4 => Self::R4,
            5 => Self::R5,
            6 => Self::R6,
            7 => Self::R7,
            8 => Self::R8,
            9 => Self::R9,
            10 => Self::R10,
            11 => Self::R11,
            12 => Self::R12,
            13 => Self::R13,
            14 => Self::R14,
            15 => Self::R15,
            16 => Self::Rsp,
            17 => Self::Rfp,
            18 => Self::Rip,
            _ => panic!("expected register ID between 0 and 18"),
        }
    }

    pub fn get_id(&self) -> u8 {
        match self {
            Self::R0 => 0,
            Self::R1 => 1,
            Self::R2 => 2,
            Self::R3 => 3,
            Self::R4 => 4,
            Self::R5 => 5,
            Self::R6 => 6,
            Self::R7 => 7,
            Self::R8 => 8,
            Self::R9 => 9,
            Self::R10 => 10,
            Self::R11 => 11,
            Self::R12 => 12,
            Self::R13 => 13,
            Self::R14 => 14,
            Self::R15 => 15,
            Self::Rsp => 16,
            Self::Rfp => 17,
            Self::Rip => 18,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Label(pub String);

impl Label {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Imm {
    Int(u64),
    Float(f64),
    True,
    False,
}

impl Imm {
    pub fn as_u64(&self) -> u64 {
        match self {
            Self::Int(int_value) => *int_value as u64,
            Self::Float(float_value) => float_value.to_bits(),
            Self::True => u64::MAX,
            Self::False => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub label: String,
    pub insts: Vec<Inst>,
}

impl Block {
    pub fn as_asm(&self) -> String {
        let mut result = self.label.clone() + ":\n";
        for inst in &self.insts {
            result += &format!("  {}\n", &inst.as_asm());
        }
        result
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Inst {
    // Useful for debugging purposes prints the registers contents
    Dbg(Register),
    PrintInt(Register),
    PrintUInt(Register),
    PrintFloat(Register),

    // Memory & registers
    Rega(Register, Imm),
    Copy(Register, Register),
    Load(Register, Register),
    Store(Register, Register),

    // Control flow
    Jump(Label),
    CJump(Register, Label),
    Branch(Register, Label, Label),

    // Binary operations
    Shl(Register, Register, Register),
    Shr(Register, Register, Register),
    And(Register, Register, Register),
    Or(Register, Register, Register),
    Xor(Register, Register, Register),
    Not(Register, Register),

    // Arithmetic operations
    SAdd(Register, Register, Register),
    UAdd(Register, Register, Register),
    FAdd(Register, Register, Register),

    Sub(Register, Register, Register),
    FSub(Register, Register, Register),

    SMul(Register, Register, Register),
    UMul(Register, Register, Register),
    FMul(Register, Register, Register),

    SDiv(Register, Register, Register),
    UDiv(Register, Register, Register),
    FDiv(Register, Register, Register),

    SRem(Register, Register, Register),
    URem(Register, Register, Register),
    FRem(Register, Register, Register),

    // Comparative operators
    Eq(Register, Register, Register),
    FEq(Register, Register, Register),

    SLt(Register, Register, Register),
    ULt(Register, Register, Register),
    FLt(Register, Register, Register),

    SGt(Register, Register, Register),
    UGt(Register, Register, Register),
    FGt(Register, Register, Register),
}

impl Inst {
    pub fn as_asm(&self) -> String {
        match self {
            Self::Dbg(reg) => format!("dbg %{}", reg.get_id()),
            Self::PrintInt(reg) => format!("print_int %{}", reg.get_id()),
            Self::PrintUInt(reg) => format!("print_uint %{}", reg.get_id()),
            Self::PrintFloat(reg) => format!("print_float %{}", reg.get_id()),

            Self::Rega(dst, imm) => format!(
                "rega %{} {}",
                dst.get_id(),
                match imm {
                    Imm::Int(i) => i.to_string(),
                    Imm::Float(f) => f.to_string(),
                    Imm::True => "true".to_string(),
                    Imm::False => "false".to_string(),
                }
            ),
            Self::Copy(dst, src) => format!("copy %{} %{}", dst.get_id(), src.get_id()),
            Self::Load(dst, adr) => format!("load %{} %{}", dst.get_id(), adr.get_id()),
            Self::Store(adr, src) => format!("store %{} %{}", adr.get_id(), src.get_id()),

            Self::Jump(label) => format!("jump @{}", &label.0),
            Self::CJump(cond_reg, true_label) => {
                format!("cjump %{} @{}", cond_reg.get_id(), &true_label.0)
            }
            Self::Branch(cond_reg, true_label, false_label) => format!(
                "branch %{} @{} @{}",
                cond_reg.get_id(),
                &true_label.0,
                &false_label.0
            ),

            Self::Shl(dst, lhs, rhs) => {
                format!("shl %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::Shr(dst, lhs, rhs) => {
                format!("shr %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::And(dst, lhs, rhs) => {
                format!("and %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::Or(dst, lhs, rhs) => {
                format!("or %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::Xor(dst, lhs, rhs) => {
                format!("xor %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::Not(dst, reg) => format!("not %{} %{}", dst.get_id(), reg.get_id()),

            Self::SAdd(dst, lhs, rhs) => {
                format!("sadd %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::UAdd(dst, lhs, rhs) => {
                format!("uadd %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::FAdd(dst, lhs, rhs) => {
                format!("fadd %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }

            Self::Sub(dst, lhs, rhs) => {
                format!("sub %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::FSub(dst, lhs, rhs) => {
                format!("fsub %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }

            Self::SMul(dst, lhs, rhs) => {
                format!("smul %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::UMul(dst, lhs, rhs) => {
                format!("umul %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::FMul(dst, lhs, rhs) => {
                format!("fmul %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }

            Self::SDiv(dst, lhs, rhs) => {
                format!("sdiv %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::UDiv(dst, lhs, rhs) => {
                format!("udiv %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::FDiv(dst, lhs, rhs) => {
                format!("fdiv %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }

            Self::SRem(dst, lhs, rhs) => {
                format!("srem %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::URem(dst, lhs, rhs) => {
                format!("urem %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::FRem(dst, lhs, rhs) => {
                format!("frem %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }

            Self::Eq(dst, lhs, rhs) => {
                format!("eq %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::FEq(dst, lhs, rhs) => {
                format!("feq %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::SLt(dst, lhs, rhs) => {
                format!("slt %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::ULt(dst, lhs, rhs) => {
                format!("ult %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::FLt(dst, lhs, rhs) => {
                format!("flt %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::SGt(dst, lhs, rhs) => {
                format!("sgt %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::UGt(dst, lhs, rhs) => {
                format!("ugt %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
            Self::FGt(dst, lhs, rhs) => {
                format!("fgt %{} %{} %{}", dst.get_id(), lhs.get_id(), rhs.get_id())
            }
        }
    }
}
