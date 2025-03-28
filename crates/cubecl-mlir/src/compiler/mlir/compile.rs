use cubecl_core::{ir::*, prelude::KernelDefinition};

pub struct MlirCompilerModule<'a, 'b> {
    registry: &'a melior::dialect::DialectRegistry,
    context: &'a melior::Context,
    location: &'a melior::ir::Location<'b>,
    module: &'a mut melior::ir::Module<'b>,
}

impl<'a, 'b> MlirCompilerModule<'a, 'b> {
    pub const fn new(
        registry: &'a melior::dialect::DialectRegistry,
        context: &'a melior::Context,
        location: &'a melior::ir::Location<'b>,
        module: &'a mut melior::ir::Module<'b>,
    ) -> Self {
        Self { registry, context, location, module }
    }
}

pub fn compile_kernel(kernel: &KernelDefinition) {
    let registry = melior::dialect::DialectRegistry::new();
    melior::utility::register_all_dialects(&registry);

    let context = melior::Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();

    let location = melior::ir::Location::unknown(&context);
    let mut module = melior::ir::Module::new(location);

    println!();
    println!("Inputs: {:?}", kernel.inputs);
    println!("Outputs: {:?}", kernel.outputs);

    compile_scope(
        &kernel.body,
        &MlirCompilerModule::new(&registry, &context, &location, &mut module),
    );

    todo!()
}

pub fn compile_scope(
    scope: &Scope,
    compiler_module: &MlirCompilerModule<'_, '_>,
) {
    for instruction in &scope.instructions {
        compile_instruction(instruction, compiler_module);
    }
}

pub fn compile_instruction(
    instruction: &Instruction,
    compiler_module: &MlirCompilerModule<'_, '_>,
) {
    match &instruction.operation {
        Operation::Copy(_variable) => todo!(),
        Operation::Arithmetic(_arithmetic) => todo!(),
        Operation::Comparison(_comparison) => todo!(),
        Operation::Bitwise(_bitwise) => todo!(),
        Operation::Operator(operator) => {
            compile_operator(operator, compiler_module);
        }
        Operation::Atomic(_atomic_op) => todo!(),
        Operation::Metadata(_metadata) => todo!(),
        Operation::Branch(_branch) => todo!(),
        Operation::Synchronization(_synchronization) => todo!(),
        Operation::Plane(_plane) => todo!(),
        Operation::CoopMma(_coop_mma) => todo!(),
        Operation::Pipeline(_pipeline_ops) => todo!(),
        Operation::Barrier(_barrier_ops) => todo!(),
        Operation::Tma(_tma_ops) => todo!(),
        Operation::NonSemantic(_non_semantic) => todo!(),
    }
}

pub fn compile_operator(
    operator: &Operator,
    compiler_module: &MlirCompilerModule<'_, '_>,
) {
    match operator {
        Operator::Index(binary_operator) => compile_index(
            &binary_operator.lhs,
            &binary_operator.rhs,
            compiler_module,
        ),
        Operator::CopyMemory(_copy_memory_operator) => todo!(),
        Operator::CopyMemoryBulk(_copy_memory_bulk_operator) => todo!(),
        Operator::Slice(_slice_operator) => todo!(),
        Operator::UncheckedIndex(_binary_operator) => todo!(),
        Operator::IndexAssign(_binary_operator) => todo!(),
        Operator::InitLine(_line_init_operator) => todo!(),
        Operator::UncheckedIndexAssign(_binary_operator) => todo!(),
        Operator::And(_binary_operator) => todo!(),
        Operator::Or(_binary_operator) => todo!(),
        Operator::Not(_unary_operator) => todo!(),
        Operator::Cast(_unary_operator) => todo!(),
        Operator::Bitcast(_unary_operator) => todo!(),
        Operator::Select(_select) => todo!(),
        Operator::ConditionalRead(_conditional_read) => todo!(),
    }
}

pub fn compile_index(
    arr: &Variable,
    index: &Variable,
    _compiler_module: &MlirCompilerModule<'_, '_>,
) {
    println!("Array: {arr:?}");
    println!("Index: {index:?}");

    todo!();
}
