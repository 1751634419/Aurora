use crate::vm::VirtualMachine;
use crate::module::{BlockType, LabelIndex};
use std::rc::Rc;

pub trait Instruction {
    fn Execute(&self, vm: &mut VirtualMachine);
}

pub type Expression = Vec<Box<dyn Instruction>>;

pub struct BlockArguments {
    pub block_type: BlockType,
    pub insts: Expression
}

pub struct IfArguments {
    pub block_type: BlockType,
    pub insts_1: Expression,
    pub insts_2: Option<Expression>
}

pub struct BrTableArguments {
    pub labels: Vec<LabelIndex>,
    pub default: LabelIndex
}

pub struct MemoryArguments {
    pub align: u32,
    pub offset: u32
}