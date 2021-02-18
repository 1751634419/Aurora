pub mod wasm_parser;
pub mod module;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::wasm_parser::WasmReader;

    #[test]
    fn it_works() {
        let result = fs::read("F:\\Test\\a.out.wasm");
        if result.is_err() {
            panic!("Error dealing with the file reading")
        }

        let data = result.unwrap();
        let mut reader = WasmReader::new(data);
        let result = reader.read_module();
        match result {
            Ok(module) => {}
            Err(msg) => {
                print!("{}", msg);
            }
        }
    }
}
