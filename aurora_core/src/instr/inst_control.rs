use crate::instr::inst::{Instruction, BlockArguments, IfArguments, BrTableArguments};
use crate::vm::VirtualMachine;
use crate::module::{LabelIndex, FunctionIndex, TypeIndex, BlockType, FunctionType, VariableType};
use std::rc::Rc;

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

const TYPE_I32: u8 = 0x7F;
const TYPE_I64: u8 = 0x7E;
const TYPE_F32: u8 = 0x7D;
const TYPE_F64: u8 = 0x7C;
const BLOCK_TYPE_I32: i32 = -1;
const BLOCK_TYPE_I64: i32 = -2;
const BLOCK_TYPE_F32: i32 = -3;
const BLOCK_TYPE_F64: i32 = -4;
const BLOCK_TYPE_EMPTY: i32 = -64;
fn make_func_type(bt: BlockType, vm: &VirtualMachine)-> Rc<FunctionType> {
    match bt {
        BLOCK_TYPE_I32 => {
            return Rc::new(FunctionType {
                tag: 0,
                parameter_types: vec![],
                result_types: vec![ VariableType::I32 ]
            })
        }
        BLOCK_TYPE_I64 => {
            return Rc::new(FunctionType {
                tag: 0,
                parameter_types: vec![],
                result_types: vec![ VariableType::I64 ]
            })
        }
        BLOCK_TYPE_F32 => {
            return Rc::new(FunctionType {
                tag: 0,
                parameter_types: vec![],
                result_types: vec![ VariableType::F32 ]
            })
        }
        BLOCK_TYPE_F64 => {
            return Rc::new(FunctionType {
                tag: 0,
                parameter_types: vec![],
                result_types: vec![ VariableType::F64 ]
            })
        }
        BLOCK_TYPE_EMPTY => {
            return Rc::new(FunctionType {
                tag: 0,
                parameter_types: vec![],
                result_types: vec![]
            })
        }

        _ => {
            return Rc::clone(&vm.module.type_section[bt as usize]);
        }
    }
}

pub struct BlockInst {
    pub bt: BlockArguments
}

impl Instruction for BlockInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        vm.enter_block(0x02 /* BLOCK */,
                       make_func_type(self.bt.block_type, vm),
                       Rc::clone(&self.bt.insts));
    }
}

pub struct LoopInst {
    pub bt: BlockArguments
}

impl Instruction for LoopInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        vm.enter_block(0x03 /* LOOP */,
                       make_func_type(self.bt.block_type, vm),
                       Rc::clone(&self.bt.insts));
    }
}

pub struct IfInst {
    pub if_arguments: IfArguments
}

impl Instruction for IfInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let expr= {
            Rc::clone({
                if vm.operand_stack.pop_bool() {
                    &self.if_arguments.insts_1
                } else {
                    &self.if_arguments.insts_2.as_ref().unwrap()
                }
            })
        };
        vm.enter_block(0x04, /* IF */
                       make_func_type(self.if_arguments.block_type, vm),
                       expr);
    }
}

fn br(label_index: usize, vm: &mut VirtualMachine) {
    for i in 0..label_index {
        vm.control_stack.pop();
    }

    let top = vm.control_stack.top();
    if top.op_code == 0x03 /* LOOP */ {
        top.program_counter.set(0);
        vm.reset_block(top);
    } else {
        vm.exit_block();
    }
}

pub struct BrInst {
    pub lable_index: LabelIndex
}

impl Instruction for BrInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        br(self.lable_index as usize, vm);
    }
}

pub struct BrIfInst {
    pub lable_index: LabelIndex
}

impl Instruction for BrIfInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        if vm.operand_stack.pop_bool() {
            br(self.lable_index as usize, vm);
        }
    }
}

pub struct BrTableInst {
    pub br_table_arguments: BrTableArguments
}

impl Instruction for BrTableInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let n = vm.operand_stack.pop_u32() as usize;

        if n < self.br_table_arguments.labels.len() {
            br(self.br_table_arguments.labels[n] as usize, vm);
        } else {
            br(self.br_table_arguments.default as usize, vm);
        }
    }
}

pub struct ReturnInst {

}

impl Instruction for ReturnInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let (_, lableIndex) = vm.control_stack.top_call_frame();
        br(lableIndex, vm);
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
        let local_count = vm.module.code_section[index].get_local_count();
        for i in 0..local_count {
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