use crate::vm::VirtualMachine;
use std::borrow::{BorrowMut, Borrow};
use std::ops::DerefMut;
use crate::instr::inst::Expression;

impl VirtualMachine {
    pub fn execute(&mut self, index: usize) {
        let expr = &self.module.clone().code_section[index].expression;
        for i in 0..expr.len() {
            expr.get(i).unwrap().Execute(self);
        }
    }

    pub fn exec_const_expr(&mut self, expr: &Expression) {
        for i in 0..expr.len() {
            expr.get(i).unwrap().Execute(self);
        }
    }
}