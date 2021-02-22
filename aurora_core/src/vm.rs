use crate::module::Module;
use std::convert::TryInto;
use std::cell::Cell;
use std::option::Option::Some;

pub struct VirtualMachine {
    pub module: Module,
    pub operand_stack: OperandStack,
}

pub struct OperandStack {
    pub slots: Vec<Cell<u64>>,
    current_index: Cell<usize>
}

impl OperandStack {
    pub fn new_operand_stack(capacity: usize) -> OperandStack {
        let mut slots = Vec::new();
        for i in 0..capacity {
            slots.push(Cell::new(0));
        }
        return OperandStack {
            slots: slots,
            current_index: Cell::new(0)
        }
    }

    pub fn push_u64(&self, val: u64) {
        self.slots[self.current_index.get()].set(val);
        self.current_index.set(self.current_index.get() + 1);
    }

    pub fn pop_u64(&self) -> Option<u64> {
        self.current_index.set(self.current_index.get() - 1);
        if let Some(c) = self.slots.get(self.current_index.get()) {
            return Some(c.get());
        } else {
            return None;
        }
    }

    pub fn push_u32(&self, val: u32) {
        self.push_i32(val as i32);
    }

    pub fn pop_u32(&self) -> Option<u32> {
        if let Some(val) = self.pop_i32() {
            // println!("{}", val);
            return Some(val as u32);
        }

        return None;
    }

    pub fn push_i64(&self, val: i64) {
        self.push_u64(val as u64);
        // self.push_u64(u64::from_be_bytes(val.to_be_bytes()));
    }

    pub fn pop_i64(&self) -> Option<i64> {
        if let Some(val) = self.pop_u64() {
            return Some(val as i64);
            // return Some(i64::from_be_bytes(val.to_be_bytes()));
        }

        return None;
    }

    pub fn push_i32(&self, val: i32) {
        // self.push_u64(u64::from_be_bytes((val as i64).to_be_bytes()));
        let mut arr: [u8; 8] = [0; 8];
        let m = val.to_be_bytes();
        for i in 0..4 {
            arr[i] = m[i];
        }
        self.push_u64(u64::from_be_bytes(arr));
    }

    pub fn pop_i32(&self) -> Option<i32> {
        if let Some(val) = self.pop_u64() {
            // return Some(val as i32);
            let arr = val.to_be_bytes();
            let mut m : [u8; 4] = [0; 4];
            for i in 0..4 {
                m[i] = arr[i];
            }
            return Some(i32::from_be_bytes(m));
        }

        return None;
    }

    pub fn push_f64(&self, val: f64) {
        self.push_u64(u64::from_be_bytes(val.to_be_bytes()));
    }

    pub fn pop_f64(&self) -> Option<f64> {
        if let Some(val) = self.pop_u64() {
            return Some(f64::from_be_bytes(val.to_be_bytes()));
        }

        return None;
    }

    pub fn push_f32(&self, val: f32) {
        self.push_u64(u64::from_be_bytes((val as f64).to_be_bytes()));
    }

    pub fn pop_f32(&self) -> Option<f32> {
        if let Some(val) = self.pop_u64() {
            return Some(f64::from_be_bytes(val.to_be_bytes()) as f32);
        }

        return None;
    }

    pub fn push_bool(&self, val: bool) {
        let val = if val { 1 } else { 0 };
        self.push_i32(val);
    }

    pub fn pop_bool(&self) -> Option<bool> {
        if let Some(val) = self.pop_i32() {
            return Some(val != 0)
        }

        return None;
    }
}