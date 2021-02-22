use crate::instr::inst::{Instruction, MemoryArguments};
use crate::vm::VirtualMachine;

pub struct I32ConstInst {
    pub val: i32
}

impl Instruction for I32ConstInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64ConstInst {
    pub val: i64
}

impl Instruction for I64ConstInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32ConstInst {
    pub val: f32
}

impl Instruction for F32ConstInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64ConstInst {
    pub val: f64
}

impl Instruction for F64ConstInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32EqzInst {

}

impl Instruction for I32EqzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32EqInst {

}

impl Instruction for I32EqInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32NeInst {

}

impl Instruction for I32NeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32LtsInst {

}

impl Instruction for I32LtsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32LtuInst {

}

impl Instruction for I32LtuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32GtsInst {

}

impl Instruction for I32GtsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32GtuInst {

}

impl Instruction for I32GtuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32LesInst {

}

impl Instruction for I32LesInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32LeuInst {

}

impl Instruction for I32LeuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32GesInst {

}

impl Instruction for I32GesInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32GeuInst {

}

impl Instruction for I32GeuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64EqzInst {

}

impl Instruction for I64EqzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64EqInst {

}

impl Instruction for I64EqInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64NeInst {

}

impl Instruction for I64NeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64LtsInst {

}

impl Instruction for I64LtsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64LtuInst {

}

impl Instruction for I64LtuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64GtsInst {

}

impl Instruction for I64GtsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64GtuInst {

}

impl Instruction for I64GtuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64LesInst {

}

impl Instruction for I64LesInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64LeuInst {

}

impl Instruction for I64LeuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64GesInst {

}

impl Instruction for I64GesInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64GeuInst {

}

impl Instruction for I64GeuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32EqInst {

}

impl Instruction for F32EqInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32NeInst {

}

impl Instruction for F32NeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32LtInst {

}

impl Instruction for F32LtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32GtInst {

}

impl Instruction for F32GtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32LeInst {

}

impl Instruction for F32LeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32GeInst {

}

impl Instruction for F32GeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64EqInst {

}

impl Instruction for F64EqInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64NeInst {

}

impl Instruction for F64NeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64LtInst {

}

impl Instruction for F64LtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64GtInst {

}

impl Instruction for F64GtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64LeInst {

}

impl Instruction for F64LeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64GeInst {

}

impl Instruction for F64GeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32ClzInst {

}

impl Instruction for I32ClzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32CtzInst {

}

impl Instruction for I32CtzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32PopcntInst {

}

impl Instruction for I32PopcntInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32AddInst {

}

impl Instruction for I32AddInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32SubInst {

}

impl Instruction for I32SubInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32MulInst {

}

impl Instruction for I32MulInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32DivsInst {

}

impl Instruction for I32DivsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32DivuInst {

}

impl Instruction for I32DivuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32RemsInst {

}

impl Instruction for I32RemsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32RemuInst {

}

impl Instruction for I32RemuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32AndInst {

}

impl Instruction for I32AndInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32OrInst {

}

impl Instruction for I32OrInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32XorInst {

}

impl Instruction for I32XorInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32ShlInst {

}

impl Instruction for I32ShlInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32ShrsInst {

}

impl Instruction for I32ShrsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32ShruInst {

}

impl Instruction for I32ShruInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32RotlInst {

}

impl Instruction for I32RotlInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32RotrInst {

}

impl Instruction for I32RotrInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64ClzInst {

}

impl Instruction for I64ClzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64CtzInst {

}

impl Instruction for I64CtzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64PopcntInst {

}

impl Instruction for I64PopcntInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64AddInst {

}

impl Instruction for I64AddInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64SubInst {

}

impl Instruction for I64SubInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64MulInst {

}

impl Instruction for I64MulInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64DivsInst {

}

impl Instruction for I64DivsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64DivuInst {

}

impl Instruction for I64DivuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64RemsInst {

}

impl Instruction for I64RemsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64RemuInst {

}

impl Instruction for I64RemuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64AndInst {

}

impl Instruction for I64AndInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64OrInst {

}

impl Instruction for I64OrInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64XorInst {

}

impl Instruction for I64XorInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64ShlInst {

}

impl Instruction for I64ShlInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64ShrsInst {

}

impl Instruction for I64ShrsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64ShruInst {

}

impl Instruction for I64ShruInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64RotlInst {

}

impl Instruction for I64RotlInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64RotrInst {

}

impl Instruction for I64RotrInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32AbsInst {

}

impl Instruction for F32AbsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32NegInst {

}

impl Instruction for F32NegInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32CeilInst {

}

impl Instruction for F32CeilInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32FloorInst {

}

impl Instruction for F32FloorInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32TruncInst {

}

impl Instruction for F32TruncInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32NearestInst {

}

impl Instruction for F32NearestInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32SqrtInst {

}

impl Instruction for F32SqrtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32AddInst {

}

impl Instruction for F32AddInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32SubInst {

}

impl Instruction for F32SubInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32MulInst {

}

impl Instruction for F32MulInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32DivInst {

}

impl Instruction for F32DivInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32MinInst {

}

impl Instruction for F32MinInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32MaxInst {

}

impl Instruction for F32MaxInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32CopySignInst {

}

impl Instruction for F32CopySignInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64AbsInst {

}

impl Instruction for F64AbsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64NegInst {

}

impl Instruction for F64NegInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64CeilInst {

}

impl Instruction for F64CeilInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64FloorInst {

}

impl Instruction for F64FloorInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64TruncInst {

}

impl Instruction for F64TruncInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64NearestInst {

}

impl Instruction for F64NearestInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64SqrtInst {

}

impl Instruction for F64SqrtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64AddInst {

}

impl Instruction for F64AddInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64SubInst {

}

impl Instruction for F64SubInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64MulInst {

}

impl Instruction for F64MulInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64DivInst {

}

impl Instruction for F64DivInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64MinInst {

}

impl Instruction for F64MinInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64MaxInst {

}

impl Instruction for F64MaxInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64CopySignInst {

}

impl Instruction for F64CopySignInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32WrapI64Inst {

}

impl Instruction for I32WrapI64Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32TruncF32SInst {

}

impl Instruction for I32TruncF32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32TruncF32UInst {

}

impl Instruction for I32TruncF32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32TruncF64SInst {

}

impl Instruction for I32TruncF64SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32TruncF64UInst {

}

impl Instruction for I32TruncF64UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64ExtendI32SInst {

}

impl Instruction for I64ExtendI32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64ExtendI32UInst {

}

impl Instruction for I64ExtendI32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64TruncF32SInst {

}

impl Instruction for I64TruncF32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64TruncF32UInst {

}

impl Instruction for I64TruncF32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64TruncF64SInst {

}

impl Instruction for I64TruncF64SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64TruncF64UInst {

}

impl Instruction for I64TruncF64UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32ConvertI32SInst {

}

impl Instruction for F32ConvertI32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32ConvertI32UInst {

}

impl Instruction for F32ConvertI32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32ConvertI64SInst {

}

impl Instruction for F32ConvertI64SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32ConvertI64UInst {

}

impl Instruction for F32ConvertI64UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32DemoteF64Inst {

}

impl Instruction for F32DemoteF64Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64ConvertI32SInst {

}

impl Instruction for F64ConvertI32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64ConvertI32UInst {

}

impl Instruction for F64ConvertI32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64ConvertI64SInst {

}

impl Instruction for F64ConvertI64SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64ConvertI64UInst {

}

impl Instruction for F64ConvertI64UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64PromoteF32Inst {

}

impl Instruction for F64PromoteF32Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32ReinterpretF32Inst {

}

impl Instruction for I32ReinterpretF32Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64ReinterpretF64Inst {

}

impl Instruction for I64ReinterpretF64Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F32ReinterpretI32Inst {

}

impl Instruction for F32ReinterpretI32Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct F64ReinterpretI64Inst {

}

impl Instruction for F64ReinterpretI64Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32Extend8SInst {

}

impl Instruction for I32Extend8SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I32Extend16SInst {

}

impl Instruction for I32Extend16SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64Extend8SInst {

}

impl Instruction for I64Extend8SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64Extend16SInst {

}

impl Instruction for I64Extend16SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct I64Extend32SInst {

}

impl Instruction for I64Extend32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct TruncSatInst {
    pub operand: u8
}

impl Instruction for TruncSatInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}