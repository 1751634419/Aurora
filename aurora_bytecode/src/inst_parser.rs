use crate::wasm_parser::WasmReader;
use aurora_core::instr::inst::{Expression, BlockArguments, IfArguments, BrTableArguments, MemoryArguments};
use aurora_core::instr::inst::Instruction;
use aurora_core::instr::inst_control::*;
use aurora_core::module::BlockType;
use aurora_core::instr::inst_parametric::{DropInst, SelectInst};
use aurora_core::instr::inst_variable::{LocalGetInst, LocalSetInst, LocalTeeInst, GlobalGetInst, GlobalSetInst};
use aurora_core::instr::inst_memory::{I32LoadInst, I64LoadInst, F32LoadInst, F64LoadInst, I32Load8SInst, I32Load8UInst, I32Load16SInst, I32Load16UInst, I64Load8SInst, I64Load8UInst, I64Load16SInst, I64Load16UInst, I64Load32SInst, I64Load32UInst, I32StoreInst, I64StoreInst, F32StoreInst, F64StoreInst, I32Store8Inst, I32Store16Inst, I64Store8Inst, I64Store16Inst, I64Store32Inst, MemorySizeInst, MemoryGrowInst};
use aurora_core::instr::inst_numeric::*;
use std::any::Any;
use std::rc::Rc;

// WASM BYTECODE
const UNREACHABLE: u8 = 0x00;
const NOP: u8 = 0x01;
const BLOCK: u8 = 0x02; // BlockArgs
const LOOP: u8 = 0x03; // BlockArgs
const IF: u8 = 0x04; // IfArgs
const ELSE_: u8 = 0x05;
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
                BLOCK_TYPE_I32 | BLOCK_TYPE_I64 | BLOCK_TYPE_F32 | BLOCK_TYPE_F64 | BLOCK_TYPE_EMPTY => {
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
    
    pub fn read_memory_arguments(&mut self) -> MemoryArguments {
        return MemoryArguments {
            align: self.read_var_u32(),
            offset: self.read_var_u32()
        }
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

            LOCAL_GET => {
                return (Box::new(LocalGetInst { local_index: self.read_var_u32() }), LOCAL_GET);
            }

            LOCAL_SET => {
                return (Box::new(LocalSetInst { local_index: self.read_var_u32() }), LOCAL_SET);
            }

            LOCAL_TEE => {
                return (Box::new(LocalTeeInst { local_index: self.read_var_u32() }), LOCAL_TEE);
            }

            GLOBAL_GET => {
                return (Box::new(GlobalGetInst { global_index: self.read_var_u32() }), GLOBAL_GET)
            }

            GLOBAL_SET => {
                return (Box::new(GlobalSetInst { global_index: self.read_var_u32() }), GLOBAL_SET)
            }

            I32_LOAD => {
                return (Box::new(I32LoadInst { memory_arguments: self.read_memory_arguments() }), I32_LOAD);
            }

            I64_LOAD => {
                return (Box::new(I64LoadInst { memory_arguments: self.read_memory_arguments() }), I64_LOAD);
            }

            F32_LOAD => {
                return (Box::new(F32LoadInst { memory_arguments: self.read_memory_arguments() }), F32_LOAD);
            }

            F64_LOAD => {
                return (Box::new(F64LoadInst { memory_arguments: self.read_memory_arguments() }), F64_LOAD);
            }

            I32_LOAD8S => {
                return (Box::new(I32Load8SInst { memory_arguments: self.read_memory_arguments() }), I32_LOAD8S);
            }

            I32_LOAD8U => {
                return (Box::new(I32Load8UInst { memory_arguments: self.read_memory_arguments() }), I32_LOAD8U);
            }

            I32_LOAD16S => {
                return (Box::new(I32Load16SInst { memory_arguments: self.read_memory_arguments() }), I32_LOAD16S);
            }

            I32_LOAD16U => {
                return (Box::new(I32Load16UInst { memory_arguments: self.read_memory_arguments() }), I32_LOAD16U);
            }

            I64_LOAD8S => {
                return (Box::new(I64Load8SInst { memory_arguments: self.read_memory_arguments() }), I64_LOAD8S);
            }

            I64_LOAD8U => {
                return (Box::new(I64Load8UInst { memory_arguments: self.read_memory_arguments() }), I64_LOAD8U);
            }

            I64_LOAD16S => {
                return (Box::new(I64Load16SInst { memory_arguments: self.read_memory_arguments() }), I64_LOAD16S);
            }

            I64_LOAD16U => {
                return (Box::new(I64Load16UInst { memory_arguments: self.read_memory_arguments() }), I64_LOAD16U);
            }

            I64_LOAD32S => {
                return (Box::new(I64Load32SInst { memory_arguments: self.read_memory_arguments() }), I64_LOAD32S);
            }

            I64_LOAD32U => {
                return (Box::new(I64Load32UInst { memory_arguments: self.read_memory_arguments() }), I64_LOAD32U);
            }

            I32_STORE => {
                return (Box::new(I32StoreInst { memory_arguments: self.read_memory_arguments() }), I32_STORE);
            }

            I64_STORE => {
                return (Box::new(I64StoreInst { memory_arguments: self.read_memory_arguments() }), I64_STORE);
            }

            F32_STORE => {
                return (Box::new(F32StoreInst { memory_arguments: self.read_memory_arguments() }), F32_STORE);
            }

            F64_STORE => {
                return (Box::new(F64StoreInst { memory_arguments: self.read_memory_arguments() }), F64_STORE);
            }

            I32_STORE8 => {
                return (Box::new(I32Store8Inst { memory_arguments: self.read_memory_arguments() }), I32_STORE8);
            }

            I32_STORE16 => {
                return (Box::new(I32Store16Inst { memory_arguments: self.read_memory_arguments() }), I32_STORE16);
            }

            I64_STORE8 => {
                return (Box::new(I64Store8Inst { memory_arguments: self.read_memory_arguments() }), I64_STORE8);
            }

            I64_STORE16 => {
                return (Box::new(I64Store16Inst { memory_arguments: self.read_memory_arguments() }), I64_STORE16);
            }

            I64_STORE32 => {
                return (Box::new(I64Store32Inst { memory_arguments: self.read_memory_arguments() }), I64_STORE32);
            }

            MEMORY_SIZE => {
                self.read_zero();
                return (Box::new(MemorySizeInst {}), MEMORY_SIZE);
            }

            MEMORY_GROW => {
                self.read_zero();
                return (Box::new(MemoryGrowInst {}), MEMORY_GROW);
            }

            I32_CONST => {
                return (Box::new(I32ConstInst { val: self.read_var_i32() }), I32_CONST);
            }

            I64_CONST => {
                return (Box::new(I64ConstInst { val: self.read_var_i64() }), I64_CONST);
            }

            F32_CONST => {
                return (Box::new(F32ConstInst { val: self.read_f32() }), F32_CONST);
            }

            F64_CONST => {
                return (Box::new(F64ConstInst { val: self.read_f64() }), F64_CONST);
            }

            I32_EQZ => {
                return (Box::new(I32EqzInst {}), I32_EQZ);
            }

            I32_EQ => {
                return (Box::new(I32EqInst {}), I32_EQ);
            }

            I32_NE => {
                return (Box::new(I32NeInst {}), I32_NE);
            }

            I32_LTS => {
                return (Box::new(I32LtsInst {}), I32_LTS);
            }

            I32_LTU => {
                return (Box::new(I32LtuInst {}), I32_LTU);
            }

            I32_GTS => {
                return (Box::new(I32GtsInst {}), I32_GTS);
            }

            I32_GTU => {
                return (Box::new(I32GtuInst {}), I32_GTU);
            }

            I32_LES => {
                return (Box::new(I32LesInst {}), I32_LES);
            }

            I32_LEU => {
                return (Box::new(I32LeuInst {}), I32_LEU);
            }

            I32_GES => {
                return (Box::new(I32GesInst {}), I32_GES);
            }

            I32_GEU => {
                return (Box::new(I32GeuInst {}), I32_GEU);
            }

            I64_EQZ => {
                return (Box::new(I64EqzInst {}), I64_EQZ);
            }

            I64_EQ => {
                return (Box::new(I64EqInst {}), I64_EQ);
            }

            I64_NE => {
                return (Box::new(I64NeInst {}), I64_NE);
            }

            I64_LTS => {
                return (Box::new(I64LtsInst {}), I64_LTS);
            }

            I64_LTU => {
                return (Box::new(I64LtuInst {}), I64_LTU);
            }

            I64_GTS => {
                return (Box::new(I64GtsInst {}), I64_GTS);
            }

            I64_GTU => {
                return (Box::new(I64GtuInst {}), I64_GTU);
            }

            I64_LES => {
                return (Box::new(I64LesInst {}), I64_LES);
            }

            I64_LEU => {
                return (Box::new(I64LeuInst {}), I64_LEU);
            }

            I64_GES => {
                return (Box::new(I64GesInst {}), I64_GES);
            }

            I64_GEU => {
                return (Box::new(I64GeuInst {}), I64_GEU);
            }

            F32_EQ => {
                return (Box::new(F32EqInst {}), F32_EQ);
            }

            F32_NE => {
                return (Box::new(F32NeInst {}), F32_NE);
            }

            F32_LT => {
                return (Box::new(F32LtInst {}), F32_LT);
            }

            F32_GT => {
                return (Box::new(F32GtInst {}), F32_GT);
            }

            F32_LE => {
                return (Box::new(F32LeInst {}), F32_LE);
            }

            F32_GE => {
                return (Box::new(F32GeInst {}), F32_GE);
            }

            F64_EQ => {
                return (Box::new(F64EqInst {}), F64_EQ);
            }

            F64_NE => {
                return (Box::new(F64NeInst {}), F64_NE);
            }

            F64_LT => {
                return (Box::new(F64LtInst {}), F64_LT);
            }

            F64_GT => {
                return (Box::new(F64GtInst {}), F64_GT);
            }

            F64_LE => {
                return (Box::new(F64LeInst {}), F64_LE);
            }

            F64_GE => {
                return (Box::new(F64GeInst {}), F64_GE);
            }

            I32_CLZ => {
                return (Box::new(I32ClzInst {}), I32_CLZ);
            }

            I32_CTZ => {
                return (Box::new(I32CtzInst {}), I32_CTZ);
            }

            I32_POPCNT => {
                return (Box::new(I32PopCntInst {}), I32_POPCNT);
            }

            I32_ADD => {
                return (Box::new(I32AddInst {}), I32_ADD);
            }

            I32_SUB => {
                return (Box::new(I32SubInst {}), I32_SUB);
            }

            I32_MUL => {
                return (Box::new(I32MulInst {}), I32_MUL);
            }

            I32_DIVS => {
                return (Box::new(I32DivsInst {}), I32_DIVS);
            }

            I32_DIVU => {
                return (Box::new(I32DivuInst {}), I32_DIVU);
            }

            I32_REMS => {
                return (Box::new(I32RemsInst {}), I32_REMS);
            }

            I32_REMU => {
                return (Box::new(I32RemuInst {}), I32_REMU);
            }

            I32_AND => {
                return (Box::new(I32AndInst {}), I32_AND);
            }

            I32_OR => {
                return (Box::new(I32OrInst {}), I32_OR);
            }

            I32_XOR => {
                return (Box::new(I32XorInst {}), I32_XOR);
            }

            I32_SHL => {
                return (Box::new(I32ShlInst {}), I32_SHL);
            }

            I32_SHRS => {
                return (Box::new(I32ShrsInst {}), I32_SHRS);
            }

            I32_SHRU => {
                return (Box::new(I32ShruInst {}), I32_SHRU);
            }

            I32_ROTL => {
                return (Box::new(I32RotlInst {}), I32_ROTL);
            }

            I32_ROTR => {
                return (Box::new(I32RotrInst {}), I32_ROTR);
            }

            I64_CLZ => {
                return (Box::new(I64ClzInst {}), I64_CLZ);
            }

            I64_CTZ => {
                return (Box::new(I64CtzInst {}), I64_CTZ);
            }

            I64_POPCNT => {
                return (Box::new(I64PopCntInst {}), I64_POPCNT);
            }

            I64_ADD => {
                return (Box::new(I64AddInst {}), I64_ADD);
            }

            I64_SUB => {
                return (Box::new(I64SubInst {}), I64_SUB);
            }

            I64_MUL => {
                return (Box::new(I64MulInst {}), I64_MUL);
            }

            I64_DIVS => {
                return (Box::new(I64DivsInst {}), I64_DIVS);
            }

            I64_DIVU => {
                return (Box::new(I64DivuInst {}), I64_DIVU);
            }

            I64_REMS => {
                return (Box::new(I64RemsInst {}), I64_REMS);
            }

            I64_REMU => {
                return (Box::new(I64RemuInst {}), I64_REMU);
            }

            I64_AND => {
                return (Box::new(I64AndInst {}), I64_AND);
            }

            I64_OR => {
                return (Box::new(I64OrInst {}), I64_OR);
            }

            I64_XOR => {
                return (Box::new(I64XorInst {}), I64_XOR);
            }

            I64_SHL => {
                return (Box::new(I64ShlInst {}), I64_SHL);
            }

            I64_SHRS => {
                return (Box::new(I64ShrsInst {}), I64_SHRS);
            }

            I64_SHRU => {
                return (Box::new(I64ShruInst {}), I64_SHRU);
            }

            I64_ROTL => {
                return (Box::new(I64RotlInst {}), I64_ROTL);
            }

            I64_ROTR => {
                return (Box::new(I64RotrInst {}), I64_ROTR);
            }

            F32_ABS => {
                return (Box::new(F32AbsInst {}), F32_ABS);
            }

            F32_NEG => {
                return (Box::new(F32NegInst {}), F32_NEG);
            }

            F32_CEIL => {
                return (Box::new(F32CeilInst {}), F32_CEIL);
            }

            F32_FLOOR => {
                return (Box::new(F32FloorInst {}), F32_FLOOR);
            }

            F32_TRUNC => {
                return (Box::new(F32TruncInst {}), F32_TRUNC);
            }

            F32_NEAREST => {
                return (Box::new(F32NearestInst {}), F32_NEAREST);
            }

            F32_SQRT => {
                return (Box::new(F32SqrtInst {}), F32_SQRT);
            }

            F32_ADD => {
                return (Box::new(F32AddInst {}), F32_ADD);
            }

            F32_SUB => {
                return (Box::new(F32SubInst {}), F32_SUB);
            }

            F32_MUL => {
                return (Box::new(F32MulInst {}), F32_MUL);
            }

            F32_DIV => {
                return (Box::new(F32DivInst {}), F32_DIV);
            }

            F32_MIN => {
                return (Box::new(F32MinInst {}), F32_MIN);
            }

            F32_MAX => {
                return (Box::new(F32MaxInst {}), F32_MAX);
            }

            F32_COPY_SIGN => {
                return (Box::new(F32CopySignInst {}), F32_COPY_SIGN);
            }

            F64_ABS => {
                return (Box::new(F64AbsInst {}), F64_ABS);
            }

            F64_NEG => {
                return (Box::new(F64NegInst {}), F64_NEG);
            }

            F64_CEIL => {
                return (Box::new(F64CeilInst {}), F64_CEIL);
            }

            F64_FLOOR => {
                return (Box::new(F64FloorInst {}), F64_FLOOR);
            }

            F64_TRUNC => {
                return (Box::new(F64TruncInst {}), F64_TRUNC);
            }

            F64_NEAREST => {
                return (Box::new(F64NearestInst {}), F64_NEAREST);
            }

            F64_SQRT => {
                return (Box::new(F64SqrtInst {}), F64_SQRT);
            }

            F64_ADD => {
                return (Box::new(F64AddInst {}), F64_ADD);
            }

            F64_SUB => {
                return (Box::new(F64SubInst {}), F64_SUB);
            }

            F64_MUL => {
                return (Box::new(F64MulInst {}), F64_MUL);
            }

            F64_DIV => {
                return (Box::new(F64DivInst {}), F64_DIV);
            }

            F64_MIN => {
                return (Box::new(F64MinInst {}), F64_MIN);
            }

            F64_MAX => {
                return (Box::new(F64MaxInst {}), F64_MAX);
            }

            F64_COPY_SIGN => {
                return (Box::new(F64CopySignInst {}), F64_COPY_SIGN);
            }

            I32_WRAP_I64 => {
                return (Box::new(I32WrapI64Inst {}), I32_WRAP_I64);
            }

            I32_TRUNC_F32S => {
                return (Box::new(I32TruncF32SInst {}), I32_TRUNC_F32S);
            }

            I32_TRUNC_F32U => {
                return (Box::new(I32TruncF32UInst {}), I32_TRUNC_F32U);
            }

            I32_TRUNC_F64S => {
                return (Box::new(I32TruncF64SInst {}), I32_TRUNC_F64S);
            }

            I32_TRUNC_F64U => {
                return (Box::new(I32TruncF64UInst {}), I32_TRUNC_F64U);
            }

            I64_EXTEND_I32S => {
                return (Box::new(I64ExtendI32SInst {}), I64_EXTEND_I32S);
            }

            I64_EXTEND_I32U => {
                return (Box::new(I64ExtendI32UInst {}), I64_EXTEND_I32U);
            }

            I64_TRUNC_F32S => {
                return (Box::new(I64TruncF32SInst {}), I64_TRUNC_F32S);
            }

            I64_TRUNC_F32U => {
                return (Box::new(I64TruncF32UInst {}), I64_TRUNC_F32U);
            }

            I64_TRUNC_F64S => {
                return (Box::new(I64TruncF64SInst {}), I64_TRUNC_F64S);
            }

            I64_TRUNC_F64U => {
                return (Box::new(I64TruncF64UInst {}), I64_TRUNC_F64U);
            }

            F32_CONVERT_I32S => {
                return (Box::new(F32ConvertI32SInst {}), F32_CONVERT_I32S);
            }

            F32_CONVERT_I32U => {
                return (Box::new(F32ConvertI32UInst {}), F32_CONVERT_I32U);
            }

            F32_CONVERT_I64S => {
                return (Box::new(F32ConvertI64SInst {}), F32_CONVERT_I64S);
            }

            F32_CONVERT_I64U => {
                return (Box::new(F32ConvertI64UInst {}), F32_CONVERT_I64U);
            }

            F32_DEMOTE_F64 => {
                return (Box::new(F32DemoteF64Inst {}), F32_DEMOTE_F64);
            }

            F64_CONVERT_I32S => {
                return (Box::new(F64ConvertI32SInst {}), F64_CONVERT_I32S);
            }

            F64_CONVERT_I32U => {
                return (Box::new(F64ConvertI32UInst {}), F64_CONVERT_I32U);
            }

            F64_CONVERT_I64S => {
                return (Box::new(F64ConvertI64SInst {}), F64_CONVERT_I64S);
            }

            F64_CONVERT_I64U => {
                return (Box::new(F64ConvertI64UInst {}), F64_CONVERT_I64U);
            }

            F64_PROMOTE_F32 => {
                return (Box::new(F64PromoteF32Inst {}), F64_PROMOTE_F32);
            }

            I32_REINTERPRET_F32 => {
                return (Box::new(I32ReinterpretF32Inst {}), I32_REINTERPRET_F32);
            }

            I64_REINTERPRET_F64 => {
                return (Box::new(I64ReinterpretF64Inst {}), I64_REINTERPRET_F64);
            }

            F32_REINTERPRET_I32 => {
                return (Box::new(F32ReinterpretI32Inst {}), F32_REINTERPRET_I32);
            }

            F64_REINTERPRET_I64 => {
                return (Box::new(F64ReinterpretI64Inst {}), F64_REINTERPRET_I64);
            }

            I32_EXTEND_8S => {
                return (Box::new(I32Extend8SInst {}), I32_EXTEND_8S);
            }

            I32_EXTEND_16S => {
                return (Box::new(I32Extend16SInst {}), I32_EXTEND_16S);
            }

            I64_EXTEND_8S => {
                return (Box::new(I64Extend8SInst {}), I64_EXTEND_8S);
            }

            I64_EXTEND_16S => {
                return (Box::new(I64Extend16SInst {}), I64_EXTEND_16S);
            }

            I64_EXTEND_32S => {
                return (Box::new(I64Extend32SInst {}), I64_EXTEND_32S);
            }

            TRUNC_SAT => {
                return (Box::new(TruncSatInst { operand: self.read_u8() }), TRUNC_SAT);
            }

            _ => {
                panic!("unrecognized instruction");
            }
        }
    }

    pub fn read_expression(&mut self) -> (Expression, u8) {
        let mut vec: Vec<Box<dyn Instruction>> = Vec::new();

        let end_mark;
        loop {
            let (inst, bc) = self.read_instruction();

            if bc == ELSE_ || bc == END_ {
                end_mark = bc;
                break;
            }

            vec.push(inst);
        }

        return (Rc::new(vec), end_mark);
    }
}