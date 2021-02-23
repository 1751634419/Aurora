use crate::instr::inst::{Instruction, BlockArguments, IfArguments, BrTableArguments};
use crate::vm::VirtualMachine;
use crate::module::{BlockType, LabelIndex, FunctionIndex, TypeIndex};

pub struct UnreachableInst {}

impl Instruction for UnreachableInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        panic!("Unreachable");
    }
}

pub struct NopInst {}

impl Instruction for NopInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        // nothing to do
    }
}

pub struct BlockInst {
    pub bt: BlockArguments
}

impl Instruction for BlockInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct LoopInst {
    pub bt: BlockArguments
}

impl Instruction for LoopInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct IfInst {
    pub if_arguments: IfArguments
}

impl Instruction for IfInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct BrInst {
    pub lable_index: LabelIndex
}

impl Instruction for BrInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct BrIfInst {
    pub lable_index: LabelIndex
}

impl Instruction for BrIfInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
        // hack
        if vm.operand_stack.pop_bool() {
            vm.exit_block();
        }
    }
}

pub struct BrTableInst {
    pub br_table_arguments: BrTableArguments
}

impl Instruction for BrTableInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct ReturnInst {

}

impl Instruction for ReturnInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct CallInst {
    pub func_index: FunctionIndex
}

impl Instruction for CallInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let index = self.func_index as usize;
        let imported_func_count = vm.module.import_section.len(); // todo

        if index < imported_func_count {
            CallInst::call_assert_func(vm, index);
        } else {
            CallInst::call_internal_func(vm, index - imported_func_count);
        }
    }
}

impl CallInst {
    fn call_internal_func(vm: &mut VirtualMachine, index: usize) {
        let ft_idx = vm.module.func_section[index];
        let ft = vm.module.type_section[ft_idx as usize].clone();
        vm.enter_block(0x10, ft, vm.module.code_section[index].expression.clone());

        // initialize the local variable table
        let localCount = vm.module.code_section[index].locals.len();
        for i in 0..localCount {
            vm.operand_stack.push_u64(0);
        }
    }

    fn call_assert_func(vm: &mut VirtualMachine, func_index: usize) {
        // hack
        let stack = &mut vm.operand_stack;
        let imp_sec = &vm.module.import_section[func_index];
        match imp_sec.name.as_str() {
            "assert_true" => {
                assert_eq!(stack.pop_bool(), true);
            }
            "assert_false" => {
                assert_eq!(stack.pop_bool(), false);
            }
            "assert_eq_i32" => {
                let v1 = stack.pop_u32();
                let v2 = stack.pop_u32();
                assert_eq!(v1, v2);
            }
            "assert_eq_i64" => {
                let v1 = stack.pop_u64();
                let v2 = stack.pop_u64();
                assert_eq!(v1, v2);
            }
            "assert_eq_f32" => {
                let v1 = stack.pop_f32();
                let v2 = stack.pop_f32();
                assert_eq!(v1, v2);
            }
            "assert_eq_f64" => {
                let v1 = stack.pop_f64();
                let v2 = stack.pop_f64();
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
    fn execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}