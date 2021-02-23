use crate::module::{Module, MemoryType};
use std::convert::TryInto;
use std::cell::Cell;
use std::option::Option::Some;
use std::rc::Rc;

pub struct VirtualMachine {
    pub module: Rc<Module>,
    pub operand_stack: OperandStack,
    pub memory: Memory
}

pub struct Memory {
    memory_type: MemoryType,
    content: Vec<u8>
}

impl Memory {
    pub fn new_memory(mt: MemoryType) -> Memory {
        let mut content: Vec<u8> = Vec::new();
        let initial_len = (mt.min as usize) * 65536_usize;
        for i in 0..initial_len {
            content.push(0);
        }
        return Memory {
            memory_type: mt,
            content: content
        }
    }

    pub fn write(&mut self, offset: u64, data: &[u8]) {
        let offset = offset as usize;
        for i in 0..data.len() {
            self.content[i + offset] = data[i];
        }
    }

    pub fn write_vec(&mut self, offset: u64, data: &Vec<u8>) {
        let offset = offset as usize;
        for i in 0..data.len() {
            self.content[i + offset] = data[i];
        }
    }

    pub fn read(&self, offset: u64, buffer: &mut [u8]) {
        let offset = offset as usize;
        for i in 0..buffer.len() {
            buffer[i] = self.content[i + offset];
        }
    }

    pub fn size(&self) -> u32 {
        return (self.content.len() as u32) / 65536_u32;
    }

    pub fn grow(&mut self, n: u32) -> Result<u32, String> {
        let old_page_size = self.size();
        if let Some(max) = self.memory_type.max {
            if old_page_size + n > max {
                return Result::Err("memory overflowed".to_string());
            }
        }
        let pushing_count = old_page_size * 65536_u32;
        for i in 0..pushing_count {
            self.content.push(0);
        }

        return Ok(old_page_size)
    }
}

pub struct OperandStack {
    pub slots: Vec<u64>
}

impl OperandStack {
    pub fn new_operand_stack() -> OperandStack {
        return OperandStack {
            slots: Vec::new(),
        }
    }

    pub fn push_u64(&mut self, val: u64) {
        self.slots.push(val);
    }

    pub fn pop_u64(&mut self) -> Option<u64> {
        return self.slots.pop();
    }

    pub fn push_u32(&mut self, val: u32) {
        // self.push_i32(val as i32);
        self.push_u64(val as u64);
    }

    pub fn pop_u32(&mut self) -> Option<u32> {
        // if let Some(val) = self.pop_i32() {
            // return Some(val as u32);
        // }
        if let Some(val) = self.pop_u64() {
            return Some(val as u32);
        }

        return None;
    }

    pub fn push_i64(&mut self, val: i64) {
        self.push_u64(val as u64);
        // self.push_u64(u64::from_be_bytes(val.to_be_bytes()));
    }

    pub fn pop_i64(&mut self) -> Option<i64> {
        if let Some(val) = self.pop_u64() {
            return Some(val as i64);
            // return Some(i64::from_be_bytes(val.to_be_bytes()));
        }

        return None;
    }

    pub fn push_i32(&mut self, val: i32) {
        self.push_u32(val as u32);
        // let mut arr: [u8; 8] = [0; 8];
        // let m = val.to_be_bytes();
        // for i in 0..4 {
        //     arr[i] = m[i];
        // }
        // self.push_u64(u64::from_be_bytes(arr));
    }

    pub fn pop_i32(&mut self) -> Option<i32> {
        // if let Some(val) = self.pop_u64() {
        //     let arr = val.to_be_bytes();
        //     let mut m : [u8; 4] = [0; 4];
        //     for i in 0..4 {
        //         m[i] = arr[i];
        //     }
        //     return Some(i32::from_be_bytes(m));
        // }
        if let Some(val) = self.pop_u32() {
            return Some(val as i32);
        }

        return None;
    }

    pub fn push_f64(&mut self, val: f64) {
        self.push_u64(u64::from_be_bytes(val.to_be_bytes()));
    }

    pub fn pop_f64(&mut self) -> Option<f64> {
        if let Some(val) = self.pop_u64() {
            return Some(f64::from_be_bytes(val.to_be_bytes()));
        }

        return None;
    }

    pub fn push_f32(&mut self, val: f32) {
        self.push_u32(u32::from_le_bytes(val.to_le_bytes()));
    }

    pub fn pop_f32(&mut self) -> Option<f32> {
        if let Some(val) = self.pop_u32() {
            return Some(f32::from_le_bytes(val.to_le_bytes()));
        }

        return None;
    }

    pub fn push_bool(&mut self, val: bool) {
        let val = if val { 1 } else { 0 };
        self.push_i32(val);
    }

    pub fn pop_bool(&mut self) -> Option<bool> {
        if let Some(val) = self.pop_i32() {
            return Some(val != 0)
        }

        return None;
    }
}