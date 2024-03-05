// pub mod bindings;
pub mod cli;
pub mod wav;

pub mod utils {

    pub fn u32_transmute_ascii_str_le(string: &str) -> Result<u32, &'static str> {
        let bytes = (*string).as_bytes();
        if bytes.len() != 4 {
            return Err("Incorect number of bytes!")
        }
        let num_arr: [u8; 4] = bytes.try_into().unwrap();
        let num = u32::from_le_bytes(num_arr);
        Ok(num)
    }
}

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}