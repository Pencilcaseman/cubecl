use crate::{
    hip::{HipDialect, arch::AMDArchitecture},
    shared::{
        Architecture, Component, Elem, Fragment, FragmentIdent, FragmentLayout,
        SupportedWmmaCombinations, Variable, WmmaCompiler, WmmaInstruction,
    },
};
use cubecl_core::ir::{self as gpu};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WmmaIntrinsicCompiler {}

impl WmmaCompiler<HipDialect<Self>> for WmmaIntrinsicCompiler {
    type Architecture = AMDArchitecture;

    fn wmma_includes(_f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // nothing to do
        Ok(())
    }

    fn deftypes(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("typedef __bf16 bhalf8_t __attribute__((ext_vector_type(8)));\n")?;
        f.write_str("typedef __bf16 bhalf16_t __attribute__((ext_vector_type(16)));\n")?;
        f.write_str("typedef _Float16 half8_t __attribute__((ext_vector_type(8)));\n")?;
        f.write_str("typedef _Float16 half16_t __attribute__((ext_vector_type(16)));\n")?;
        f.write_str("typedef float float8_t __attribute__((ext_vector_type(8)));\n")
    }

    fn local_variables(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // threads 0-15 and threads 16-31 of the wavefront hold the same fragments respectively
        // in other words fragments are duplicated
        // so lanes 0,16 / 1,17 / ... / 15, 31 are the same
        f.write_str("uint wmmaLane = uint(threadIdx.x % 16);\n")
    }

    fn compile_fragment_ident(
        _ident: &FragmentIdent<HipDialect<Self>>,
        _f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        // nothing to do
        Ok(())
    }

    fn compile_fragment_layout(
        _layout: &FragmentLayout<HipDialect<Self>>,
        _f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        // nothing to do
        Ok(())
    }

    fn compile_fragment(
        fragment: &Fragment<HipDialect<Self>>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match fragment.ident {
            FragmentIdent::A | FragmentIdent::B => match fragment.elem {
                Elem::F16 => write!(f, "half16_t"),
                Elem::BF16 => write!(f, "bhalf16_t"),
                other => panic!("unsupported type {other} for {fragment}"),
            },
            FragmentIdent::Accumulator => match fragment.elem {
                Elem::F16 => write!(f, "half16_t"),
                Elem::BF16 => write!(f, "bhalf16_t"),
                Elem::F32 => write!(f, "float8_t"),
                other => panic!("unsupported type {other} for {fragment}"),
            },
            FragmentIdent::_Dialect(_) => Ok(()),
        }
    }

    fn compile_instruction(
        instruction: &WmmaInstruction<HipDialect<Self>>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match instruction {
            WmmaInstruction::Fill { frag, value } => {
                let fill_with_zeros =
                    matches!(value, Variable::ConstantScalar(number, _) if number.is_zero());
                if fill_with_zeros {
                    write!(
                        f,
                        "// fill
{frag} = {{}};
"
                    )
                } else {
                    write!(
                        f,
                        "// fill
for (uint i = 0; i < uint(8); ++i) {{
  {frag}[i] = {value};
}}
"
                    )
                }
            }
            WmmaInstruction::Load {
                frag,
                value,
                layout,
                ..
            } => {
                // Matrix A must be in column major layout (so fragments correspond to a row)
                // Matrices B, C and D must be in row major layout (so fragments correspond to a column)
                //
                // Each lane is a thread so each column get 8 VGPRs used to store fragments
                // Here is the layout for C and D matrices and how they map to registers
                //
                // Lane index   0      1      2      3      ...     13     14     15     ...     17     18     ...     30     31
                // --------------------------------------------------------------------------------------------------------------
                // VGPR0      | 1,1  | 1,2  | 1,3  | 1,4  | ...  | 1,13 | 1,14 | 1,15 | ...  | 2,1  | 2,2  | ...  | 2,15 | 2,16 |
                // --------------------------------------------------------------------------------------------------------------
                // VGPR1      | 3,1  | 3,2  | 3,3  | 3,4  | ...  | 3,13 | 3,14 | 3,15 | ...  | 4,1  | 4,2  | ...  | 4,15 | 4,16 |
                // --------------------------------------------------------------------------------------------------------------
                // VGPR2      | 5,1  | 5,2  | 5,3  | 5,4  | ...  | 5,13 | 5,14 | 5,15 | ...  | 6,1  | 6,2  | ...  | 6,15 | 6,16 |
                // --------------------------------------------------------------------------------------------------------------
                // VGPR3      | 7,1  | 7,2  | 7,3  | 7,4  | ...  | 7,13 | 7,14 | 7,15 | ...  | 8,1  | 8,2  | ...  | 8,15 | 8,16 |
                // --------------------------------------------------------------------------------------------------------------
                // VGPR4      | 9,1  | 9,2  | 9,3  | 9,4  | ...  | 9,13 | 9,14 | 9,15 | ...  | 10,1 | 10,2 | ...  | 10,15| 10,16|
                // --------------------------------------------------------------------------------------------------------------
                // VGPR5      | 11,1 | 11,2 | 11,3 | 11,4 | ...  | 11,13| 11,14| 11,15| ...  | 12,1 | 12,2 | ...  | 12,15| 12,16|
                // --------------------------------------------------------------------------------------------------------------
                // VGPR6      | 13,1 | 13,2 | 13,3 | 13,4 | ...  | 13,13| 13,14| 13,15| ...  | 14,1 | 14,2 | ...  | 14,15| 14,16|
                // --------------------------------------------------------------------------------------------------------------
                // VGPR7      | 15,1 | 15,2 | 15,3 | 15,4 | ...  | 15,13| 15,14| 15,15| ...  | 16,1 | 16,2 | ...  | 16,15| 16,16|
                // --------------------------------------------------------------------------------------------------------------
                let item = value.item();
                let mut value_ident = format!("{value}");
                if item.vectorization > 1 {
                    writeln!(
                        f,
                        "__half* {value}_half = reinterpret_cast<__half*>({value});"
                    )?;
                    value_ident = format!("{value}_half");
                }
                // TODO: support iu8 and iu4
                let (index, length, step) = match frag {
                    Variable::WmmaFragment { frag: inner, .. } => {
                        match inner.ident {
                            FragmentIdent::A | FragmentIdent::B => {
                                let length = 16;
                                let step = 1;
                                // fragment a and b are always in half precision and they don't require special attention
                                // to how they are stored in memory as matrix A and B are also in half precision
                                let index = if (inner.ident == FragmentIdent::A
                                    && inner.layout.unwrap() == FragmentLayout::ColMajor)
                                    || (inner.ident == FragmentIdent::B
                                        && inner.layout.unwrap() == FragmentLayout::RowMajor)
                                {
                                    "i * uint(16) + wmmaLane"
                                } else {
                                    "i + wmmaLane * uint(16)"
                                };
                                (index, length, step)
                            }
                            FragmentIdent::Accumulator => {
                                let length = 8;
                                let step = get_output_accumulator_index_step(value, inner);
                                let index = match layout {
                                    Some(FragmentLayout::ColMajor) => {
                                        "(i * uint(2) + threadIdx.x / uint(16)) + wmmaLane * uint(16)"
                                    }
                                    Some(FragmentLayout::RowMajor) => {
                                        "(i * uint(2) + threadIdx.x / uint(16)) * uint(16) + wmmaLane"
                                    }
                                    _ => panic!(
                                        "cannot load data to an accumulator without knowing the layout of the data"
                                    ),
                                };
                                (index, length, step)
                            }
                            other => panic!("unknown matrix identifier {other}"),
                        }
                    }
                    other => panic!("{other} is not a WMMMA fragment!"),
                };
                write!(
                    f,
                    "// load
for (uint i = 0; i < uint({length}); ++i) {{
  {frag}[i * {step}] = {value_ident}[{index}];
}}
"
                )
            }
            WmmaInstruction::Execute {
                frag_a,
                frag_b,
                frag_c,
                frag_d,
                warp_size,
            } => {
                if *warp_size == 64 {
                    panic!("Wavefront size 64 not yet supported.")
                }
                let ab_format = if let Variable::WmmaFragment { frag: inner_a, .. } = frag_a {
                    if let Variable::WmmaFragment { frag: inner_b, .. } = frag_b {
                        if inner_a.elem == inner_b.elem {
                            match inner_a.elem {
                                Elem::F16 => "f16",
                                Elem::BF16 => "bf16",
                                other => {
                                    panic!("{other} format not supported for {frag_a} and {frag_b}")
                                }
                            }
                        } else {
                            panic!(
                                "{frag_a} and {frag_b} have different types (respectively {} and {})",
                                inner_a.elem, inner_b.elem
                            )
                        }
                    } else {
                        panic!("{frag_b} is not a WMMA fragment!")
                    }
                } else {
                    panic!("{frag_a} is not a WMMA fragment!")
                };
                let (cd_format, opsel) = if let Variable::WmmaFragment { frag: inner_c, .. } =
                    frag_c
                {
                    if let Variable::WmmaFragment { frag: inner_d, .. } = frag_d {
                        if inner_c.elem == inner_d.elem {
                            match inner_c.elem {
                                Elem::F32 => ("f32", ""),
                                Elem::F16 => ("f16", ", false"),
                                Elem::BF16 => ("bf16", ", false"),
                                other => {
                                    panic!("{other} format not supported for {frag_c} and {frag_d}")
                                }
                            }
                        } else {
                            panic!(
                                "{frag_c} and {frag_d} have different types (respectively {} and {})",
                                inner_c.elem, inner_d.elem
                            )
                        }
                    } else {
                        panic!("{frag_d} is not a WMMA fragment!")
                    }
                } else {
                    panic!("{frag_c} is not a WMMA fragment!")
                };
                writeln!(
                    f,
                    "{frag_d} = __builtin_amdgcn_wmma_{cd_format}_16x16x16_{ab_format}_w{warp_size}({frag_a}, {frag_b}, {frag_c}{opsel});"
                )
            }
            WmmaInstruction::Store {
                output,
                frag,
                layout,
                ..
            } => {
                let item = output.item();
                let mut output_ident = format!("{output}");
                if item.vectorization > 1 {
                    writeln!(
                        f,
                        "float* {output}_float = reinterpret_cast<float*>({output});"
                    )?;
                    output_ident = format!("{output}_float");
                }
                // frag holds a result column where threads 0-15 of the wavefront have the even rows and threads 16-31 the odd rows
                // moreover, since we use OPSEL to false in the Execute instruction in f16 output format, the output elements are
                // stored in even indexes (0, 2, 4, ...) (low 16-bits of the VGPR) in frag
                let frag_idx = match frag {
                    Variable::WmmaFragment { frag: inner, .. } => match inner.elem {
                        Elem::F16 | Elem::BF16 => "elemIdx * 2",
                        Elem::F32 => "elemIdx",
                        other => panic!(
                            "C fragment format cannot be {other}. Only f16, bf16 and f32 are supported."
                        ),
                    },
                    other => panic!("{frag} is not a WMMA fragment (it is a {other})!"),
                };
                // FragmentLayout here represents the desired layout of the matrix C
                let output_idx = match layout {
                    FragmentLayout::ColMajor => "wmmaLane * uint(16) + rowIdx",
                    FragmentLayout::RowMajor => "wmmaLane + rowIdx * uint(16)",
                    FragmentLayout::_Dialect(_) => "",
                };
                write!(
                    f,
                    "// store
for (uint elemIdx = 0; elemIdx < uint(8); ++elemIdx) {{
  const uint rowIdx = elemIdx * uint(2) + threadIdx.x / uint(16);
  {output_ident}[{output_idx}] = {frag}[{frag_idx}];
}}
 "
                )
            }
            WmmaInstruction::Cast { input, output } => {
                let step = match output {
                    Variable::WmmaFragment { frag: inner, .. } => match inner.ident {
                        FragmentIdent::Accumulator => {
                            get_output_accumulator_index_step(input, inner)
                        }
                        _ => 1,
                    },
                    _ => 1,
                };
                write!(
                    f,
                    "// cast
for (uint elemIdx = 0; elemIdx < uint(8); ++elemIdx) {{
  {output}[elemIdx * {step}] = {input}[elemIdx];
}}
 "
                )
            }
        }
    }

    fn supported_wmma_combinations(arch: &Self::Architecture) -> SupportedWmmaCombinations {
        // Reference: https://gpuopen.com/learn/wmma_on_rdna3/
        let mut result: SupportedWmmaCombinations = vec![];
        if arch.is_wmma_capable() {
            // Types fully supported.
            let types = vec![
                (
                    gpu::Elem::Float(gpu::FloatKind::F16), // i
                    gpu::Elem::Float(gpu::FloatKind::F16), // o
                    gpu::Elem::Float(gpu::FloatKind::F16), // c
                ),
                (
                    gpu::Elem::Float(gpu::FloatKind::F16),
                    gpu::Elem::Float(gpu::FloatKind::F16),
                    gpu::Elem::Float(gpu::FloatKind::F32),
                ),
                (
                    gpu::Elem::Float(gpu::FloatKind::BF16),
                    gpu::Elem::Float(gpu::FloatKind::BF16),
                    gpu::Elem::Float(gpu::FloatKind::F32),
                ),
            ];
            let combinations: SupportedWmmaCombinations = types
                .into_iter()
                //                           m   n   k
                .map(|(i, o, c)| (i, o, c, vec![(16, 16, 16)]))
                .collect();
            result.extend(combinations);
        }
        result
    }
}

fn get_output_accumulator_index_step(
    input: &Variable<HipDialect<WmmaIntrinsicCompiler>>,
    output: &Fragment<HipDialect<WmmaIntrinsicCompiler>>,
) -> u32 {
    // Each VGPR is 32 bit wide and there is 8 VGPR per lane, an accumulator can then be either:
    // - a vector of 8 float
    // - a vector of 16 half
    // Depending on the precision used for the input, the whole 32 bits per register will be used or
    // just only 16 bits. In such a case we always use the lower 16 bits (opsel set to false) which means
    // that we only assign values to even indexes of the accumulator (0, 2, 4, ...)

    assert_eq!(
        output.ident,
        FragmentIdent::<HipDialect<WmmaIntrinsicCompiler>>::Accumulator
    );

    match input.elem() {
        Elem::F16 | Elem::BF16 | Elem::F32 => {
            match output.elem {
                // loading into accumulator of 16 half precision
                Elem::F16 | Elem::BF16 => 2,
                // loading into accumulator of 8 full precision
                Elem::F32 => 1,
                other => panic!("unsupported format {other} for {output}"),
            }
        }
        other => panic!("unsupported format {other} for {input}"),
    }
}
