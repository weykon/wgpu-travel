use wgpu::{Device, Queue, Adapter};

pub struct AdapterStorage {
    pub device: Device,
    pub queue: Queue,
    pub adapter: Adapter,
}
