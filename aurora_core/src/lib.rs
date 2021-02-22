pub mod module;
pub mod vm;
pub mod instr;
pub mod interp;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
