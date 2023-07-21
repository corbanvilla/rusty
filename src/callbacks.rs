use crate::codegen::llvm_index::LlvmTypedIndex;

// TODO - move the lexer callback here at some point is probably a good idea

pub trait LLVMDataTypeCallback: Send {
    fn on_parse_datatype(&mut self, llvm_datatype: LlvmTypedIndex);
}
