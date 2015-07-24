
use std::io::Read;
use std::io;

pub fn read_all(reader: &mut Read, data: &mut [u8]) -> io::Result<usize> {
	let mut pos = 0;
	while pos < data.len() {
		let slice = &mut data[pos..];
		let n = try!(reader.read(slice));
		pos += n;
	}
	
	Ok(pos)
}

pub fn read_big_endian_u32(reader: &mut Read) -> io::Result<u32> {
	let mut data_arr = [0u8; 4];
	try!(read_all(reader, &mut data_arr));

	let mut data:u32 = 0;
	
	for n in data_arr.iter() {
		data = (data << 8) + *n as u32;
	}
	
	Ok(data)
}

pub fn read_big_endian_u16(reader: &mut Read) -> io::Result<u16> {
	let mut data_arr = [0u8; 2];
	try!(read_all(reader, &mut data_arr));

	let mut data:u16 = 0;
	
	for n in data_arr.iter() {
		data = (data << 8) + *n as u16;
	}
	
	Ok(data)
}

pub fn vector_to_big_endian_u32(data_arr: &mut Vec<u8>) -> u32 {
	let mut data:u32 = 0;
	
	for n in data_arr.iter() {
		data = (data << 8) + *n as u32;
	}
	
	data
}

pub fn read_vector(reader: &mut Read, length:u32) -> io::Result<Vec<u8>> {
	let mut data: Vec<u8> = vec![0; length as usize];
	try!(read_all(reader, &mut data));

	Ok(data)
}
