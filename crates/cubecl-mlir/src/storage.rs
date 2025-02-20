use std::{
    alloc::{alloc, dealloc, Layout},
    collections::HashMap,
    ffi,
    ptr::NonNull,
};

use cubecl_runtime::storage::{
    ComputeStorage, StorageHandle, StorageId, StorageUtilization,
};
use derive_new::new;

// pub type MlirPointer = *mut u8;
// pub type MlirNonNull = NonNull<u8>;

#[derive(Debug, Copy, Clone)]
pub struct MlirPointer {
    ptr: *mut u8,
}

#[derive(Debug, Copy, Clone)]
pub struct MlirNonNull {
    non_null: NonNull<u8>,
}

unsafe impl Send for MlirPointer {}
unsafe impl Sync for MlirPointer {}
unsafe impl Send for MlirNonNull {}
unsafe impl Sync for MlirNonNull {}

impl From<*mut u8> for MlirPointer {
    fn from(ptr: *mut u8) -> Self {
        Self { ptr }
    }
}

impl MlirNonNull {
    pub fn new(ptr: MlirPointer) -> Self {
        Self { non_null: NonNull::new(ptr.ptr).expect("Pointer was NULL") }
    }
}

#[derive(Debug)]
pub struct MlirStorage {
    memory: HashMap<StorageId, MlirNonNull>,
    deallocations: Vec<StorageId>,
}

#[derive(new, Debug)]
pub struct MlirResource {
    pub ptr: MlirNonNull,
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

impl Default for MlirStorage {
    fn default() -> Self {
        Self { memory: Default::default(), deallocations: Default::default() }
    }
}

impl MlirStorage {
    pub fn new() -> Self {
        Default::default()
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
        let id = StorageId::new();
        let layout =

        Layout::from_size_align(size as usize,
            Self::ALIGNMENT as usize)
            .expect("Failed to construct Layout. Ensure size is non-zero and does not overflow ISIZE");

        unsafe {
            let ptr = MlirPointer::from(alloc(layout));
            self.memory.insert(id, MlirNonNull::new(ptr));
        }

        StorageHandle::new(id, StorageUtilization { offset: 0, size })
    }

    fn dealloc(&mut self, id: StorageId) {
        todo!()
    }
}
