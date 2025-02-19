use std::future::Future;

use cubecl_common::future;
use cubecl_core::{prelude::CubeTask, Feature};
use cubecl_runtime::{
    server::{self, ComputeServer},
    storage::BindingResource,
    TimestampsError, TimestampsResult,
};
use derive_new::new;

use crate::{compiler::MlirCompiler, storage::MlirStorage};

#[derive(new, Debug)]
pub struct MlirServer;

impl ComputeServer for MlirServer {
    type Kernel = Box<dyn CubeTask<MlirCompiler>>;

    type Storage = MlirStorage;

    type Feature = Feature;

    fn read(
        &mut self,
        bindings: Vec<server::Binding>,
    ) -> impl Future<Output = Vec<Vec<u8>>> + Send + 'static {
        async { todo!() }
    }

    fn get_resource(
        &mut self,
        binding: server::Binding,
    ) -> BindingResource<Self> {
        todo!()
    }

    fn create(&mut self, data: &[u8]) -> server::Handle {
        todo!()
    }

    fn empty(&mut self, size: usize) -> server::Handle {
        todo!()
    }

    unsafe fn execute(
        &mut self,
        kernel: Self::Kernel,
        count: server::CubeCount,
        bindings: Vec<server::Binding>,
        kind: cubecl_core::ExecutionMode,
    ) {
        todo!()
    }

    fn flush(&mut self) {
        todo!()
    }

    fn sync(&mut self) -> impl Future<Output = ()> + Send + 'static {
        async { todo!() }
    }

    fn sync_elapsed(
        &mut self,
    ) -> impl Future<Output = TimestampsResult> + Send + 'static {
        async { todo!() }
    }

    fn memory_usage(&self) -> cubecl_core::MemoryUsage {
        todo!()
    }

    fn enable_timestamps(&mut self) {
        todo!()
    }

    fn disable_timestamps(&mut self) {
        todo!()
    }
}
