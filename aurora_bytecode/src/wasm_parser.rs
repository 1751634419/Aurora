use std::convert::TryInto;
use aurora_core::module::{Module, CustomSection, FunctionType, VariableType, Import, ImportDescription, TableType, Limits, GlobalType, MemoryType, Global, ExportDescription, Export, ExportTag, Element, Code, Locals, Data};
use std::string::FromUtf8Error;

// parser supporting LEB-128
// see https://en.wikipedia.org/wiki/LEB128#Decode_signed_integer for details
fn decode_var_int(data: &Vec<u8>, beginning_point: usize, size: usize) -> Result<(i128, usize), String> { // T(result, end pointer index)
    let mut result: i128 = 0;

    for i in beginning_point..(beginning_point + size) {
        let relative_count = i - beginning_point;
        let b = data[i];
        if relative_count == size / 7 {
            if b & 0x80 != 0 {
                return Err("The integer received is too long.".to_string());
            }

            if b & 0x40 == 0 && b >> (size - relative_count * 7 - 1) != 0 ||
                b & 0x40 != 0 && (b | 0x80) as i8 >> (size - relative_count * 7 - 1) != -1 {
                return Err("The integer is too large.".to_string());
            }
        }
        result |= (b as i128 & 0x7f) << (relative_count * 7);
        if b & 0x80 == 0 {
            if (relative_count * 7 < size) && (b & 0x40 != 0) {
                result = result | (-1 << ((relative_count + 1) * 7));
            }

            return Ok((result, i + 1));
        }
    }
    return Err("Unexpected End".to_string());
}

// https://en.wikipedia.org/wiki/LEB128#Decode_unsigned_integer
fn decode_var_uint(data: &Vec<u8>, beginning_point: usize, size: usize) -> Result<(u128, usize), String> { // T(result, end pointer index)
    let mut result: u128 = 0;

    for i in beginning_point..(beginning_point + size) {
        let relative_count = i - beginning_point;
        let b = data[i];
        if relative_count == size / 7 {
            if b & 0x80 != 0 {
                return Err("The integer received is too long.".to_string());
            }

            if b >> (size - relative_count * 7) > 0 {
                return Err("The integer is too large.".to_string());
            }
        }
        result |= (b as u128 & 0x7f) << (relative_count * 7);
        if b & 0x80 == 0 {
            return Ok((result, i + 1));
        }
    }
    return Err("Unexpected End".to_string());
}

pub struct WasmReader {
    data: Vec<u8>,
    point: usize,
}

// basic type reading
impl WasmReader {
    pub fn new(data: Vec<u8>) -> WasmReader {
        return WasmReader {
            data,
            point: 0,
        }
    }

    pub fn remaining(&self) -> usize {
        return self.data.len() - self.point;
    }

    pub fn read_f32(&mut self) -> f32 {
        let data = self.read_data(4);
        let d: [u8; 4] = [*data.get(0).unwrap(), *data.get(1).unwrap(),
            *data.get(2).unwrap(), *data.get(3).unwrap()];

        return f32::from_le_bytes(d);
    }

    pub fn read_f64(&mut self) -> f64 {
        let data = self.read_data(8);

        let d: [u8; 8] = [*data.get(0).unwrap(), *data.get(1).unwrap(),
            *data.get(2).unwrap(), *data.get(3).unwrap(),
            *data.get(4).unwrap(), *data.get(5).unwrap(),
            *data.get(6).unwrap(), *data.get(7).unwrap()];

        return f64::from_le_bytes(d);
    }

    pub fn read_u64(&mut self) -> u64 {
        let data = self.read_data(8);

        let d: [u8; 8] = [*data.get(0).unwrap(), *data.get(1).unwrap(),
            *data.get(2).unwrap(), *data.get(3).unwrap(),
            *data.get(4).unwrap(), *data.get(5).unwrap(),
            *data.get(6).unwrap(), *data.get(7).unwrap()];

        return u64::from_le_bytes(d);
    }

    pub fn read_u32(&mut self) -> u32 {
        let data = self.read_data(4);
        let d: [u8; 4] = [*data.get(0).unwrap(), *data.get(1).unwrap(),
            *data.get(2).unwrap(), *data.get(3).unwrap()];

        return u32::from_le_bytes(d);
    }

    pub fn read_u8(&mut self) -> u8 {
        let d = self.data[self.point];
        self.point += 1;
        return d;
    }

    pub fn read_data(&mut self, size: usize) -> Vec<u8> {
        let src_data = &self.data[self.point..self.point + size];
        self.point += size;

        return src_data.to_vec();
    }

    /* E*a*s*t*e*r e*g*g: I'm so glad to see you seeing through this project.
     Join our Discord server to communicate: https://discord.gg/uz6QG5cj
     Any kind of message, except for the toxic one, is allowed. :3
     And if you feel bored, feel free to join the server https://discord.gg/WAtA7fvb5R
     I am waiting for the arrival of every one of you.
     Planned to be removed when the virtual machine works normally Lmao*/
    pub fn read_var_i32(&mut self) -> i32 {
        let r = decode_var_int(&self.data, self.point, 32);

        match r {
            Ok(tuple) => {
                self.point = tuple.1;
                return tuple.0.try_into().unwrap();
            }

            Err(err) => {
                panic!(err);
            }
        }
    }

    pub fn read_var_i64(&mut self) -> i64 {
        let r = decode_var_int(&self.data, self.point, 64);

        match r {
            Ok(tuple) => {
                self.point = tuple.1;
                return tuple.0.try_into().unwrap();
            }

            Err(err) => {
                panic!(err);
            }
        }
    }

    pub fn read_var_u32(&mut self) -> u32 {
        let r = decode_var_uint(&self.data, self.point, 32);

        match r {
            Ok(tuple) => {
                self.point = tuple.1;
                return tuple.0.try_into().unwrap();
            }

            Err(err) => {
                panic!(err);
            }
        }
    }

    pub fn read_var_u64(&mut self) -> u64 {
        let r = decode_var_uint(&self.data, self.point, 64);

        match r {
            Ok(tuple) => {
                self.point = tuple.1;
                return tuple.0.try_into().unwrap();
            }

            Err(err) => {
                panic!(err);
            }
        }
    }

    pub fn read_bytes(&mut self) -> Vec<u8> {
        let len = self.read_var_u32();
        let vec = self.read_data(len as usize);
        return vec;
    }

    pub fn read_name(&mut self) -> Result<String, FromUtf8Error> {
        return String::from_utf8(self.read_bytes());
    }
}

const SECTION_CUSTOM_ID : u8 = 0;
const SECTION_TYPE_ID : u8 = 1;
const SECTION_IMPORT_ID : u8 = 2;
const SECTION_FUNCTION_ID : u8 = 3;
const SECTION_TABLE_ID : u8 = 4;
const SECTION_MEMORY_ID : u8 = 5;
const SECTION_GLOBAL_ID : u8 = 6;
const SECTION_EXPORT_ID : u8 = 7;
const SECTION_START_ID : u8 = 8;
const SECTION_ELEMENT_ID : u8 = 9;
const SECTION_CODE_ID : u8 = 10;
const SECTION_DATA_ID : u8 = 11;

const TYPE_I32: u8 = 0x7F;
const TYPE_I64: u8 = 0x7E;
const TYPE_F32: u8 = 0x7D;
const TYPE_F64: u8 = 0x7C;

const IMPORT_TAG_FUNCTION: u8 = 0;
const IMPORT_TAG_TABLE: u8 = 1;
const IMPORT_TAG_MEMORY: u8 = 2;
const IMPORT_TAG_GLOBAL: u8 = 3;

const EXPORT_TAG_FUNCTION: u8 = 0;
const EXPORT_TAG_TABLE: u8 = 1;
const EXPORT_TAG_MEMORY: u8 = 2;
const EXPORT_TAG_GLOBAL: u8 = 3;

// wasm reading
impl WasmReader {
    pub fn read_zero(&mut self) -> u8 {
        let b = self.read_u8();
        if b != 0 {
            panic!("zero flag expected");
        }
        return 0;
    }

    pub fn read_variable_type(&mut self) -> VariableType {
        let v = self.read_u8();

        match v {
            TYPE_I32 => {
                return VariableType::I32;
            }
            TYPE_I64 => {
                return VariableType::I64;
            }
            TYPE_F32 => {
                return VariableType::F32;
            }
            TYPE_F64 => {
                return VariableType::F64;
            }

            _ => {
                panic!("Malformed variable type")
            }
        }
    }

    pub fn read_variable_types(&mut self) -> Vec<VariableType> {
        let mut vec: Vec<VariableType> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_variable_type());
        }

        return vec;
    }

    pub fn read_function_type(&mut self) -> FunctionType {
        return FunctionType {
            tag: self.read_u8(),
            parameter_types: self.read_variable_types(),
            result_types: self.read_variable_types()
        }
    }

    pub fn read_type_section(&mut self) -> Vec<FunctionType> {
        let mut vec: Vec<FunctionType> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_function_type());
        }

        return vec;
    }

    pub fn read_custom_section(&mut self) -> CustomSection {
        let mut tmp_reader = WasmReader::new(self.read_bytes());

        return CustomSection {
            name: tmp_reader.read_name().unwrap(),
            data: tmp_reader.read_data(tmp_reader.remaining())
        };
    }

    pub fn read_limits(&mut self) -> Limits {
        let tag = self.read_u8();
        let min = self.read_var_u32();
        let mut max: Option<u32> = Option::None;

        if tag == 1 {
            max = Option::Some(self.read_var_u32());
        }

        return Limits {
            tag: tag,
            min: min,
            max: max
        };
    }

    pub fn read_table_type(&mut self) -> TableType {
        return TableType {
            element_type: self.read_u8(),
            limits: self.read_limits()
        };
    }

    pub fn read_global_type(&mut self) -> GlobalType {
        return GlobalType {
            variable_type: self.read_variable_type(),
            mutable: self.read_u8() == 1
        };
    }

    pub fn read_import_description(&mut self) -> ImportDescription {
        let desc = self.read_u8();

        match desc {
            IMPORT_TAG_FUNCTION => {
                return ImportDescription {
                    tag: aurora_core::module::ImportTag::Function,
                    function_type: Some(self.read_var_u32()),
                    table: None,
                    memory: None,
                    global: None
                }
            }

            IMPORT_TAG_TABLE => {
                return ImportDescription {
                    tag: aurora_core::module::ImportTag::Table,
                    function_type: None,
                    table: Some(self.read_table_type()),
                    memory: None,
                    global: None
                };
            }

            IMPORT_TAG_MEMORY => {
                return ImportDescription {
                    tag: aurora_core::module::ImportTag::Memory,
                    function_type: None,
                    table: None,
                    memory: Some(self.read_limits()),
                    global: None
                };
            }

            IMPORT_TAG_GLOBAL => {
                return ImportDescription {
                    tag: aurora_core::module::ImportTag::Global,
                    function_type: None,
                    table: None,
                    memory: None,
                    global: Some(self.read_global_type())
                };
            }

            _ => {
                panic!("Malformed import description")
            }
        }
    }

    pub fn read_import(&mut self) -> Import {
        return Import {
            module: self.read_name().unwrap(),
            name: self.read_name().unwrap(),
            description: self.read_import_description()
        };
    }

    pub fn read_import_section(&mut self) -> Vec<Import> {
        let mut vec: Vec<Import> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_import());
        }

        return vec;
    }

    pub fn read_export_description(&mut self) -> ExportDescription {
        let mut tag: ExportTag;
        
        match self.read_u8() {
            EXPORT_TAG_FUNCTION => tag = aurora_core::module::ExportTag::Function,
            EXPORT_TAG_GLOBAL => tag = aurora_core::module::ExportTag::Global,
            EXPORT_TAG_MEMORY => tag = aurora_core::module::ExportTag::Memory,
            EXPORT_TAG_TABLE => tag = aurora_core::module::ExportTag::Table,
            _ => panic!("Unmalformed export tag")
        }
        
        return ExportDescription {
            tag,
            index: self.read_var_u32()
        }
    }

    pub fn read_export(&mut self) -> Export {
        return Export {
            name: self.read_name().unwrap(),
            description: self.read_export_description()
        };
    }

    pub fn read_export_section(&mut self) -> Vec<Export> {
        let mut vec: Vec<Export> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_export());
        }

        return vec;
    }

    pub fn read_indices(&mut self) -> Vec<u32> {
        let mut vec: Vec<u32> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_var_u32());
        }

        return vec;
    }

    pub fn read_table_section(&mut self) -> Vec<TableType> {
        let mut vec: Vec<TableType> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_table_type());
        }

        return vec;
    }

    pub fn read_memory_section(&mut self) -> Vec<MemoryType> {
        let mut vec: Vec<MemoryType> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_limits());
        }

        return vec;
    }

    pub fn read_global_section(&mut self) -> Vec<Global> {
        let mut vec: Vec<Global> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(Global {
                global_type: self.read_global_type(),
                default: self.read_expression().0
            });
        }

        return vec;
    }

    pub fn read_start_section(&mut self) -> u32 {
        return self.read_var_u32();
    }

    pub fn read_element(&mut self) -> Element {
        return Element {
            table: self.read_var_u32(),
            offset: self.read_expression().0,
            default: self.read_indices()
        };
    }

    pub fn read_element_section(&mut self) -> Vec<Element> {
        let mut vec: Vec<Element> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_element());
        }

        return vec;
    }

    pub fn read_locals(&mut self) -> Locals {
        return Locals {
            count: self.read_var_u32(),
            local_type: self.read_variable_type()
        }
    }

    pub fn read_locals_vector(&mut self) -> Vec<Locals> {
        let mut vec: Vec<Locals> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_locals());
        }

        return vec;
    }
    
    pub fn read_code(&mut self) -> Code {
        let n = self.read_var_u32() as usize;
        let remaining = self.remaining();
        let code = Code {
            locals: self.read_locals_vector(),
            expression: self.read_expression().0
        };
        if self.remaining() + n != remaining {
            panic!("Invalid code")
        }

        return code;
    }

    pub fn read_code_section(&mut self) -> Vec<Code> {
        let mut vec: Vec<Code> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_code());
        }

        return vec;
    }

    pub fn read_data_section(&mut self) -> Vec<Data> {
        let mut vec: Vec<Data> = Vec::new();
        let size = self.read_var_u32();

        for i in 0..size {
            vec.push(self.read_data_instance());
        }

        return vec;
    }

    pub fn read_data_instance(&mut self) -> Data {
        return Data {
            memory: self.read_var_u32(),
            offset: self.read_expression().0,
            default: self.read_bytes()
        };
    }

    pub fn read_module(&mut self) -> Result<Module, String> {
        let magic = self.read_u32();
        if magic != 0x6D736100 { // magic number: '\0asm'
            return Err("The magic header isn't matched.".to_string());
        }
        let version = self.read_u32();
        if version != 1 {
            return Err("The version isn't matched.".to_ascii_lowercase());
        }

        let mut custom_secs: Vec<CustomSection> = Vec::new();
        let mut type_secs: Vec<FunctionType> = Vec::new();
        let mut import_secs: Vec<Import> = Vec::new();
        let mut function_secs: Vec<u32> = Vec::new();
        let mut table_secs: Vec<TableType> = Vec::new();
        let mut memory_secs: Vec<MemoryType> = Vec::new();
        let mut global_secs: Vec<Global> = Vec::new();
        let mut export_secs: Vec<Export> = Vec::new();
        let mut start_sec: u32 = 0;
        let mut element_secs: Vec<Element> = Vec::new();
        let mut code_secs: Vec<Code> = Vec::new();
        let mut data_secs: Vec<Data> = Vec::new();

        // read the sections
        let mut prev_sec_id = 0 as u8;
        while self.remaining() > 0 {
            let sec_id = self.read_u8();
            if sec_id == SECTION_CUSTOM_ID {
                custom_secs.push(self.read_custom_section());
                continue;
            }

            if sec_id > SECTION_DATA_ID {
                return Err("Malformed Section ID".to_string());
            }
            if sec_id <= prev_sec_id {
                return Err("Unexpected rewind after the last section".to_string());
            }
            prev_sec_id = sec_id;

            let n = self.read_var_u32() as usize;
            let remaining = self.remaining(); // used for verifying its validity
            match sec_id {
                SECTION_TYPE_ID => {
                    type_secs = self.read_type_section();
                }
                SECTION_IMPORT_ID => {
                    import_secs = self.read_import_section();
                }
                SECTION_FUNCTION_ID => {
                    function_secs = self.read_indices();
                }
                SECTION_TABLE_ID => {
                    table_secs = self.read_table_section();
                }
                SECTION_MEMORY_ID => {
                    memory_secs = self.read_memory_section();
                }
                SECTION_GLOBAL_ID => {
                    global_secs = self.read_global_section();
                }
                SECTION_EXPORT_ID => {
                    export_secs = self.read_export_section();
                }
                SECTION_START_ID => {
                    start_sec = self.read_start_section();
                }
                SECTION_ELEMENT_ID => {
                    element_secs = self.read_element_section();
                }
                SECTION_CODE_ID => {
                    code_secs = self.read_code_section();
                }
                SECTION_DATA_ID => {
                    data_secs = self.read_data_section();
                }

                _ => {
                    return Err("Unknown section type".to_string());
                }
            }
            if self.remaining() + n != remaining {
                return Err("Mismatched section size".to_string());
            }
        }

        let module = Module {
            version,
            custom_sections: custom_secs,
            type_section: type_secs,
            import_section: import_secs,
            func_section: function_secs,
            table_section: table_secs,
            memory_section: memory_secs,
            global_section: global_secs,
            export_section: export_secs,
            start_section: start_sec,
            element_section: element_secs,
            code_section: code_secs,
            data_section: data_secs
        };
        return Ok(module);
    }
}