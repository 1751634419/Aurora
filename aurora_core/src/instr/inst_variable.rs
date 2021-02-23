use crate::instr::inst::Instruction;
use crate::vm::VirtualMachine;
use crate::module::{LocalIndex, GlobalIndex};

pub struct LocalGetInst {
    pub local_index: LocalIndex
}

impl Instruction for LocalGetInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let idx = vm.operand_stack.local_0_index + (self.local_index as usize);
        let val = vm.operand_stack.get(idx);
        vm.operand_stack.push_u64(val);
    }
}

pub struct LocalSetInst {
    pub local_index: LocalIndex
}

impl Instruction for LocalSetInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let idx = vm.operand_stack.local_0_index + (self.local_index as usize);
        let val = vm.operand_stack.pop_u64();
        vm.operand_stack.set(idx, val);
    }
}

pub struct LocalTeeInst {
    pub local_index: LocalIndex
}

impl Instruction for LocalTeeInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let idx = vm.operand_stack.local_0_index + (self.local_index as usize);
        let val = vm.operand_stack.top();
        vm.operand_stack.set(idx, val);
    }
}

pub struct GlobalGetInst {
    pub global_index: GlobalIndex
}

impl Instruction for GlobalGetInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.global_table.get_u64(self.global_index as usize);
        vm.operand_stack.push_u64(val);
    }
}

pub struct GlobalSetInst {
    pub global_index: GlobalIndex
}

impl Instruction for GlobalSetInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_u64();
        vm.global_table.set_u64(self.global_index as usize, val);
    }
}