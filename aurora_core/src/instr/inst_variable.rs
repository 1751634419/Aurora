use crate::instr::inst::Instruction;
use crate::vm::VirtualMachine;
use crate::module::{LocalIndex, GlobalIndex};

pub struct LocalGetInst {
    pub local_index: LocalIndex
}

impl Instruction for LocalGetInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct LocalSetInst {
    pub local_index: LocalIndex
}

impl Instruction for LocalSetInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct LocalTeeInst {
    pub local_index: LocalIndex
}

impl Instruction for LocalTeeInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct GlobalGetInst {
    pub global_index: GlobalIndex
}

impl Instruction for GlobalGetInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}

pub struct GlobalSetInst {
    pub global_index: GlobalIndex
}

impl Instruction for GlobalSetInst {
    fn Execute(&self, vm: &mut VirtualMachine) {
        // todo UNIMPLEMENTED
    }
}