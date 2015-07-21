
use std::io::Read;

pub fn read_big_endian_u32(reader: &mut Read) -> u32 {
	let mut data_arr = [0u8; 4];
	match reader.read(&mut data_arr) {
		Ok(num_read) if num_read == data_arr.len() => (),
		_ => panic!("Failed to read bytes")
	}

	let mut data:u32 = 0;
	
	for n in data_arr.iter() {
		data = (data << 8) + *n as u32;
	}
	
	data
}

pub fn read_big_endian_u16(reader: &mut Read) -> u16 {
	let mut data_arr = [0u8; 2];
	match reader.read(&mut data_arr) {
		Ok(num_read) if num_read == data_arr.len() => (),
		_ => panic!("Failed to read bytes")
	}

	let mut data:u16 = 0;
	
	for n in data_arr.iter() {
		data = (data << 8) + *n as u16;
	}
	
	data
}

pub fn vector_to_big_endian_u32(data_arr: &mut Vec<u8>) -> u32 {
	let mut data:u32 = 0;
	
	for n in data_arr.iter() {
		data = (data << 8) + *n as u32;
	}
	
	data
}

pub fn read_vector(reader: &mut Read, length:u32) -> Vec<u8> {
	let mut data: Vec<u8> = vec![0; length as usize];
	match reader.read(&mut data) {
		Ok(num_read) if num_read == data.len() => (),
		_ => panic!("Failed to read bytes")
	}
	data
}
