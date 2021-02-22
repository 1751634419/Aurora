use crate::instr::inst::Instruction;
use crate::vm::VirtualMachine;

pub struct DropInst {

}

impl Instruction for DropInst {
    fn Execute(&self, vm: &VirtualMachine) {
        vm.operand_stack.pop_u64();
    }
}

pub struct SelectInst {

}

impl Instruction for SelectInst {
    fn Execute(&self, vm: &VirtualMachine) {
        let flag = vm.operand_stack.pop_bool().unwrap();
        let (b, a) = (vm.operand_stack.pop_u64().unwrap(), vm.operand_stack.pop_u64().unwrap());

        vm.operand_stack.push_u64(
            if flag {
                a
            } else {
                b
            }
        );
    }
}