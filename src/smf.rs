
use std::io::Read;
use std::io;

use utils;
use events;

#[derive(Debug)]
pub enum SMFError {
	Io(io::Error),
	Parse(String)
}

impl From<io::Error> for SMFError {
    fn from(err: io::Error) -> SMFError {
        SMFError::Io(err)
    }
}

#[derive(Debug)]
pub struct FileHeader {
	pub magic: [u8; 4],
	pub length: u32,
	pub format: u16,
	pub num_tracks: u16,
	pub division: u16
}

#[derive(Debug)]
pub struct TrackHeader {
	pub magic: [u8; 4],
	pub length: u32
}

#[derive(Debug)]
pub struct Track {
	pub header: TrackHeader,
	pub events: Vec<events::Event>
}

#[derive(Debug)]
pub struct MIDIFile {
	pub header: FileHeader,
	pub tracks: Vec<Track>
}

/// 
/// Try to read and parse "Standard MIDI file format"-file
/// from the supplied reader
///
pub fn read_file(reader: &mut Read) -> Result<MIDIFile, SMFError> {

	let file_header = try!(read_file_header(reader));

	let mut tracks:Vec<Track> = Vec::new();
	
	for _ in 1..file_header.num_tracks+1 {
		let track_header = try!(read_track_header(reader));
		
		let mut track = try!(utils::read_vector(reader, track_header.length));
		let event_list = parse_events(&mut track);
		
		let parsed_track = Track{header: track_header, events: event_list};
		tracks.push(parsed_track);
	}
	
	Ok(MIDIFile{header: file_header, tracks: tracks})
}

fn read_file_header(reader: &mut Read) -> Result<FileHeader, SMFError> {
	let mut magic = [0u8; 4];
	try!(utils::read_all(reader, &mut magic));

	if magic != ['M' as u8, 'T' as u8, 'h' as u8, 'd' as u8] {
		return Err(SMFError::Parse(format!("Not a MIDI header '{:?}'", magic)));
	}
	
	let length = try!(utils::read_big_endian_u32(reader));
	if length != 6 {
		return Err(SMFError::Parse(format!("Wrong header size '{:?}'", length)));
	}
	let format = try!(utils::read_big_endian_u16(reader));
	let num_tracks = try!(utils::read_big_endian_u16(reader));
	let division = try!(utils::read_big_endian_u16(reader));

	let header = FileHeader{magic: magic, length: length, format: format, num_tracks: num_tracks, division: division};
	Ok(header)
}

fn read_track_header(reader: &mut Read) -> Result<TrackHeader, SMFError> {
	let mut magic = [0u8; 4];
	try!(utils::read_all(reader, &mut magic));

	if magic != ['M' as u8, 'T' as u8, 'r' as u8, 'k' as u8] {
		return Err(SMFError::Parse(format!("Not a MIDI track header '{:?}'", magic)));
	}
	
	let length = try!(utils::read_big_endian_u32(reader));

	let header = TrackHeader{magic: magic, length: length};
	Ok(header)
}

fn pop_variable_len(track: &mut Vec<u8>) -> u32 {
	let mut len:u32 = 0;
	
	loop {
		let val = track.pop().unwrap();
		len = (len << 7) + (val & 0x7f) as u32;
		if val & 0x80 == 0 {
			break;
		}
	}

	len
}

fn handle_meta_event(track: &mut Vec<u8>) -> events::MetaEvent {
	let meta_type = track.pop().unwrap();
	
	let length = pop_variable_len(track);

	let mut data:Vec<u8> = Vec::new();
	for _ in 0..length {
		let byte = track.pop().unwrap();
		data.push(byte);
	}

	let event = events::create_meta_event(meta_type, &mut data);
	
	event
}

fn handle_midi_event(status_byte: u8, track: &mut Vec<u8>) -> events::MIDIEvent {

	let event = events::create_midi_event(status_byte, track);
	
	event
}

fn parse_events(track: &mut Vec<u8>) -> Vec<events::Event> {
	let mut event_list: Vec<events::Event> = Vec::new();

	// pop() removes last element, so reverse the array
	track.reverse();
	
	let mut current_status:u8 = 0;
	
	while track.len() > 0 {
		
		let delta_time = pop_variable_len(track);

		let tmp_status_byte = track.pop().unwrap();
		
		if tmp_status_byte >= 0x80 {
			current_status = tmp_status_byte;
		} else {
			// midi running status
			track.push(tmp_status_byte);
		}
		
		let event = match current_status {
			0xff => {
				let me = handle_meta_event(track);
				let et = events::EventType::Meta(me);
				events::Event{delta_time: delta_time, event: et}
				},
			0xf0 | 0xf7 => {
				println!("sysex event");
				break;
				},
			_ => {
				let me = handle_midi_event(current_status, track);
				let et = events::EventType::MIDI(me);
				events::Event{delta_time: delta_time, event: et}
				}
		};
		
		event_list.push(event);
	}
	
	event_list
}
