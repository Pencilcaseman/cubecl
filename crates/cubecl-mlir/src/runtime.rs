use cubecl_core::{
    AtomicFeature, CubeDim, DeviceId, Feature, MemoryConfiguration, Runtime,
    channel::{ComputeChannel, MutexComputeChannel},
    ir::{Elem, FloatKind},
    prelude::ComputeClient,
};
use cubecl_runtime::{
    ComputeRuntime, DeviceProperties,
    memory_management::{HardwareProperties, MemoryDeviceProperties},
    storage::ComputeStorage,
};

use crate::{
    compiler::{MlirCompilationOptions, MlirCompiler},
    storage::MlirStorage,
};

#[derive(Debug)]
pub struct MlirRuntime;

use crate::server::MlirServer;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MlirDevice {
    Cpu,
}

impl Default for MlirDevice {
    fn default() -> Self {
        Self::Cpu
    }
}

static MLIR_RUNTIME: ComputeRuntime<
    MlirDevice,
    MlirServer,
    MutexComputeChannel<MlirServer>,
> = ComputeRuntime::new();

impl Runtime for MlirRuntime {
    type Compiler = MlirCompiler;

    type Server = MlirServer;

    type Channel = MutexComputeChannel<MlirServer>;

    type Device = MlirDevice;

    fn device_id(_device: &Self::Device) -> DeviceId {
        todo!()
    }

    fn client(
        device: &Self::Device,
    ) -> ComputeClient<Self::Server, Self::Channel> {
        MLIR_RUNTIME.client(device, move || {
            let mem_props = MemoryDeviceProperties {
                max_page_size: 4096,
                alignment: MlirStorage::ALIGNMENT,
            };

            // TODO: These are all random numbers
            let hardware_props = HardwareProperties {
                plane_size_min: 0,
                plane_size_max: 0,
                max_bindings: 1024,
                max_shared_memory_size: 4096,
                max_cube_count: CubeDim::new(1024, 1024, 1024),
                max_units_per_cube: 1024,
                max_cube_dim: CubeDim::new(1024, 1024, 1024),
            };

            let device_props =
                DeviceProperties::new(&[], mem_props.clone(), hardware_props);

            let server = MlirServer::new(
                mem_props,
                MemoryConfiguration::default(),
                MlirCompilationOptions::default(),
            );
            let channel = MutexComputeChannel::new(server);

            ComputeClient::new(channel, device_props, ())
        })
    }

    fn name(
        _client: &ComputeClient<Self::Server, Self::Channel>,
    ) -> &'static str {
        todo!()
    }

    fn supported_line_sizes() -> &'static [u8] {
        todo!()
    }

    fn max_cube_count() -> (u32, u32, u32) {
        todo!()
    }

    fn require_array_lengths() -> bool {
        false
    }

    fn line_size_elem(elem: &Elem) -> impl Iterator<Item = u8> + Clone {
        Self::supported_line_sizes()
            .iter()
            .filter(|v| **v as usize * elem.size() <= 16)
            .copied() // 128 bits
    }
}
