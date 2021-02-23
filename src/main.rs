use std::fs;
use aurora_bytecode::wasm_parser::WasmReader;
use aurora_core::vm::{VirtualMachine, OperandStack, Memory};
use aurora_core::module::MemoryType;
use std::rc::Rc;

fn init_memory(vm: &mut VirtualMachine) {
    let data_sec = &vm.module.clone().data_section;
    for i in 0..data_sec.len() {
        vm.exec_const_expr(&data_sec[i].offset);
        let offset = vm.operand_stack.pop_u64().unwrap();
        vm.memory.write_vec(offset, &data_sec[i].default);
    }
}

fn main() {
    let result = fs::read("E:\\wat\\test6\\ch06_mem.wasm");
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
            };

            init_memory(&mut vm);

            let si = vm.module.start_section.clone() as usize;
            let isl = vm.module.import_section.len();
            vm.execute(si - isl);
        }
        Err(msg) => {
            print!("{}", msg);
        }
    }
}