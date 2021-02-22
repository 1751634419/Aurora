use crate::instr::inst::Instruction;
use crate::vm::VirtualMachine;

pub struct DropInst {

}

impl Instruction for DropInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct SelectInst {

}

impl Instruction for SelectInst {
    fn Execute(&self, vm: &VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}