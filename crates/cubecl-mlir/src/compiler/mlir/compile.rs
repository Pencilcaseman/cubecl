use std::{cell::RefCell, rc::Rc, sync::Arc};

use cubecl_core::{ir::*, prelude::KernelDefinition};
use rustc_hash::FxHashMap;

pub struct MlirCompilerModule<'a, 'b> {
    location: &'a melior::ir::Location<'b>,
    module: &'a melior::ir::Module<'b>,

    type_map: FxHashMap<(Option<VariableKind>, Item), melior::ir::Type<'b>>,
    blocks: FxHashMap<u32, melior::ir::Block<'b>>,
    variable_map: FxHashMap<VariableKind, (u32, usize)>,

    current_block: u32,
}

impl<'a, 'b> MlirCompilerModule<'a, 'b> {
    pub fn new(
        location: &'a melior::ir::Location<'b>,
        module: &'a melior::ir::Module<'b>,
    ) -> Self {
        Self {
            location,
            module,
            type_map: FxHashMap::default(),
            blocks: FxHashMap::default(),
            variable_map: FxHashMap::default(),
            current_block: 0,
        }
    }

    pub fn get_or_create_type(&mut self, kind: VariableKind, item: Item) {
        use melior::ir::r#type::{IntegerType as Int, Type as T};

        let context = unsafe { self.location.context().to_ref() };

        let base_type =
            self.type_map.entry((None, item)).or_insert_with(|| {
                match item.elem {
                    Elem::Float(float_kind) => match float_kind {
                        FloatKind::F16 | FloatKind::BF16 => T::float16(context),
                        FloatKind::Flex32
                        | FloatKind::F32
                        | FloatKind::TF32 => T::float32(context),
                        FloatKind::F64 => T::float64(context),
                    },
                    Elem::Int(int_kind) => match int_kind {
                        IntKind::I8 => Int::signed(context, 8),
                        IntKind::I16 => Int::signed(context, 16),
                        IntKind::I32 => Int::signed(context, 32),
                        IntKind::I64 => Int::signed(context, 64),
                    }
                    .into(),
                    Elem::UInt(uint_kind) => match uint_kind {
                        UIntKind::U8 => Int::unsigned(context, 8),
                        UIntKind::U16 => Int::unsigned(context, 16),
                        UIntKind::U32 => Int::unsigned(context, 32),
                        UIntKind::U64 => Int::unsigned(context, 64),
                    }
                    .into(),
                    Elem::AtomicFloat(_float_kind) => todo!(),
                    Elem::AtomicInt(_int_kind) => todo!(),
                    Elem::AtomicUInt(_uint_kind) => todo!(),
                    Elem::Bool => todo!(),
                }
            });

        let _thing = match kind {
            VariableKind::GlobalInputArray(_)
            | VariableKind::GlobalOutputArray(_)
            | VariableKind::LocalArray { id: _, length: _ } => {
                // NOTE: Dimensions = [1] is fine. The dimensions aren't checked
                // by LLVM/MLIR and are only used for strided accesses

                melior::ir::r#type::MemRefType::new(
                    *base_type,
                    &[1],
                    None,
                    None,
                )
                .into()
            }

            VariableKind::GlobalScalar(_)
            | VariableKind::ConstantScalar(_)
            | VariableKind::LocalConst { id: _ }
            | VariableKind::LocalMut { id: _ } => *base_type,

            VariableKind::Versioned { id: _, version: _ } => todo!(),
            VariableKind::TensorMap(_) => todo!(),
            VariableKind::ConstantArray { id: _, length: _ } => todo!(),
            VariableKind::SharedMemory { id: _, length: _, alignment: _ } => {
                todo!()
            }
            VariableKind::Matrix { id: _, mat: _ } => todo!(),
            VariableKind::Slice { id: _ } => todo!(),
            VariableKind::Builtin(_builtin) => todo!(),
            VariableKind::Pipeline { id: _, item: _, num_stages: _ } => todo!(),
            VariableKind::Barrier { id: _, item: _, level: _ } => todo!(),
        };

        // self.type_map.insert((kind, item), r#type);
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

    let mut compiler_module = MlirCompilerModule::new(&location, &mut module);

    for (index, variable) in kernel.inputs.iter().enumerate() {
        let key = match variable.location {
            cubecl_core::compute::Location::Storage => {
                VariableKind::GlobalInputArray(
                    u32::try_from(index)
                        .expect("Too many variables in function definition"),
                )
            }
            cubecl_core::compute::Location::Cube => todo!(),
        };

        // Safe, as the entry block is guaranteed to be created first
        compiler_module.variable_map.insert(key, (0, index));
    }

    compile_scope(&kernel.body, &compiler_module);

    todo!()
}

pub fn compile_scope(
    scope: &Scope,
    compiler_module: &MlirCompilerModule<'_, '_>,
) {
    // let block = Block::new(&[(index_type, location), (index_type,
    // location)]);

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
    compiler_module: &MlirCompilerModule<'_, '_>,
) {
    println!("Array: {arr:?}");
    println!("Index: {index:?}");

    let arr_var = compiler_module
        .variable_map
        .get(&arr.kind)
        .expect("Array to be indexed not registered");

    // let op = melior::dialect::memref::load(
    //     block.argument(0).unwrap().into(),
    //     &[const_zero.result(0).unwrap().into()],
    //     location,
    // );

    todo!();
}
