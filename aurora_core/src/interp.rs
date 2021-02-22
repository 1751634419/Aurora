use crate::vm::VirtualMachine;
use std::borrow::BorrowMut;
use std::ops::DerefMut;

impl VirtualMachine {
    pub fn execute(&mut self, index: usize) {
        let expr = &self.module.code_section[index].expression;
        for i in 0..expr.len() {
            let inst = &expr[i];
            inst.Execute(self);
        }
    }
}