use std::ffi;

use cubecl_runtime::storage::{
    ComputeStorage, StorageHandle, StorageId, StorageUtilization,
};
use derive_new::new;

#[derive(new, Debug)]
pub struct MlirStorage;

#[derive(new, Debug)]
pub struct MlirResource {
    pub ptr: ffi::c_void,
    offset: u64,
    size: u64,
}

impl MlirResource {
    /// Return the buffer size.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Return the buffer offset.
    pub fn offset(&self) -> u64 {
        self.offset
    }
}

impl ComputeStorage for MlirStorage {
    type Resource = MlirResource;

    // Use 64-byte alignment to cover all SIMD architectures.
    // SSE:     16 bytes
    // AltiVec: 16 bytes
    // NEON:    16 bytes
    // AVX:     32 bytes
    // AVX-512: 64 bytes
    const ALIGNMENT: u64 = 64;

    fn get(&mut self, handle: &StorageHandle) -> Self::Resource {
        todo!()
    }

    fn alloc(&mut self, size: u64) -> StorageHandle {
        todo!()
    }

    fn dealloc(&mut self, id: StorageId) {
        todo!()
    }
}
