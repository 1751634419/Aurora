use crate::wasm_parser::WasmReader;
use aurora_core::instr::inst::{Expression, BlockArguments, IfArguments, BrTableArguments};
use aurora_core::instr::inst::Instruction;
use aurora_core::instr::inst_control::{UnreachableInst, NopInst, BlockInst, LoopInst, IfInst, BrInst, BrIfInst, BrTableInst, ReturnInst, CallInst, CallIndirectInst};
use aurora_core::module::BlockType;
use aurora_core::instr::inst_parametric::{DropInst, SelectInst};

// WASM BYTECODE
const UNREACHABLE: u8 = 0x00;
const NOP: u8 = 0x01;
const BLOCK: u8 = 0x02; // BlockArgs
const LOOP: u8 = 0x03; // BlockArgs
const IF: u8 = 0x04; // IfArgs
const ELSE_: u8 = 05;
const END_: u8 = 0x0B;
const BR: u8 = 0x0C; // label_idx(VarU32)
const BR_IF: u8 = 0x0D; // label_idx(VarU32)
const BR_TABLE: u8 = 0x0E; // BrTableArgs
const RETURN: u8 = 0x0F;
const CALL: u8 = 0x10; // func_idx(VarU32)
const CALL_INDIRECT: u8 = 0x11; // CallIndirectArgs
const DROP: u8 = 0x1A;
const SELECT: u8 = 0x1B;
const LOCAL_GET: u8 = 0x20; // local_idx(VarU32)
const LOCAL_SET: u8 = 0x21; // local_idx(VarU32)
const LOCAL_TEE: u8 = 0x22; // local_idx(VarU32)
const GLOBAL_GET: u8 = 0x23; // global_idx(VarU32)
const GLOBAL_SET: u8 = 0x24; // global_idx(VarU32)
const I32_LOAD: u8 = 0x28; // START_MEMORY_ARGS
const I64_LOAD: u8 = 0x29;
const F32_LOAD: u8 = 0x2A;
const F64_LOAD: u8 = 0x2B;
const I32_LOAD8S: u8 = 0x2C;
const I32_LOAD8U: u8 = 0x2D;
const I32_LOAD16S: u8 = 0x2E;
const I32_LOAD16U: u8 = 0x2F;
const I64_LOAD8S: u8 = 0x30;
const I64_LOAD8U: u8 = 0x31;
const I64_LOAD16S: u8 = 0x32;
const I64_LOAD16U: u8 = 0x33;
const I64_LOAD32S: u8 = 0x34;
const I64_LOAD32U: u8 = 0x35;
const I32_STORE: u8 = 0x36;
const I64_STORE: u8 = 0x37;
const F32_STORE: u8 = 0x38;
const F64_STORE: u8 = 0x39;
const I32_STORE8: u8 = 0x3A;
const I32_STORE16: u8 = 0x3B;
const I64_STORE8: u8 = 0x3C;
const I64_STORE16: u8 = 0x3D;
const I64_STORE32: u8 = 0x3E; // END_MEMORY_ARGS
const MEMORY_SIZE: u8 = 0x3F; // zero
const MEMORY_GROW: u8 = 0x40; // zero
const I32_CONST: u8 = 0x41; // Var_S32
const I64_CONST: u8 = 0x42; // Var_S64
const F32_CONST: u8 = 0x43; // F32
const F64_CONST: u8 = 0x44; // F64
const I32_EQZ: u8 = 0x45;
const I32_EQ: u8 = 0x46;
const I32_NE: u8 = 0x47;
const I32_LTS: u8 = 0x48;
const I32_LTU: u8 = 0x49;
const I32_GTS: u8 = 0x4A;
const I32_GTU: u8 = 0x4B;
const I32_LES: u8 = 0x4C;
const I32_LEU: u8 = 0x4D;
const I32_GES: u8 = 0x4E;
const I32_GEU: u8 = 0x4F;
const I64_EQZ: u8 = 0x50;
const I64_EQ: u8 = 0x51;
const I64_NE: u8 = 0x52;
const I64_LTS: u8 = 0x53;
const I64_LTU: u8 = 0x54;
const I64_GTS: u8 = 0x55;
const I64_GTU: u8 = 0x56;
const I64_LES: u8 = 0x57;
const I64_LEU: u8 = 0x58;
const I64_GES: u8 = 0x59;
const I64_GEU: u8 = 0x5A;
const F32_EQ: u8 = 0x5B;
const F32_NE: u8 = 0x5C;
const F32_LT: u8 = 0x5D;
const F32_GT: u8 = 0x5E;
const F32_LE: u8 = 0x5F;
const F32_GE: u8 = 0x60;
const F64_EQ: u8 = 0x61;
const F64_NE: u8 = 0x62;
const F64_LT: u8 = 0x63;
const F64_GT: u8 = 0x64;
const F64_LE: u8 = 0x65;
const F64_GE: u8 = 0x66;
const I32_CLZ: u8 = 0x67;
const I32_CTZ: u8 = 0x68;
const I32_POPCNT: u8 = 0x69;
const I32_ADD: u8 = 0x6A;
const I32_SUB: u8 = 0x6B;
const I32_MUL: u8 = 0x6C;
const I32_DIVS: u8 = 0x6D;
const I32_DIVU: u8 = 0x6E;
const I32_REMS: u8 = 0x6F;
const I32_REMU: u8 = 0x70;
const I32_AND: u8 = 0x71;
const I32_OR: u8 = 0x72;
const I32_XOR: u8 = 0x73;
const I32_SHL: u8 = 0x74;
const I32_SHRS: u8 = 0x75;
const I32_SHRU: u8 = 0x76;
const I32_ROTL: u8 = 0x77;
const I32_ROTR: u8 = 0x78;
const I64_CLZ: u8 = 0x79;
const I64_CTZ: u8 = 0x7A;
const I64_POPCNT: u8 = 0x7B;
const I64_ADD: u8 = 0x7C;
const I64_SUB: u8 = 0x7D;
const I64_MUL: u8 = 0x7E;
const I64_DIVS: u8 = 0x7F;
const I64_DIVU: u8 = 0x80;
const I64_REMS: u8 = 0x81;
const I64_REMU: u8 = 0x82;
const I64_AND: u8 = 0x83;
const I64_OR: u8 = 0x84;
const I64_XOR: u8 = 0x85;
const I64_SHL: u8 = 0x86;
const I64_SHRS: u8 = 0x87;
const I64_SHRU: u8 = 0x88;
const I64_ROTL: u8 = 0x89;
const I64_ROTR: u8 = 0x8A;
const F32_ABS: u8 = 0x8B;
const F32_NEG: u8 = 0x8C;
const F32_CEIL: u8 = 0x8D;
const F32_FLOOR: u8 = 0x8E;
const F32_TRUNC: u8 = 0x8F;
const F32_NEAREST: u8 = 0x90;
const F32_SQRT: u8 = 0x91;
const F32_ADD: u8 = 0x92;
const F32_SUB: u8 = 0x93;
const F32_MUL: u8 = 0x94;
const F32_DIV: u8 = 0x95;
const F32_MIN: u8 = 0x96;
const F32_MAX: u8 = 0x97;
const F32_COPY_SIGN: u8 = 0x98;
const F64_ABS: u8 = 0x99;
const F64_NEG: u8 = 0x9A;
const F64_CEIL: u8 = 0x9B;
const F64_FLOOR: u8 = 0x9C;
const F64_TRUNC: u8 = 0x9D;
const F64_NEAREST: u8 = 0x9E;
const F64_SQRT: u8 = 0x9F;
const F64_ADD: u8 = 0xA0;
const F64_SUB: u8 = 0xA1;
const F64_MUL: u8 = 0xA2;
const F64_DIV: u8 = 0xA3;
const F64_MIN: u8 = 0xA4;
const F64_MAX: u8 = 0xA5;
const F64_COPY_SIGN: u8 = 0xA6;
const I32_WRAP_I64: u8 = 0xA7;
const I32_TRUNC_F32S: u8 = 0xA8;
const I32_TRUNC_F32U: u8 = 0xA9;
const I32_TRUNC_F64S: u8 = 0xAA;
const I32_TRUNC_F64U: u8 = 0xAB;
const I64_EXTEND_I32S: u8 = 0xAC;
const I64_EXTEND_I32U: u8 = 0xAD;
const I64_TRUNC_F32S: u8 = 0xAE;
const I64_TRUNC_F32U: u8 = 0xAF;
const I64_TRUNC_F64S: u8 = 0xB0;
const I64_TRUNC_F64U: u8 = 0xB1;
const F32_CONVERT_I32S: u8 = 0xB2;
const F32_CONVERT_I32U: u8 = 0xB3;
const F32_CONVERT_I64S: u8 = 0xB4;
const F32_CONVERT_I64U: u8 = 0xB5;
const F32_DEMOTE_F64: u8 = 0xB6;
const F64_CONVERT_I32S: u8 = 0xB7;
const F64_CONVERT_I32U: u8 = 0xB8;
const F64_CONVERT_I64S: u8 = 0xB9;
const F64_CONVERT_I64U: u8 = 0xBA;
const F64_PROMOTE_F32: u8 = 0xBB;
const I32_REINTERPRET_F32: u8 = 0xBC;
const I64_REINTERPRET_F64: u8 = 0xBD;
const F32_REINTERPRET_I32: u8 = 0xBE;
const F64_REINTERPRET_I64: u8 = 0xBF;
const I32_EXTEND_8S: u8 = 0xC0;
const I32_EXTEND_16S: u8 = 0xC1;
const I64_EXTEND_8S: u8 = 0xC2;
const I64_EXTEND_16S: u8 = 0xC3;
const I64_EXTEND_32S: u8 = 0xC4;
const TRUNC_SAT: u8 = 0xFC; // Byte

const BLOCK_TYPE_I32: i32 = -1;
const BLOCK_TYPE_I64: i32 = -2;
const BLOCK_TYPE_F32: i32 = -3;
const BLOCK_TYPE_F64: i32 = -4;
const BLOCK_TYPE_EMPTY: i32 = -64;
// instruction parser
impl WasmReader {
    pub fn read_block_type(&mut self) -> Result<BlockType, String> {
        let bt = self.read_var_i32();

        if bt < 0 {
            match bt {
                BLOCK_TYPE_I32 | BLOCK_TYPE_I64 | BLOCK_TYPE_F32 | BLOCK_TYPE_F64 => {
                    // nothing to do actually
                }

                _ => {
                    // you're dead meat
                    return Err("malformed block type".to_string());
                }
            }
        }

        return Ok(bt);
    }

    pub fn read_block_arguments(&mut self) -> BlockArguments {
        let block_type = self.read_block_type().unwrap();
        let (expr, end) = self.read_expression();
        if end != END_ {
            panic!("invalid block end");
        }

        return BlockArguments {
            block_type: block_type,
            insts: expr
        };
    }

    pub fn read_if_arguments(&mut self) -> IfArguments {
        let bt = self.read_block_type().unwrap();
        let (instrs1, end) = self.read_expression();

        if end == ELSE_ {
            let(instrs2, end) = self.read_expression();

            if end != END_ {
                panic!("invalid block end");
            }

            return IfArguments {
                block_type: bt,
                insts_1: instrs1,
                insts_2: Option::Some(instrs2)
            }
        }

        return IfArguments {
            block_type: bt,
            insts_1: instrs1,
            insts_2: None
        }
    }
    
    pub fn read_call_indirect_arguments(&mut self) -> u32 {
        let type_idx = self.read_var_u32();
        self.read_zero();
        return type_idx;
    }

    pub fn read_br_table_arguments(&mut self) -> BrTableArguments {
        return BrTableArguments {
            labels: self.read_indices(),
            default: self.read_var_u32()
        }
    }
    
    pub fn read_instruction(&mut self) -> (Box<dyn Instruction>, u8) {
        let b = self.read_u8();

        match b {
            UNREACHABLE => {
                return (Box::new(UnreachableInst {}), UNREACHABLE);
            }

            NOP => {
                return (Box::new(NopInst {}), NOP);
            }

            BLOCK => {
                return (Box::new(BlockInst { bt: self.read_block_arguments(), }), BLOCK)
            }

            LOOP => {
                return (Box::new(LoopInst { bt: self.read_block_arguments(), }), LOOP)
            }

            IF => {
                return (Box::new(IfInst { if_arguments: self.read_if_arguments() }), IF);
            }

            ELSE_ => {
                return (Box::new( NopInst {} /* placeholder */ ), ELSE_);
            }

            END_ => {
                return (Box::new( NopInst {} /* placeholder */ ), END_);
            }

            BR => {
                return (Box::new(BrInst { lable_index: self.read_var_u32() }), BR);
            }

            BR_IF => {
                return (Box::new(BrIfInst { lable_index: self.read_var_u32() }), BR_IF);
            }
            
            BR_TABLE => {
                return (Box::new(BrTableInst { br_table_arguments: self.read_br_table_arguments() }), BR_TABLE);
            }

            RETURN => {
                return (Box::new(ReturnInst {}), RETURN)
            }

            CALL => {
                return (Box::new(CallInst { func_index: self.read_var_u32() }), CALL)
            }
            
            CALL_INDIRECT => {
                return (Box::new(CallIndirectInst { type_idx: self.read_call_indirect_arguments() }), CALL_INDIRECT)
            }

            DROP => {
                return (Box::new(DropInst {}), DROP)
            }

            SELECT => {
                return (Box::new(SelectInst {}), SELECT)
            }

            // TODO NOT READY YET
            _ => {
                panic!("");
            }
        }
    }

    pub fn read_expression(&mut self) -> (Expression, u8) {
        let mut vec: Vec<Box<dyn Instruction>> = Vec::new();

        let mut end_mark;
        loop {
            let (inst, bc) = self.read_instruction();

            if bc == ELSE_ || bc == END_ {
                end_mark = bc;
                break;
            }

            vec.push(inst);
        }

        return (vec, end_mark);
    }
}