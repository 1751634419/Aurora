use crate::instr::inst::{Instruction, BlockArguments, IfArguments, BrTableArguments};
use crate::vm::VirtualMachine;
use crate::module::{BlockType, LabelIndex, FunctionIndex, TypeIndex};

pub struct UnreachableInst {}

impl Instruction for UnreachableInst {
    fn Execute(&self, vm: &VirtualMachine) {
        panic!("Unreachable");
    }
}

pub struct NopInst {}

impl Instruction for NopInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // nothing to do
    }
}

pub struct BlockInst {
    pub bt: BlockArguments
}

impl Instruction for BlockInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct LoopInst {
    pub bt: BlockArguments
}

impl Instruction for LoopInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct IfInst {
    pub if_arguments: IfArguments
}

impl Instruction for IfInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct BrInst {
    pub lable_index: LabelIndex
}

impl Instruction for BrInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct BrIfInst {
    pub lable_index: LabelIndex
}

impl Instruction for BrIfInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct BrTableInst {
    pub br_table_arguments: BrTableArguments
}

impl Instruction for BrTableInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct ReturnInst {

}

impl Instruction for ReturnInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct CallInst {
    pub func_index: FunctionIndex
}

impl Instruction for CallInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct CallIndirectInst {
    pub type_idx: TypeIndex
}

impl Instruction for CallIndirectInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}