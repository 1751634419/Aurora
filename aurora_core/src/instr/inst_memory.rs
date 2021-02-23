use crate::instr::inst::{Instruction, MemoryArguments};
use crate::vm::VirtualMachine;

fn read_u8(vm: &mut VirtualMachine, offset: u32) -> u8 {
    let start_addr = vm.operand_stack.pop_u32() as u64 + offset as u64;
    let mut data: [u8; 1] = [0; 1];
    vm.memory.read(start_addr, &mut data);
    return u8::from_le_bytes(data);
}

fn read_u16(vm: &mut VirtualMachine, offset: u32) -> u16 {
    let start_addr = vm.operand_stack.pop_u32() as u64 + offset as u64;
    let mut data: [u8; 2] = [0; 2];
    vm.memory.read(start_addr, &mut data);
    return u16::from_le_bytes(data);
}

fn read_u32(vm: &mut VirtualMachine, offset: u32) -> u32 {
    let start_addr = vm.operand_stack.pop_u32() as u64 + offset as u64;
    let mut data: [u8; 4] = [0; 4];
    vm.memory.read(start_addr, &mut data);
    return u32::from_le_bytes(data);
}

fn read_u64(vm: &mut VirtualMachine, offset: u32) -> u64 {
    let start_addr = vm.operand_stack.pop_u32() as u64 + offset as u64;
    let mut data: [u8; 8] = [0; 8];
    vm.memory.read(start_addr, &mut data);
    return u64::from_le_bytes(data);
}

fn write_u8(vm: &mut VirtualMachine, offset: u32, val: u8) {
    let start_addr = vm.operand_stack.pop_u32() as u64 + offset as u64;
    let mut data: [u8; 1] = val.to_le_bytes();
    vm.memory.write(start_addr, &mut data);
}

fn write_u16(vm: &mut VirtualMachine, offset: u32, val: u16) {
    let start_addr = vm.operand_stack.pop_u32() as u64 + offset as u64;
    let mut data: [u8; 2] = val.to_le_bytes();
    vm.memory.write(start_addr, &mut data);
}

fn write_u32(vm: &mut VirtualMachine, offset: u32, val: u32) {
    let start_addr = vm.operand_stack.pop_u32() as u64 + offset as u64;
    let mut data: [u8; 4] = val.to_le_bytes();
    vm.memory.write(start_addr, &mut data);
}

fn write_u64(vm: &mut VirtualMachine, offset: u32, val: u64) {
    let start_addr = vm.operand_stack.pop_u32() as u64 + offset as u64;
    let mut data: [u8; 8] = val.to_le_bytes();
    vm.memory.write(start_addr, &mut data);
}

pub struct I32LoadInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I32LoadInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u32(vm, self.memory_arguments.offset);
        vm.operand_stack.push_u32(val);
    }
}

pub struct I64LoadInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64LoadInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u64(vm, self.memory_arguments.offset);
        vm.operand_stack.push_u64(val);
    }
}

pub struct F32LoadInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for F32LoadInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u32(vm, self.memory_arguments.offset);
        let val = f32::from_le_bytes(val.to_le_bytes());
        vm.operand_stack.push_f32(val);
    }
}

pub struct F64LoadInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for F64LoadInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u64(vm, self.memory_arguments.offset);
        let val = f64::from_le_bytes(val.to_le_bytes());
        vm.operand_stack.push_f64(val);
    }
}

pub struct I32Load8SInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I32Load8SInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u8(vm, self.memory_arguments.offset);
        let val = val as i8;
        vm.operand_stack.push_i32(val as i32);
    }
}

pub struct I32Load8UInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I32Load8UInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u8(vm, self.memory_arguments.offset);
        vm.operand_stack.push_u32(val as u32);
    }
}

pub struct I32Load16SInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I32Load16SInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u16(vm, self.memory_arguments.offset);
        let val = val as i16;
        vm.operand_stack.push_i32(val as i32);
    }
}

pub struct I32Load16UInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I32Load16UInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u16(vm, self.memory_arguments.offset);
        vm.operand_stack.push_u32(val as u32);
    }
}

pub struct I64Load8SInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64Load8SInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u8(vm, self.memory_arguments.offset);
        let val = val as i8;
        vm.operand_stack.push_i64(val as i64);
    }
}

pub struct I64Load8UInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64Load8UInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u8(vm, self.memory_arguments.offset);
        vm.operand_stack.push_u64(val as u64);
    }
}

pub struct I64Load16SInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64Load16SInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u16(vm, self.memory_arguments.offset);
        let val = val as i16;
        vm.operand_stack.push_i64(val as i64);
    }
}

pub struct I64Load16UInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64Load16UInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u16(vm, self.memory_arguments.offset);
        vm.operand_stack.push_u64(val as u64);
    }
}

pub struct I64Load32SInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64Load32SInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u32(vm, self.memory_arguments.offset);
        let val = val as i32;
        vm.operand_stack.push_i64(val as i64);
    }
}

pub struct I64Load32UInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64Load32UInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = read_u32(vm, self.memory_arguments.offset);
        vm.operand_stack.push_u64(val as u64);
    }
}

pub struct I32StoreInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I32StoreInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_u32();
        write_u32(vm, self.memory_arguments.offset, val);
    }
}

pub struct I64StoreInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64StoreInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_u64();
        write_u64(vm, self.memory_arguments.offset, val);
    }
}

pub struct F32StoreInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for F32StoreInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_f32();
        let val = u32::from_le_bytes(val.to_le_bytes());
        write_u32(vm, self.memory_arguments.offset, val);
    }
}

pub struct F64StoreInst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for F64StoreInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_f64();
        let val = u64::from_le_bytes(val.to_le_bytes());
        write_u64(vm, self.memory_arguments.offset, val);
    }
}

pub struct I32Store8Inst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I32Store8Inst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_u32();
        let val = val as u8;
        write_u8(vm, self.memory_arguments.offset, val);
    }
}

pub struct I32Store16Inst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I32Store16Inst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_u32();
        let val = val as u16;
        write_u16(vm, self.memory_arguments.offset, val);
    }
}

pub struct I64Store8Inst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64Store8Inst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_u64();
        let val = val as u8;
        write_u8(vm, self.memory_arguments.offset, val);
    }
}

pub struct I64Store16Inst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64Store16Inst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_u64();
        let val = val as u16;
        write_u16(vm, self.memory_arguments.offset, val);
    }
}

pub struct I64Store32Inst {
    pub memory_arguments: MemoryArguments
}

impl Instruction for I64Store32Inst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let val = vm.operand_stack.pop_u64();
        let val = val as u32;
        write_u32(vm, self.memory_arguments.offset, val);
    }
}

pub struct MemorySizeInst {

}

impl Instruction for MemorySizeInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let size = vm.memory.size();
        vm.operand_stack.push_u32(size);
    }
}

pub struct MemoryGrowInst {

}

impl Instruction for MemoryGrowInst {
    fn execute(&self, vm: &mut VirtualMachine) {
        let n = vm.operand_stack.pop_u32();
        let result = vm.memory.grow(n);
        vm.operand_stack.push_u32(result);
    }
}