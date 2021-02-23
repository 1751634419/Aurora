use crate::instr::inst::{Instruction, BlockArguments, IfArguments, BrTableArguments};
use crate::vm::VirtualMachine;
use crate::module::{BlockType, LabelIndex, FunctionIndex, TypeIndex};

pub struct UnreachableInst {}

impl Instruction for UnreachableInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        panic!("Unreachable");
    }
}

pub struct NopInst {}

impl Instruction for NopInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // nothing to do
    }
}

pub struct BlockInst {
    pub bt: BlockArguments
}

impl Instruction for BlockInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct LoopInst {
    pub bt: BlockArguments
}

impl Instruction for LoopInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct IfInst {
    pub if_arguments: IfArguments
}

impl Instruction for IfInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct BrInst {
    pub lable_index: LabelIndex
}

impl Instruction for BrInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct BrIfInst {
    pub lable_index: LabelIndex
}

impl Instruction for BrIfInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct BrTableInst {
    pub br_table_arguments: BrTableArguments
}

impl Instruction for BrTableInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct ReturnInst {

}

impl Instruction for ReturnInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct CallInst {
    pub func_index: FunctionIndex
}

impl Instruction for CallInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
        // hack
        let mut stack = &mut vm.operand_stack;
        let imp_sec = &vm.module.import_section[self.func_index as usize];
        match imp_sec.name.as_str() {
            "assert_true" => {
                assert_eq!(stack.pop_bool().unwrap(), true);
            }
            "assert_false" => {
                assert_eq!(stack.pop_bool().unwrap(), false);
            }
            "assert_eq_i32" => {
                let v1 = stack.pop_u32().unwrap();
                let v2 = stack.pop_u32().unwrap();
                assert_eq!(v1, v2);
            }
            "assert_eq_i64" => {
                let v1 = stack.pop_u64().unwrap();
                let v2 = stack.pop_u64().unwrap();
                assert_eq!(v1, v2);
            }
            "assert_eq_f32" => {
                let v1 = stack.pop_f32().unwrap();
                let v2 = stack.pop_f32().unwrap();
                assert_eq!(v1, v2);
            }
            "assert_eq_f64" => {
                let v1 = stack.pop_f64().unwrap();
                let v2 = stack.pop_f64().unwrap();
                assert_eq!(v1, v2);
            }

            _ => {
                panic!("wrong");
            }
        }
    }
}

pub struct CallIndirectInst {
    pub type_idx: TypeIndex
}

impl Instruction for CallIndirectInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}