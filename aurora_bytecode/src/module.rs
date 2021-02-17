use std::sync::Arc;
use std::any::Any;

pub struct Module {
    pub version: u32,
    pub custom_sections: CustomSection,
    pub type_section: Vec<FunctionType>,
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
    pub name: str,
    pub data: Vec<u8>
}

// TODO
pub type Expression = Option<Any>;

pub type TypeIndex = u32;
pub type FunctionIndex = u32;
pub type TableIndex = u32;
pub type MemoryIndex = u32;
pub type GlobalIndex = u32;
pub type LocalIndex = u32;
pub type LabelIndex = u32;

pub enum VariableType {
    I32 = 0x7F as u8,
    I64 = 0x7E as u8,
    F32 = 0x7D as u8,
    F64 = 0x7C as u8
}

pub struct FunctionType {
    pub parameter_types: Vec<VariableType>,
    pub result_types: Vec<VariableType>
}

pub struct Import {
    pub module: str,
    pub name: str,
    pub description: ImportDescription
}

pub enum ImportTag {
    Function = 0,
    Table = 1,
    Memory = 2,
    Global = 3
}

pub struct ImportDescription {
    pub tag: ImportTag,
    pub function_type: Option<TypeIndex>,
    pub table: Option<TableIndex>,
    pub memory: Option<MemoryIndex>,
    pub global: Option<GlobalIndex>
}

pub struct Limits {
    pub tag: u8,
    pub min: u32,
    pub max: u32
}

pub type TableType = Limits;

pub type MemoryType = Limits;

pub struct Global {
    pub global_type: GlobalType,
    pub default: Expression
}

pub struct GlobalType {
    pub variable_type: VariableType,
    pub mutable: bool
}

pub struct Export {
    pub name: str,
    pub description: ExportDescription
}

pub struct ExportDescription {
    pub tag: ExportTag,
    pub index: u32
}

pub enum ExportTag {
    Function = 0,
    Table = 1,
    Memory = 2,
    Global = 3
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

pub struct Data {
    pub memory: MemoryIndex,
    pub offset: Expression,
    pub default: Vec<u8>
}