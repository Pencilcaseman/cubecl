use cubecl_core::{
    channel::{ComputeChannel, MutexComputeChannel},
    ir::{Elem, FloatKind},
    AtomicFeature, DeviceId, Feature, Runtime,
};

use crate::compiler::MlirCompiler;

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
    ) -> cubecl_core::prelude::ComputeClient<Self::Server, Self::Channel> {
        todo!()
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
