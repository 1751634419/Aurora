use crate::instr::inst::{Instruction, MemoryArguments};
use crate::vm::VirtualMachine;
use std::cmp::min;
use std::convert::TryInto;

pub struct I32ConstInst {
    pub val: i32
}

impl Instruction for I32ConstInst {
    fn Execute(&self, vm: &VirtualMachine) {
        vm.operand_stack.push_i32(self.val);
    }
}

pub struct I64ConstInst {
    pub val: i64
}

impl Instruction for I64ConstInst {
    fn Execute(&self, vm: &VirtualMachine) {
        vm.operand_stack.push_i64(self.val);
    }
}

pub struct F32ConstInst {
    pub val: f32
}

impl Instruction for F32ConstInst {
    fn Execute(&self, vm: &VirtualMachine) {
        vm.operand_stack.push_f32(self.val);
    }
}

pub struct F64ConstInst {
    pub val: f64
}

impl Instruction for F64ConstInst {
    fn Execute(&self, vm: &VirtualMachine) {
        vm.operand_stack.push_f64(self.val);
    }
}

pub struct I32EqzInst {

}

impl Instruction for I32EqzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let stack = &vm.operand_stack;
        let val = stack.pop_u32().unwrap();
        stack.push_bool(val == 0);
    }
}

pub struct I32EqInst {

}

impl Instruction for I32EqInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_bool(a == b);
    }
}

pub struct I32NeInst {

}

impl Instruction for I32NeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_bool(a != b);
    }
}

pub struct I32LtsInst {

}

impl Instruction for I32LtsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i32().unwrap();
        let a = stack.pop_i32().unwrap();
        stack.push_bool(a < b);
    }
}

pub struct I32LtuInst {

}

impl Instruction for I32LtuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_bool(a < b);
    }
}

pub struct I32GtsInst {

}

impl Instruction for I32GtsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i32().unwrap();
        let a = stack.pop_i32().unwrap();
        stack.push_bool(a > b);
    }
}

pub struct I32GtuInst {

}

impl Instruction for I32GtuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_bool(a > b);
    }
}

pub struct I32LesInst {

}

impl Instruction for I32LesInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i32().unwrap();
        let a = stack.pop_i32().unwrap();
        stack.push_bool(a <= b);
    }
}

pub struct I32LeuInst {

}

impl Instruction for I32LeuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_bool(a <= b);
    }
}

pub struct I32GesInst {

}

impl Instruction for I32GesInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i32().unwrap();
        let a = stack.pop_i32().unwrap();
        stack.push_bool(a >= b);
    }
}

pub struct I32GeuInst {

}

impl Instruction for I32GeuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_bool(a >= b);
    }
}

pub struct I64EqzInst {

}

impl Instruction for I64EqzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u64().unwrap() == 0;
        stack.push_bool(val);
    }
}

pub struct I64EqInst {

}

impl Instruction for I64EqInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_bool(a == b);
    }
}

pub struct I64NeInst {

}

impl Instruction for I64NeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_bool(a != b);
    }
}

pub struct I64LtsInst {

}

impl Instruction for I64LtsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i64().unwrap();
        let a = stack.pop_i64().unwrap();
        stack.push_bool(a < b);
    }
}

pub struct I64LtuInst {

}

impl Instruction for I64LtuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_bool(a < b);
    }
}

pub struct I64GtsInst {

}

impl Instruction for I64GtsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i64().unwrap();
        let a = stack.pop_i64().unwrap();
        stack.push_bool(a > b);
    }
}

pub struct I64GtuInst {

}

impl Instruction for I64GtuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_bool(a > b);
    }
}

pub struct I64LesInst {

}

impl Instruction for I64LesInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i64().unwrap();
        let a = stack.pop_i64().unwrap();
        stack.push_bool(a <= b);
    }
}

pub struct I64LeuInst {

}

impl Instruction for I64LeuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_bool(a <= b);
    }
}

pub struct I64GesInst {

}

impl Instruction for I64GesInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i64().unwrap();
        let a = stack.pop_i64().unwrap();
        stack.push_bool(a >= b);
    }
}

pub struct I64GeuInst {

}

impl Instruction for I64GeuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_bool(a >= b);
    }
}

pub struct F32EqInst {

}

impl Instruction for F32EqInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_bool(a == b);
    }
}

pub struct F32NeInst {

}

impl Instruction for F32NeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_bool(a != b);
    }
}

pub struct F32LtInst {

}

impl Instruction for F32LtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_bool(a < b);
    }
}

pub struct F32GtInst {

}

impl Instruction for F32GtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_bool(a > b);
    }
}

pub struct F32LeInst {

}

impl Instruction for F32LeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_bool(a <= b);
    }
}

pub struct F32GeInst {

}

impl Instruction for F32GeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_bool(a >= b);
    }
}

pub struct F64EqInst {

}

impl Instruction for F64EqInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_bool(a == b);
    }
}

pub struct F64NeInst {

}

impl Instruction for F64NeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_bool(a != b);
    }
}

pub struct F64LtInst {

}

impl Instruction for F64LtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_bool(a < b);
    }
}

pub struct F64GtInst {

}

impl Instruction for F64GtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_bool(a > b);
    }
}

pub struct F64LeInst {

}

impl Instruction for F64LeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_bool(a <= b);
    }
}

pub struct F64GeInst {

}

impl Instruction for F64GeInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_bool(a >= b);
    }
}

pub struct I32ClzInst {

}

impl Instruction for I32ClzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u32().unwrap();
        stack.push_u32(val.leading_zeros());
    }
}

pub struct I32CtzInst {

}

impl Instruction for I32CtzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u32().unwrap();
        stack.push_u32(val.trailing_zeros());
    }
}

pub struct I32PopCntInst {

}

impl Instruction for I32PopCntInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u32().unwrap();
        stack.push_u32(val.count_ones());
    }
}

pub struct I32AddInst {

}

impl Instruction for I32AddInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a + b);
    }
}

pub struct I32SubInst {

}

impl Instruction for I32SubInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a - b);
    }
}

pub struct I32MulInst {

}

impl Instruction for I32MulInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a * b);
    }
}

pub struct I32DivsInst {

}

impl Instruction for I32DivsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i32().unwrap();
        let a = stack.pop_i32().unwrap();
        stack.push_i32(a / b);
    }
}

pub struct I32DivuInst {

}

impl Instruction for I32DivuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a / b);
    }
}

pub struct I32RemsInst {

}

impl Instruction for I32RemsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i32().unwrap();
        let a = stack.pop_i32().unwrap();
        stack.push_i32(a % b);
    }
}

pub struct I32RemuInst {

}

impl Instruction for I32RemuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a % b);
    }
}

pub struct I32AndInst {

}

impl Instruction for I32AndInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a & b);
    }
}

pub struct I32OrInst {

}

impl Instruction for I32OrInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a | b);
    }
}

pub struct I32XorInst {

}

impl Instruction for I32XorInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a ^ b);
    }
}

pub struct I32ShlInst {

}

impl Instruction for I32ShlInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a << (b % 32));
    }
}

pub struct I32ShrsInst {

}

impl Instruction for I32ShrsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i32().unwrap();
        let a = stack.pop_i32().unwrap();
        stack.push_i32(a >> (b % 32));
    }
}

pub struct I32ShruInst {

}

impl Instruction for I32ShruInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a >> (b % 32));
    }
}

pub struct I32RotlInst {

}

impl Instruction for I32RotlInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a.rotate_left(b));
    }
}

pub struct I32RotrInst {

}

impl Instruction for I32RotrInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u32().unwrap();
        let a = stack.pop_u32().unwrap();
        stack.push_u32(a.rotate_right(b));
    }
}

pub struct I64ClzInst {

}

impl Instruction for I64ClzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u64().unwrap();
        stack.push_u64(val.leading_zeros() as u64);
    }
}

pub struct I64CtzInst {

}

impl Instruction for I64CtzInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u64().unwrap();
        stack.push_u64(val.trailing_zeros() as u64);
    }
}

pub struct I64PopCntInst {

}

impl Instruction for I64PopCntInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u64().unwrap();
        stack.push_u64(val.count_ones() as u64);
    }
}

pub struct I64AddInst {

}

impl Instruction for I64AddInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a + b);
    }
}

pub struct I64SubInst {

}

impl Instruction for I64SubInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a - b);
    }
}

pub struct I64MulInst {

}

impl Instruction for I64MulInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a * b);
    }
}

pub struct I64DivsInst {

}

impl Instruction for I64DivsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i64().unwrap();
        let a = stack.pop_i64().unwrap();
        stack.push_i64(a / b);
    }
}

pub struct I64DivuInst {

}

impl Instruction for I64DivuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a / b);
    }
}

pub struct I64RemsInst {

}

impl Instruction for I64RemsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i64().unwrap();
        let a = stack.pop_i64().unwrap();
        stack.push_i64(a % b);
    }
}

pub struct I64RemuInst {

}

impl Instruction for I64RemuInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a % b);
    }
}

pub struct I64AndInst {

}

impl Instruction for I64AndInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a & b);
    }
}

pub struct I64OrInst {

}

impl Instruction for I64OrInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a | b);
    }
}

pub struct I64XorInst {

}

impl Instruction for I64XorInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a ^ b);
    }
}

pub struct I64ShlInst {

}

impl Instruction for I64ShlInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a << (b % 64));
    }
}

pub struct I64ShrsInst {

}

impl Instruction for I64ShrsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_i64().unwrap();
        let a = stack.pop_i64().unwrap();
        stack.push_i64(a >> (b % 64));
    }
}

pub struct I64ShruInst {

}

impl Instruction for I64ShruInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a >> (b % 64));
    }
}

pub struct I64RotlInst {

}

impl Instruction for I64RotlInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a.rotate_left(b as u32));
    }
}

pub struct I64RotrInst {

}

impl Instruction for I64RotrInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_u64().unwrap();
        let a = stack.pop_u64().unwrap();
        stack.push_u64(a.rotate_right(b as u32));

    }
}

pub struct F32AbsInst {

}

impl Instruction for F32AbsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap();
        stack.push_f32(val.abs());
    }
}

pub struct F32NegInst {

}

impl Instruction for F32NegInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap();
        stack.push_f32(-val);
    }
}

pub struct F32CeilInst {

}

impl Instruction for F32CeilInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap();
        stack.push_f32(val.ceil());
    }
}

pub struct F32FloorInst {

}

impl Instruction for F32FloorInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap();
        stack.push_f32(val.floor());
    }
}

pub struct F32TruncInst {

}

impl Instruction for F32TruncInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap();
        stack.push_f32(val.trunc());
    }
}

pub struct F32NearestInst {

}

impl Instruction for F32NearestInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap();
        stack.push_f32(val.round());
    }
}

pub struct F32SqrtInst {

}

impl Instruction for F32SqrtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap();
        stack.push_f32(val.sqrt());
    }
}

pub struct F32AddInst {

}

impl Instruction for F32AddInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_f32(a + b);
    }
}

pub struct F32SubInst {

}

impl Instruction for F32SubInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_f32(a - b);
    }
}

pub struct F32MulInst {

}

impl Instruction for F32MulInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_f32(a * b);
    }
}

pub struct F32DivInst {

}

impl Instruction for F32DivInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_f32(a / b);
    }
}

pub struct F32MinInst {

}

impl Instruction for F32MinInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_f32(a.min(b));
    }
}

pub struct F32MaxInst {

}

impl Instruction for F32MaxInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_f32(a.max(b));
    }
}

pub struct F32CopySignInst {

}

impl Instruction for F32CopySignInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f32().unwrap();
        let a = stack.pop_f32().unwrap();
        stack.push_f32(a.copysign(b));
    }
}

pub struct F64AbsInst {

}

impl Instruction for F64AbsInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap();
        stack.push_f64(val.abs());
    }
}

pub struct F64NegInst {

}

impl Instruction for F64NegInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap();
        stack.push_f64(-val);
    }
}

pub struct F64CeilInst {

}

impl Instruction for F64CeilInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap();
        stack.push_f64(val.ceil());
    }
}

pub struct F64FloorInst {

}

impl Instruction for F64FloorInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap();
        stack.push_f64(val.floor());
    }
}

pub struct F64TruncInst {

}

impl Instruction for F64TruncInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap();
        stack.push_f64(val.trunc());
    }
}

pub struct F64NearestInst {

}

impl Instruction for F64NearestInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap();
        stack.push_f64(val.round());
    }
}

pub struct F64SqrtInst {

}

impl Instruction for F64SqrtInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap();
        stack.push_f64(val.sqrt());
    }
}

pub struct F64AddInst {

}

impl Instruction for F64AddInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_f64(a + b);
    }
}

pub struct F64SubInst {

}

impl Instruction for F64SubInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_f64(a - b);
    }
}

pub struct F64MulInst {

}

impl Instruction for F64MulInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_f64(a * b);
    }
}

pub struct F64DivInst {

}

impl Instruction for F64DivInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_f64(a / b);
    }
}

pub struct F64MinInst {

}

impl Instruction for F64MinInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_f64(a.min(b));
    }
}

pub struct F64MaxInst {

}

impl Instruction for F64MaxInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_f64(a.max(b));
    }
}

pub struct F64CopySignInst {

}

impl Instruction for F64CopySignInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let b = stack.pop_f64().unwrap();
        let a = stack.pop_f64().unwrap();
        stack.push_f64(a.copysign(b));
    }
}

pub struct I32WrapI64Inst {

}

impl Instruction for I32WrapI64Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u64().unwrap();
        stack.push_u32(val as u32);
    }
}

pub struct I32TruncF32SInst {

}

impl Instruction for I32TruncF32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = (stack.pop_f32().unwrap() as f64).trunc();
        stack.push_i32(val as i32);
    }
}

pub struct I32TruncF32UInst {

}

impl Instruction for I32TruncF32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = (stack.pop_f32().unwrap() as f64).trunc();
        stack.push_u32(val as u32);
    }
}

pub struct I32TruncF64SInst {

}

impl Instruction for I32TruncF64SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap().trunc();
        stack.push_i32(val as i32);
    }
}

pub struct I32TruncF64UInst {

}

impl Instruction for I32TruncF64UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap().trunc();
        stack.push_u32(val as u32);
    }
}

pub struct I64ExtendI32SInst {

}

impl Instruction for I64ExtendI32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i32().unwrap();
        stack.push_i64(val as i64);
    }
}

pub struct I64ExtendI32UInst {

}

impl Instruction for I64ExtendI32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u32().unwrap();
        stack.push_u64(val as u64);
    }
}

pub struct I64TruncF32SInst {

}

impl Instruction for I64TruncF32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap().trunc();
        stack.push_i64(val as i64);
    }
}

pub struct I64TruncF32UInst {

}

impl Instruction for I64TruncF32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap().trunc();
        stack.push_u64(val as u64);
    }
}

pub struct I64TruncF64SInst {

}

impl Instruction for I64TruncF64SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap().trunc();
        stack.push_i64(val as i64);
    }
}

pub struct I64TruncF64UInst {

}

impl Instruction for I64TruncF64UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap().trunc();
        stack.push_u64(val as u64);
    }
}

pub struct F32ConvertI32SInst {

}

impl Instruction for F32ConvertI32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i32().unwrap();
        stack.push_f32(val as f32);
    }
}

pub struct F32ConvertI32UInst {

}

impl Instruction for F32ConvertI32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u32().unwrap();
        stack.push_f32(val as f32);
    }
}

pub struct F32ConvertI64SInst {

}

impl Instruction for F32ConvertI64SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i64().unwrap();
        stack.push_f32(val as f32);
    }
}

pub struct F32ConvertI64UInst {

}

impl Instruction for F32ConvertI64UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u64().unwrap();
        stack.push_f32(val as f32);
    }
}

pub struct F32DemoteF64Inst {

}

impl Instruction for F32DemoteF64Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f64().unwrap();
        stack.push_f32(val as f32);
    }
}

pub struct F64ConvertI32SInst {

}

impl Instruction for F64ConvertI32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i32().unwrap();
        stack.push_f64(val as f64);
    }
}

pub struct F64ConvertI32UInst {

}

impl Instruction for F64ConvertI32UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u32().unwrap();
        stack.push_f64(val as f64);
    }
}

pub struct F64ConvertI64SInst {

}

impl Instruction for F64ConvertI64SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i64().unwrap();
        stack.push_f64(val as f64);
    }
}

pub struct F64ConvertI64UInst {

}

impl Instruction for F64ConvertI64UInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u64().unwrap();
        stack.push_f64(val as f64);
    }
}

pub struct F64PromoteF32Inst {

}

impl Instruction for F64PromoteF32Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap();
        stack.push_f64(val as f64);
    }
}

pub struct I32ReinterpretF32Inst {

}

impl Instruction for I32ReinterpretF32Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_f32().unwrap();
        stack.push_u32(u32::from_be_bytes(val.to_be_bytes()));
    }
}

pub struct I64ReinterpretF64Inst {

}

impl Instruction for I64ReinterpretF64Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        // nothing to do
    }
}

pub struct F32ReinterpretI32Inst {

}

impl Instruction for F32ReinterpretI32Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_u32().unwrap();
        stack.push_f32(f32::from_be_bytes(val.to_be_bytes()));
    }
}

pub struct F64ReinterpretI64Inst {

}

impl Instruction for F64ReinterpretI64Inst {
    fn Execute(&self, vm: &VirtualMachine) {
        // nothing to do
    }
}

pub struct I32Extend8SInst {

}

impl Instruction for I32Extend8SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i32().unwrap();
        stack.push_i32((val as i8) as i32);
    }
}

pub struct I32Extend16SInst {

}

impl Instruction for I32Extend16SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i32().unwrap();
        stack.push_i32((val as i16) as i32);
    }
}

pub struct I64Extend8SInst {

}

impl Instruction for I64Extend8SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i64().unwrap();
        stack.push_i64((val as i8) as i64);
    }
}

pub struct I64Extend16SInst {

}

impl Instruction for I64Extend16SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i64().unwrap();
        stack.push_i64((val as i16) as i64);
    }
}

pub struct I64Extend32SInst {

}

impl Instruction for I64Extend32SInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let mut stack = &vm.operand_stack;
        let val = stack.pop_i64().unwrap();
        stack.push_i64((val as i32) as i64);
    }
}

pub struct TruncSatInst {
    pub operand: u8
}

impl Instruction for TruncSatInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let stack = &vm.operand_stack;

        match self.operand {
            0 => {
                let v = trunc_sat_s(stack.pop_f32().unwrap() as f64, 32);
                stack.push_i32(v as i32);
            }
            1 => {
                let v = trunc_sat_u(stack.pop_f32().unwrap() as f64, 32);
                stack.push_u32(v as u32);
            }
            2 => {
                let v = trunc_sat_s(stack.pop_f64().unwrap(), 32);
                stack.push_i32(v as i32);
            }
            3 => {
                let v = trunc_sat_u(stack.pop_f64().unwrap(), 32);
                stack.push_u32(v as u32);
            }
            4 => {
                let v = trunc_sat_s(stack.pop_f32().unwrap() as f64, 64);
                stack.push_i64(v);
            }
            5 => {
                let v = trunc_sat_u(stack.pop_f32().unwrap() as f64, 64);
                stack.push_u64(v);
            }
            6 => {
                let v = trunc_sat_s(stack.pop_f64().unwrap(), 64);
                stack.push_i64(v);
            }
            7 => {
                let v = trunc_sat_u(stack.pop_f64().unwrap(), 64);
                stack.push_u64(v);
            }
            _ => {
                panic!("malformed operand code");
            }
        }
    }

}

fn trunc_sat_u(z: f64, n: i8) -> u64 {
    if z.is_nan() {
        return 0;
    }

    if z.is_infinite() && z.is_sign_negative() {
        return 0;
    }

    let max = ((1 as u64) << n) - 1;
    if z.is_infinite() && z.is_sign_positive() {
        return max;
    }
    let x = z.trunc();
    if x < 0.0 {
        return 0;
    } else if x >= max as f64 {
        return max;
    } else {
        return x as u64;
    }
}

fn trunc_sat_s(z: f64, n: i8) -> i64 {
    if z.is_nan() {
        return 0;
    }

    let min = (-1_i64 << (n - 1));
    let max = (1_i64 << (n - 1)) - 1;
    if z.is_infinite() {
        if z.is_sign_negative() {
            return min;
        } else {
            return max;
        }
    }
    let x = z.trunc();
    if x < min as f64 {
        return min;
    } else if x >= max as f64 {
        return max;
    } else {
        return x as i64;
    }
}