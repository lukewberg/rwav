use std::{
    fmt::Display,
    fs,
    io::Read,
    path::Path,
};

use bytemuck::{Pod, Zeroable};

use crate::AudioBalanceFade;

pub struct WavSample {}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct WavHeader {
    chunk_id: [u8; 4],
    chunk_size: u32,
    format: [u8; 4],
    fmt: FmtSubChunk,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct FmtSubChunk {
    subchunk_1_id: [u8; 4],
    subchunk_1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct DataSubChunk {
    subchunk_2_id: [u8; 4],
    subchunk_2_size: u32,
}

struct Data {
    data: Box<[u8]>,
}

pub struct WavFile {}

impl WavHeader {
    pub fn parse(path: &Path) -> Option<WavHeader> {
        let mut file_handle = fs::File::open(path).expect("Unable to read file!");
        let mut file_buffer = vec![0u8; std::mem::size_of::<WavHeader>()];
        file_handle.read_exact(&mut file_buffer).unwrap();
        let header = bytemuck::try_from_bytes::<WavHeader>(&file_buffer)
            .expect("Unable to transmute wav header!");
        Some(*header)
    }
}

// impl Display for WavHeader {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Format header - {}", String::from_utf8_lossy(&self.format));
//         write!(f, "Format block - {}", String::from_utf8_lossy(&self.format));
//         write!(f, "Audio format - {}", String::from_utf8_lossy(&self.format));
//     }
// }
