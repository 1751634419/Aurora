use std::fs;
use aurora_bytecode::wasm_parser::WasmReader;
use aurora_core::vm::{VirtualMachine, OperandStack, Memory, ControlStack, GlobalTable, GlobalVariable};
use aurora_core::module::MemoryType;
use std::rc::Rc;
use std::cell::Cell;
use aurora_core::instr::inst::{Expression, Instruction};
use aurora_core::instr::inst_control::CallInst;

fn init_start(vm: &mut VirtualMachine) {
    let mut insts: Vec<Box<dyn Instruction>> = Vec::new();
    insts.push(Box::new(CallInst { func_index: vm.module.start_section  }));
    let expr: Expression = Rc::new(insts);
    vm.execute_expr(&expr);
}

fn init_memory(vm: &mut VirtualMachine) {
    let data_sec = &vm.module.clone().data_section;
    for i in 0..data_sec.len() {
        vm.execute_expr(&data_sec[i].offset);
        let offset = vm.operand_stack.pop_u64();
        vm.memory.write_vec(offset, &data_sec[i].default);
    }
}

fn init_globals(vm: &mut VirtualMachine) {
    let global_sec = &vm.module.clone().global_section;

    for i in 0..global_sec.len() {
        vm.execute_expr(&global_sec[i].default);
        let val = vm.operand_stack.pop_u64();

        vm.global_table.push_global(Rc::new(GlobalVariable {
            global_type: global_sec[i].global_type.clone(),
            val: Cell::new(val)
        }));
    }
}

fn main() {
    let result = fs::read("E:\\wat\\test8\\ch08_fac.wasm");
    if result.is_err() {
        panic!("Error dealing with the file reading")
    }

    let data = result.unwrap();
    let mut reader = WasmReader::new(data);
    let result = reader.read_module();
    match result {
        Ok(module) => {
            let mut vm = VirtualMachine {
                memory: {
                    if module.memory_section.len() != 0{
                        Memory::new_memory(MemoryType {
                            tag: module.memory_section[0].tag.clone(),
                            min: module.memory_section[0].min.clone(),
                            max: module.memory_section[0].max.clone()
                        })
                    } else {
                        Memory::new_memory(MemoryType {
                            tag: 0,
                            min: 0,
                            max: None
                        })
                    }
                },
                module: Rc::new(module),
                operand_stack: OperandStack::new_operand_stack(),
                control_stack: ControlStack::new_control_stack(),
                global_table: GlobalTable::new_global_type()
            };

            init_memory(&mut vm);
            init_globals(&mut vm);
            init_start(&mut vm);

            vm.execute();
        }
        Err(msg) => {
            print!("{}", msg);
        }
    }
}