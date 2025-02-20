use cubecl_core::{
    channel::{ComputeChannel, MutexComputeChannel},
    ir::{Elem, FloatKind},
    prelude::ComputeClient,
    AtomicFeature, DeviceId, Feature, MemoryConfiguration, Runtime,
};
use cubecl_runtime::{
    memory_management::{HardwareProperties, MemoryDeviceProperties},
    storage::ComputeStorage,
    ComputeRuntime, DeviceProperties,
};

use crate::{compiler::MlirCompiler, storage::MlirStorage};

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

    fn device_id(device: &Self::Device) -> DeviceId {
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

            let hardware_props = HardwareProperties {
                plane_size_min: 0,
                plane_size_max: 0,
                max_bindings: 1024,
                max_shared_memory_size: 4096,
            };

            let device_props =
                DeviceProperties::new(&[], mem_props.clone(), hardware_props);

            let server = MlirServer::new(
                mem_props,
                MemoryConfiguration::default(),
                Default::default(),
            );
            let channel = MutexComputeChannel::new(server);

            ComputeClient::new(channel, device_props)
        })
    }

    fn name() -> &'static str {
        todo!()
    }

    fn extension() -> &'static str {
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
            .cloned() // 128 bits
    }
}
