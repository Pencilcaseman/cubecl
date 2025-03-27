use std::{
    cell::RefCell,
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex},
};

use cubecl_core::{
    Feature, MemoryConfiguration, contiguous_strides,
    ir::{Elem, VariableKind},
    prelude::CubeTask,
    server::Handle,
};
use cubecl_runtime::{
    TimestampsResult,
    memory_management::{
        MemoryDeviceProperties, MemoryHandle, MemoryManagement,
    },
    server::{self, ComputeServer},
    storage::{BindingResource, ComputeStorage},
};

use crate::{
    compiler::{MlirCompilationOptions, MlirCompiler},
    storage::MlirStorage,
};

#[derive(Debug)]
pub struct MlirServer {
    mem: MemoryManagement<MlirStorage>,
    compilation_options: MlirCompilationOptions,
}

impl MlirServer {
    pub fn new(
        memory_properties: MemoryDeviceProperties,
        memory_config: MemoryConfiguration,
        compilation_options: MlirCompilationOptions,
    ) -> Self {
        let registry = melior::dialect::DialectRegistry::new();
        melior::utility::register_all_dialects(&registry);

        Self {
            mem: MemoryManagement::from_configuration(
                MlirStorage::new(),
                &memory_properties,
                memory_config,
            ),
            compilation_options: MlirCompilationOptions::default(),
        }
    }
}

impl ComputeServer for MlirServer {
    type Kernel = Box<dyn CubeTask<MlirCompiler>>;
    type Storage = MlirStorage;
    type Feature = Feature;
    type Info = ();

    fn read(
        &mut self,
        _bindings: Vec<server::Binding>,
    ) -> impl Future<Output = Vec<Vec<u8>>> + Send + 'static {
        async { todo!() }
    }

    fn get_resource(
        &mut self,
        _binding: server::Binding,
    ) -> BindingResource<<Self::Storage as ComputeStorage>::Resource> {
        todo!()
    }

    fn create(&mut self, data: &[u8]) -> server::Handle {
        let size = data.len() as u64;

        let slice_handle = self.mem.reserve(size, None);
        let resource = self
            .mem
            .get_resource(slice_handle.clone().binding(), None, None)
            .expect("Unable to acquire resource. Maybe allocation failed?");

        Handle::new(slice_handle, None, None, size)
    }

    fn empty(&mut self, size: usize) -> server::Handle {
        let slice_handle = self.mem.reserve(size as u64, None);
        Handle::new(slice_handle, None, None, size as u64)
    }

    unsafe fn execute(
        &mut self,
        kernel: Self::Kernel,
        count: server::CubeCount,
        constants: Vec<server::ConstBinding>,
        bindings: Vec<server::Binding>,
        kind: cubecl_core::ExecutionMode,
    ) {
        println!("Kernel Name: {}", kernel.name());

        let mut kernel_id = kernel.id();
        kernel_id.mode(kind);

        let mut compiler = MlirCompiler::new();

        println!("Kernel ID: {}", kernel.id());
        println!("Count: {count:?}");
        println!("Bindings: {bindings:?}");
        println!("Mode: {kind:?}");

        println!("Memory: {:?}", bindings[0].memory);

        println!();
        for bin in bindings {
            let resource = self
                .mem
                .get_resource(
                    bin.memory.clone(),
                    bin.offset_start,
                    bin.offset_end,
                )
                .expect("Failed to get resource");

            println!("Resource: {resource:?}");
        }

        let _ = kernel.compile(&mut compiler, &self.compilation_options, kind);

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

    fn memory_cleanup(&mut self) {
        todo!();
    }

    fn enable_timestamps(&mut self) {
        todo!()
    }

    fn disable_timestamps(&mut self) {
        todo!()
    }

    fn read_tensor(
        &mut self,
        _bindings: Vec<server::BindingWithMeta>,
    ) -> impl Future<Output = Vec<Vec<u8>>> + Send + 'static {
        async { todo!() }
    }

    fn create_tensor(
        &mut self,
        data: &[u8],
        shape: &[usize],
        _elem_size: usize,
    ) -> (Handle, Vec<usize>) {
        let strides = contiguous_strides(shape);
        let handle = self.create(data);
        (handle, strides)
    }

    fn empty_tensor(
        &mut self,
        shape: &[usize],
        elem_size: usize,
    ) -> (Handle, Vec<usize>) {
        let strides = contiguous_strides(shape);
        let size = shape.iter().product::<usize>() * elem_size;
        let handle = self.empty(size);
        (handle, strides)
    }
}
