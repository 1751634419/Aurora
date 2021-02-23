use crate::vm::{VirtualMachine, ControlFrame};
use crate::instr::inst::Expression;
use crate::module::FunctionType;
use std::rc::Rc;
use std::cell::Cell;

impl VirtualMachine {
    pub fn reset_block(&mut self, frame: Rc<ControlFrame>) {
        let results = self.operand_stack.pop_u64s(frame.block_type.parameter_types.len());
        self.operand_stack.pop_u64s(self.operand_stack.size() - frame.base_pointer);
        self.operand_stack.push_u64s(&results);
    }

    pub fn enter_block(&mut self, op_code: u8, bt: Rc<FunctionType>, expr: Expression) {
        let bp = self.operand_stack.size() - bt.parameter_types.len();
        let cf = ControlFrame {
            op_code,
            block_type: bt,
            instructions: expr,
            base_pointer: bp,
            program_counter: Cell::new(0)
        };
        self.control_stack.push(Rc::new(cf));

        if op_code == 0x10 {
            self.operand_stack.local_0_index = bp;
        }
    }

    pub fn exit_block(&mut self) {
        let mut cf = self.control_stack.pop();

        let results = self.operand_stack.pop_u64s(cf.block_type.result_types.len());

        let dropNum = self.operand_stack.size() - cf.base_pointer;
        self.operand_stack.pop_u64s(dropNum);

        self.operand_stack.push_u64s(&results);

        if cf.op_code == 0x10 && self.control_stack.depth() > 0 {
            self.operand_stack.local_0_index = self.control_stack.local_top_index();
        }
    }

    pub fn execute(&mut self) {
        let initial_depth = self.control_stack.depth();

        while self.control_stack.depth() >= initial_depth {
            let cf = self.control_stack.top();

            let pc = cf.program_counter.get();
            if pc == cf.instructions.len() {
                self.exit_block();
            } else {
                cf.program_counter.set(pc + 1);
                cf.instructions[pc].execute(self);
            }
        }
    }

    pub fn execute_expr(&mut self, expr: &Expression) {
        for i in 0..expr.len() {
            expr.get(i).unwrap().execute(self);
        }
    }
}