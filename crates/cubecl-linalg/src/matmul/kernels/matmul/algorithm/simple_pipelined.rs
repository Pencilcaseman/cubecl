use super::base;
use cubecl_core::prelude::*;
use std::marker::PhantomData;

use crate::matmul::components::batch::{CubeCountDispatch, CubeDispatch};
use crate::matmul::components::global::single_stage::CyclicLoading;
use crate::matmul::components::stage::{self};
use crate::matmul::components::MatmulProblem;
use crate::matmul::components::{batch, global};
use crate::matmul::components::{tile, MatmulSelection};

pub struct SimplePipelinedAlgorithm<TMM, Dispatch = batch::TransposedDispatch> {
    pub _tmm: PhantomData<TMM>,
    pub _dispatch: PhantomData<Dispatch>,
}

impl<TMM, Dispatch> base::Algorithm for SimplePipelinedAlgorithm<TMM, Dispatch>
where
    TMM: tile::TileMatmulFamily,
    Dispatch: CubeDispatch + CubeCountDispatch,
{
    type TileMatmul = TMM;
    type StageMatmul = stage::multi_buffer::MultiBufferMatmulFamily<Self::TileMatmul>;
    type GlobalMatmul = global::single_stage::simple::SimplePipelinedMatmulFamily<
        Self::StageMatmul,
        CyclicLoading,
        CyclicLoading,
    >;

    type BatchMatmul = batch::one_to_one::OneToOneMatmulFamily<Self::GlobalMatmul, Dispatch>;
    type Selection = MatmulSelection;

    fn cube_dim(selection: &MatmulSelection) -> CubeDim {
        CubeDim::new(selection.plane_dim, selection.tile_count.m, 1)
    }

    fn cube_count(selection: &MatmulSelection, problem: &MatmulProblem) -> CubeCount {
        let m_stage = selection.tile_count.m * selection.tile_shape.m;
        let n_stage = selection.tile_count.n * selection.tile_shape.n;
        let cubes_for_m = (problem.m as u32 + m_stage - 1) / m_stage;
        let cubes_for_n = (problem.n as u32 + n_stage - 1) / n_stage;

        Dispatch::cube_count(cubes_for_m, cubes_for_n, problem.num_batches() as u32)
    }

    fn advanced_config() -> crate::matmul::kernels::matmul::AdvancedConfig {
        crate::matmul::kernels::matmul::AdvancedConfig {
            lhs_tiling_layout: stage::TilingLayout::Contiguous(stage::TilingOrder::ColMajor),
            rhs_tiling_layout: stage::TilingLayout::Contiguous(stage::TilingOrder::RowMajor),
            enforced_matrix_layout: (None, None),
        }
    }
}
