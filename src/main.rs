use std::fs;
use aurora_bytecode::wasm_parser::WasmReader;
use aurora_core::vm::{VirtualMachine, OperandStack};

fn main() {
    let result = fs::read("E:\\wat\\test5\\ch05_num.wasm");
    if result.is_err() {
        panic!("Error dealing with the file reading")
    }

    let data = result.unwrap();
    let mut reader = WasmReader::new(data);
    let result = reader.read_module();
    match result {
        Ok(module) => {
            let mut vm = VirtualMachine {
                module,
                operand_stack: OperandStack::new_operand_stack(4 * 1024 * 1024)
            };
            let si = vm.module.start_section.clone() as usize;
            let isl = vm.module.import_section.len();
            vm.execute(si - isl);
        }
        Err(msg) => {
            print!("{}", msg);
        }
    }
}