use std::future::Future;

use cubecl_common::future;
use cubecl_core::{
    prelude::CubeTask, server::Handle, Feature, MemoryConfiguration,
};
use cubecl_runtime::{
    memory_management::{MemoryDeviceProperties, MemoryManagement},
    server::{self, ComputeServer},
    storage::BindingResource,
    TimestampsError, TimestampsResult,
};
use derive_new::new;

use crate::{
    compiler::{MlirCompilationOptions, MlirCompiler},
    storage::MlirStorage,
};

#[derive(Debug)]
pub struct MlirServer {
    mem: MemoryManagement<MlirStorage>,
}

impl MlirServer {
    pub fn new(
        memory_properties: MemoryDeviceProperties,
        memory_config: MemoryConfiguration,
        compilation_options: MlirCompilationOptions,
    ) -> Self {
        Self {
            mem: MemoryManagement::from_configuration(
                MlirStorage::new(),
                &memory_properties,
                memory_config,
            ),
        }
    }
}

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
        let slice_handle = self.mem.reserve(size as u64);
        Handle::new(slice_handle, None, None, size as u64)
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
