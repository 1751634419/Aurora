use crate::vm::VirtualMachine;
use crate::module::{BlockType, LabelIndex};

pub trait Instruction {
    fn Execute(&self, vm: &VirtualMachine);
}

pub type Expression = Vec<Box<dyn Instruction>>;

pub struct BlockArguments {
    pub block_type: BlockType,
    pub insts: Vec<Box<dyn Instruction>>
}

pub struct IfArguments {
    pub block_type: BlockType,
    pub insts_1: Vec<Box<dyn Instruction>>,
    pub insts_2: Option<Vec<Box<dyn Instruction>>>
}

pub struct BrTableArguments {
    pub labels: Vec<LabelIndex>,
    pub default: LabelIndex
}

pub struct MemoryArguments {
    pub align: u32,
    pub offset: u32
}