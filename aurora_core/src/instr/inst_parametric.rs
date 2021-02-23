use crate::instr::inst::Instruction;
use crate::vm::VirtualMachine;

pub struct DropInst {

}

impl Instruction for DropInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        vm.operand_stack.pop_u64();
    }
}

pub struct SelectInst {

}

impl Instruction for SelectInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let flag = vm.operand_stack.pop_bool();
        let (b, a) = (vm.operand_stack.pop_u64(), vm.operand_stack.pop_u64());

        vm.operand_stack.push_u64(
            if flag {
                a
            } else {
                b
            }
        );
    }
}