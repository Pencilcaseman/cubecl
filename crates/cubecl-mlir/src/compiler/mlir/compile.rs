use cubecl_core::{ir::*, prelude::KernelDefinition};

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

    // kernel.inputs[0]

    // kernel.inputs[0].into

    compile_scope(&kernel.body);

    todo!()
}

pub fn compile_scope(scope: &Scope) {
    for instruction in &scope.instructions {
        compile_instruction(instruction);
    }
}

pub fn compile_instruction(instruction: &Instruction) {
    match &instruction.operation {
        Operation::Copy(variable) => todo!(),
        Operation::Arithmetic(arithmetic) => todo!(),
        Operation::Comparison(comparison) => todo!(),
        Operation::Bitwise(bitwise) => todo!(),
        Operation::Operator(operator) => compile_operator(operator),
        Operation::Atomic(atomic_op) => todo!(),
        Operation::Metadata(metadata) => todo!(),
        Operation::Branch(branch) => todo!(),
        Operation::Synchronization(synchronization) => todo!(),
        Operation::Plane(plane) => todo!(),
        Operation::CoopMma(coop_mma) => todo!(),
        Operation::Pipeline(pipeline_ops) => todo!(),
        Operation::Barrier(barrier_ops) => todo!(),
        Operation::Tma(tma_ops) => todo!(),
        Operation::NonSemantic(non_semantic) => todo!(),
    }
}

pub fn compile_operator(operator: &Operator) {
    match operator {
        Operator::Index(binary_operator) => {
            compile_index(&binary_operator.lhs, &binary_operator.rhs)
        }
        Operator::CopyMemory(copy_memory_operator) => todo!(),
        Operator::CopyMemoryBulk(copy_memory_bulk_operator) => todo!(),
        Operator::Slice(slice_operator) => todo!(),
        Operator::UncheckedIndex(binary_operator) => todo!(),
        Operator::IndexAssign(binary_operator) => todo!(),
        Operator::InitLine(line_init_operator) => todo!(),
        Operator::UncheckedIndexAssign(binary_operator) => todo!(),
        Operator::And(binary_operator) => todo!(),
        Operator::Or(binary_operator) => todo!(),
        Operator::Not(unary_operator) => todo!(),
        Operator::Cast(unary_operator) => todo!(),
        Operator::Bitcast(unary_operator) => todo!(),
        Operator::Select(select) => todo!(),
        Operator::ConditionalRead(conditional_read) => todo!(),
    }
}

pub fn compile_index(arr: &Variable, index: &Variable) {
    println!("Array: {arr:?}");
    println!("Index: {index:?}");

    todo!();
}
