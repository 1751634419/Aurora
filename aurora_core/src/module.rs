use crate::instr::inst::Expression;
use std::rc::Rc;

pub struct Module {
    pub version: u32,
    pub custom_sections: Vec<CustomSection>,
    pub type_section: Vec<Rc<FunctionType>>,
    pub import_section: Vec<Import>,
    pub func_section: Vec<TypeIndex>,
    pub table_section: Vec<TableType>,
    pub memory_section: Vec<MemoryType>,
    pub global_section: Vec<Global>,
    pub export_section: Vec<Export>,
    pub start_section: FunctionIndex,
    pub element_section: Vec<Element>,
    pub code_section: Vec<Code>,
    pub data_section: Vec<Data>
}

pub struct CustomSection {
    pub name: String,
    pub data: Vec<u8>
}

pub type TypeIndex = u32;
pub type FunctionIndex = u32;
pub type TableIndex = u32;
pub type MemoryIndex = u32;
pub type GlobalIndex = u32;
pub type LocalIndex = u32;
pub type LabelIndex = u32;

pub type BlockType = i32;

pub enum VariableType {
    I32, I64, F32, F64
}

pub struct FunctionType {
    pub tag: u8,
    pub parameter_types: Vec<VariableType>,
    pub result_types: Vec<VariableType>
}

pub struct Import {
    pub module: String,
    pub name: String,
    pub description: ImportDescription
}

pub enum ImportTag {
    Function,
    Table,
    Memory,
    Global
}

pub struct ImportDescription {
    pub tag: ImportTag,
    pub function_type: Option<TypeIndex>,
    pub table: Option<TableType>,
    pub memory: Option<MemoryType>,
    pub global: Option<GlobalType>
}

pub struct Limits {
    pub tag: u8,
    pub min: u32,
    pub max: Option<u32>
}

pub struct TableType {
    pub element_type: u8,
    pub limits: Limits
}

pub type MemoryType = Limits;

pub struct Global {
    pub global_type: Rc<GlobalType>,
    pub default: Expression
}

pub struct GlobalType {
    pub variable_type: VariableType,
    pub mutable: bool
}

pub struct Export {
    pub name: String,
    pub description: ExportDescription
}

pub struct ExportDescription {
    pub tag: ExportTag,
    pub index: u32
}

pub enum ExportTag {
    Function,
    Table,
    Memory,
    Global
}

pub struct Element {
    pub table: TableIndex,
    pub offset: Expression,
    pub default: Vec<FunctionIndex>
}

pub struct Locals {
    pub count: u32,
    pub local_type: VariableType
}

pub struct Code {
    pub locals: Vec<Locals>,
    pub expression: Expression
}

impl Code {
    pub fn get_local_count(&self) -> usize {
        let mut n = 0_usize;
        for i in 0..self.locals.len() {
            n += self.locals[i].count as usize;
        }
        return n;
    }
}

pub struct Data {
    pub memory: MemoryIndex,
    pub offset: Expression,
    pub default: Vec<u8>
}