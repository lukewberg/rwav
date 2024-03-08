use std::{
    fs::{self, File},
    io::Read,
    os::unix::fs::FileExt,
    path::Path,
};

use bytemuck::{Pod, Zeroable};

pub struct WavSample {}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct WavHeader {
    pub chunk_id: [u8; 4],
    pub chunk_size: u32,
    pub format: [u8; 4],
    pub fmt: FmtSubChunk,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct FmtSubChunk {
    pub subchunk_1_id: [u8; 4],
    pub subchunk_1_size: u32,
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct DataSubChunk {
    pub subchunk_2_id: [u8; 4],
    pub subchunk_2_size: u32,
}

struct Data {
    pub data: Box<[u8]>,
}

pub struct WavFile {
    pub handle: File,
    pub offset: u64,
    pub header: WavHeader,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ChunkHeader {
    pub chunk_id: [u8; 4],
    pub chunk_size: u32,
}

// #[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Chunk {
    pub chunk_header: ChunkHeader,
    pub data: Vec<u8>,
}

impl WavHeader {
    pub fn parse(file_handle: &mut File) -> Option<WavHeader> {
        let mut file_buffer = vec![0u8; std::mem::size_of::<WavHeader>()];
        let vec = vec![10, 11].iter();
        file_handle.read_exact(&mut file_buffer).unwrap();
        let header = bytemuck::try_from_bytes::<WavHeader>(&file_buffer)
            .expect("Unable to transmute wav header!");
        Some(*header)
    }
}

impl WavFile {
    pub fn new(path: &Path) -> Self {
        let mut file_handle = fs::File::open(path).expect("Unable to read file!");
        let header = WavHeader::parse(&mut file_handle).unwrap();

        WavFile {
            handle: file_handle,
            offset: 0,
            header,
        }
    }
}

impl Iterator for WavFile {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == 0 {
            self.offset = std::mem::size_of::<WavHeader>() as u64;
        };
        let mut info_buff = vec![0u8; std::mem::size_of::<ChunkHeader>()];
        // Read the chunk id and size
        self.handle
            .read_exact_at(&mut (*info_buff), self.offset)
            .unwrap();
        let chunk_header = bytemuck::try_from_bytes::<ChunkHeader>(&info_buff)
            .expect("Unable to transmute chunk info!");
        self.offset = self.offset + std::mem::size_of::<ChunkHeader>() as u64;

        let mut data_buffer = vec![0u8; chunk_header.chunk_size as usize];
        self.handle
            .read_exact_at(&mut data_buffer, self.offset)
            .unwrap();

        self.offset = self.offset + chunk_header.chunk_size as u64;

        Some(Chunk {
            chunk_header: *chunk_header,
            data: data_buffer,
        })
    }
}

// impl Display for WavHeader {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Format header - {}", String::from_utf8_lossy(&self.format));
//         write!(f, "Format block - {}", String::from_utf8_lossy(&self.format));
//         write!(f, "Audio format - {}", String::from_utf8_lossy(&self.format));
//     }
// }
