use cubecl_common::ExecutionMode;
use cubecl_core::{
    Compiler, WgpuCompilationOptions,
    prelude::{CompiledKernel, KernelDefinition},
    server::ComputeServer,
};
use derive_new::new;

mod mlir;
pub mod module;

use module::MlirRepresentation;

#[derive(new, Debug, Clone, Copy)]
pub struct MlirCompiler;

#[derive(Clone, Debug, Default)]
pub struct MlirCompilationOptions {
    pub opt_level: i32,
}

impl Compiler for MlirCompiler {
    type Representation = MlirRepresentation;

    type CompilationOptions = MlirCompilationOptions;

    fn compile(
        &mut self,
        kernel: KernelDefinition,
        compilation_options: &Self::CompilationOptions,
        mode: ExecutionMode,
    ) -> Self::Representation {
        println!("Compiling Kernel");

        println!("Kernel structure: {:?}", kernel.body);

        mlir::compile_kernel(&kernel);

        todo!()
    }

    fn elem_size(&self, elem: cubecl_core::ir::Elem) -> usize {
        todo!()
    }

    fn extension(&self) -> &'static str {
        todo!()
    }
}
