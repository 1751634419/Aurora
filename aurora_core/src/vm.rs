use crate::module::{Module, MemoryType, FunctionType, GlobalType};
use std::convert::TryInto;
use std::cell::Cell;
use std::option::Option::Some;
use crate::instr::inst::Expression;
use std::rc::Rc;

pub struct VirtualMachine {
    pub module: Rc<Module>,
    pub operand_stack: OperandStack,
    pub control_stack: ControlStack,
    pub memory: Memory,
    pub global_table: GlobalTable
}

pub struct GlobalTable {
    globals: Vec<Rc<GlobalVariable>>
}

impl GlobalTable {
    pub fn new_global_type() -> GlobalTable {
        return GlobalTable {
            globals: Vec::new()
        }
    }

    pub fn get_global(&self, index: usize) -> Rc<GlobalVariable> {
        return self.globals[index].clone();
    }

    pub fn push_global(&mut self, global: Rc<GlobalVariable>) {
        self.globals.push(global);
    }

    pub fn get_u64(&self, index: usize) -> u64 {
        return self.globals[index].val.get();
    }

    pub fn set_u64(&self, index: usize, val: u64) {
        let global = self.globals[index].clone();

        if !global.global_type.mutable {
            panic!("immutable global")
        }

        global.val.set(val);
    }
}

pub struct GlobalVariable {
    pub global_type: Rc<GlobalType>,
    pub val: Cell<u64>
}

pub struct Memory {
    pub memory_type: MemoryType,
    pub content: Vec<u8>
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

    pub fn grow(&mut self, n: u32) -> u32 {
        let old_page_size = self.size();
        if let Some(max) = self.memory_type.max {
            if old_page_size + n > max {
                return 0xFFFFFFFF;
            }
        }
        let pushing_count = old_page_size * 65536_u32;
        for i in 0..pushing_count {
            self.content.push(0);
        }

        return old_page_size;
    }
}

pub struct OperandStack {
    pub slots: Vec<u64>,
    pub local_0_index: usize
}

// implement the traits of the local variable table
impl OperandStack {
    pub fn size(&self) -> usize {
        return self.slots.len();
    }

    pub fn get(&self, index: usize) -> u64 {
        return self.slots[index];
    }

    pub fn set(&mut self, index: usize, val: u64) {
        self.slots[index] = val;
    }
}

// implement the traits of operand stack
impl OperandStack {
    pub fn new_operand_stack() -> OperandStack {
        return OperandStack {
            slots: Vec::new(),
            local_0_index: 0
        }
    }

    pub fn push_u64(&mut self, val: u64) {
        self.slots.push(val);
    }

    pub fn pop_u64(&mut self) -> u64 {
        return self.slots.remove(self.slots.len() - 1);
    }

    pub fn push_u32(&mut self, val: u32) {
        self.push_u64(val as u64);
    }

    pub fn pop_u32(&mut self) -> u32 {
        return self.pop_u64() as u32;
    }

    pub fn push_i64(&mut self, val: i64) {
        self.push_u64(val as u64);
    }

    pub fn pop_i64(&mut self) -> i64 {
        return self.pop_u64() as i64;
    }

    pub fn push_i32(&mut self, val: i32) {
        self.push_u32(val as u32);
    }

    pub fn pop_i32(&mut self) -> i32 {
        return self.pop_u32() as i32;
    }

    pub fn push_f64(&mut self, val: f64) {
        self.push_u64(u64::from_be_bytes(val.to_be_bytes()));
    }

    pub fn pop_f64(&mut self) -> f64 {
        return f64::from_be_bytes(self.pop_u64().to_be_bytes())
    }

    pub fn push_f32(&mut self, val: f32) {
        self.push_u32(u32::from_le_bytes(val.to_le_bytes()));
    }

    pub fn pop_f32(&mut self) -> f32 {
        return f32::from_le_bytes(self.pop_u32().to_le_bytes());
    }

    pub fn push_bool(&mut self, val: bool) {
        let val = if val { 1 } else { 0 };
        self.push_i32(val);
    }

    pub fn pop_bool(&mut self) -> bool {
        return self.pop_i32() != 0
    }

    pub fn top(&self) -> u64 {
        return self.slots[self.slots.len() - 1];
    }

    pub fn push_u64s(&mut self, table: &Vec<u64>) {
        for i in 0..table.len() {
            self.push_u64(table[i]);
        }
    }

    pub fn pop_u64s(&mut self, n: usize) -> Vec<u64> {
        let mut table: Vec<u64> = Vec::new();

        for i in 0..n {
            table.push(self.pop_u64());
        }

        return table;
    }
}

pub struct ControlFrame {
    pub op_code: u8,
    pub block_type: Rc<FunctionType>,
    pub instructions: Expression, // Rc<..>
    pub base_pointer: usize,
    pub program_counter: Cell<usize>
}

pub struct ControlStack {
    stack: Vec<Rc<ControlFrame>>
}

impl ControlStack {
    pub fn new_control_stack() -> ControlStack {
        return ControlStack {
            stack: Vec::new()
        }
    }

    pub fn push(&mut self, frame: Rc<ControlFrame>) {
        self.stack.push(frame);
    }

    pub fn pop(&mut self) -> Rc<ControlFrame> {
        return Rc::clone(&self.stack.remove(self.stack.len() - 1));
    }

    pub fn local_top_index(&self) -> usize {
        return self.top_call_frame().0.base_pointer;
    }

    pub fn top(&self) -> Rc<ControlFrame> {
        return Rc::clone(&self.stack[self.stack.len() - 1])
    }

    pub fn depth(&self) -> usize {
        return self.stack.len();
    }

    pub fn top_call_frame(&self) -> (Rc<ControlFrame>, usize) {
        let size = self.stack.len();
        for y in 0..size {
            let i = size - y - 1;
            if self.stack[i].op_code == 0x10 { // CALL: 0x10
                return (Rc::clone(&self.stack[i]), i);
            }
        }

        panic!("the top call frame isn't found");
    }
}