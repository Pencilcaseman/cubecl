#![warn(clippy::pedantic, clippy::nursery)]

// mod backend;
mod compiler;
// mod compute;
// mod device;
// mod element;
// mod graphics;
mod runtime;
mod server;
mod storage;

// pub use compiler::base::*;
// pub use compiler::wgsl::WgslCompiler;
// pub use compute::*;
// pub use device::*;
// pub use element::*;
// pub use graphics::*;
pub use runtime::*;

// #[cfg(test)]
// #[allow(unexpected_cfgs)]
// mod tests_spirv {
//     pub type TestRuntime = crate::MlirRuntime;
//     use cubecl_core::flex32;
//     use half::f16;
//
//     cubecl_core::testgen_all!(f32: [f16, flex32, f32, f64], i32: [i8, i16,
// i32, i64], u32: [u8, u16, u32, u64]);
//     cubecl_linalg::testgen_matmul_plane!([f16, f32]);
//     cubecl_linalg::testgen_matmul_tiling2d!([f16, f32, f64]);
//     cubecl_linalg::testgen_matmul_simple!([f32]);
//     cubecl_linalg::testgen_matmul_accelerated!([f16]);
//     cubecl_reduce::testgen_reduce!();
//     cubecl_reduce::testgen_shared_sum!([f32]);
// }
