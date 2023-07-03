use inkwell::context::Context;

pub struct LLVMGen<'ctx> {
    pub ctx: &'ctx Context,
}

pub trait WriteLLVM {
    fn write_llvm(&self, llvm_gen: &mut LLVMGen) -> String;
}
