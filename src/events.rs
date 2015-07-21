
use utils;

#[derive(Debug)]
pub enum MetaEventType {
	SequenceNumber = 0x00,
	Text = 0x01,
	Copyright = 0x02,
	TrackName = 0x03,
	Instrument = 0x04,
	Lyric = 0x05,
	Marker = 0x06,
	CuePoint = 0x07,
	MIDIChannel = 0x20,
	MIDIPort = 0x21,
	EndOfTrack = 0x2f,
	Tempo = 0x51,
	SMPTEOffset = 0x54,
	TimeSignature = 0x58,
	KeySignature = 0x59
}

#[derive(Debug)]
pub struct MetaEventGenericText {
	pub str: String
}

#[derive(Debug)]
pub struct MetaEventGenericData {
	pub data: Vec<u8>
}

#[derive(Debug)]
pub struct MetaEventTempo {
	pub ms_per_quarter_note: u32,
	pub bpm: f64
}

#[derive(Debug)]
pub struct MetaEventTimeSignature {
	numerator: u8,
	denominator: u8,
	number_of_midi_clocks: u8,
	number_of_32_notes: u8
}

#[derive(Debug)]
pub enum KeySignature {
	CSharpMajor = 0x0700,
	ASharpminor = 0x0701,
	FSharpmajor = 0x0600,
	DSharpminor = 0x0601,
	BMajor = 0x0500,
	GSharpminor = 0x0501,
	EMajor = 0x0400,
	CSharpMinor = 0x0401,
	AMajor = 0x0300,
	FSharpMinor = 0x0301,
	DMajor = 0x0200,
	BMinor = 0x0201,
	GMajor = 0x0100,
	EMinor = 0x0101,
	CMajor = 0x0000,
	AMinor = 0x0001,
	FMajor = 0xff00,
	DMinor = 0xff01,
	BFlatMajor = 0xfe00,
	GMinor = 0xfe01,
	EFlatMajor = 0xfd00,
	CMinor = 0xfd01,
	AFlatMajor = 0xfc00,
	FMinor = 0xfc01,
	DFlatMajor = 0xfb00,
	BFlatMinor = 0xfb01,
	GFlatMajor = 0xfa00,
	EFlatMinor = 0xfa01,
	CFlatMajor = 0xf900,
	AFlatMinor = 0xf901
}

#[derive(Debug)]
pub struct MetaEventKeySignature {
	sharp_flat: u8,
	major_minor: u8,
	key: KeySignature
}

#[derive(Debug)]
pub enum MetaEventData {
	GenericText(MetaEventGenericText),
	GenericData(MetaEventGenericData),
	Tempo(MetaEventTempo),
	TimeSignature(MetaEventTimeSignature),
	KeySignature(MetaEventKeySignature)
}


#[derive(Debug)]
pub enum MIDIEventType {
	NoteOff = 0x8, // 2 bytes note number, note velocity
	NoteOn = 0x9, // 2 bytes note number, note velocity
	PolyphonicAftertouch = 0xa, // 2 bytes note number, pressure
	ControlOrModeChange = 0xb, // 2 bytes
	ProgramChange = 0xc, // 1 byte program
	Aftertouch = 0xd, // 1 byte pressure
	PitchBendChange = 0xe // 2 bytes lsb msb
}

#[derive(Debug)]
pub struct MIDIEventNoteOff {
	pub number: u8,
	pub velocity: u8,
	pub note_name: String
}

#[derive(Debug)]
pub struct MIDIEventNoteOn {
	pub number: u8,
	pub velocity: u8,
	pub note_name: String
}

#[derive(Debug)]
pub struct MIDIEventGenericData {
	data0: u8,
	data1: u8
}

#[derive(Debug)]
pub enum MIDIEventData {
	NoteOff(MIDIEventNoteOff),
	NoteOn(MIDIEventNoteOn),
	GenericData(MIDIEventGenericData)
}

#[derive(Debug)]
pub struct MIDIEvent {
	pub status: MIDIEventType,
	pub channel: u8,
	pub data: MIDIEventData
}

#[derive(Debug)]
pub struct MetaEvent {
	pub event_type: MetaEventType,
	pub event_data: MetaEventData
}

#[derive(Debug)]
pub struct SysexEvent {
	dummy: u8
}

#[derive(Debug)]
pub enum EventType {
	MIDI(MIDIEvent),
	Meta(MetaEvent),
	Sysex(SysexEvent)
}

#[derive(Debug)]
pub struct Event {
	pub delta_time: u32,
	pub event: EventType
}

fn u8_to_meta_event_type(meta_type:u8) -> MetaEventType {
	let met = match meta_type {
		0x00 => MetaEventType::SequenceNumber,
		0x01 => MetaEventType::Text,
		0x02 => MetaEventType::Copyright,
		0x03 => MetaEventType::TrackName,
		0x04 => MetaEventType::Instrument,
		0x05 => MetaEventType::Lyric,
		0x06 => MetaEventType::Marker,
		0x07 => MetaEventType::CuePoint,
		0x20 => MetaEventType::MIDIChannel,
		0x21 => MetaEventType::MIDIPort,
		0x2f => MetaEventType::EndOfTrack,
		0x51 => MetaEventType::Tempo,
		0x54 => MetaEventType::SMPTEOffset,
		0x58 => MetaEventType::TimeSignature,
		0x59 => MetaEventType::KeySignature,
		_ => {
			panic!("Unknown meta event type {} ", meta_type);
		}
	};

	met
}



pub fn create_meta_event(meta_type:u8, data: &mut Vec<u8>) -> MetaEvent {
	let human_readable = u8_to_meta_event_type(meta_type);
	
	let event = match human_readable {
		MetaEventType::Text | 
		MetaEventType::Copyright |
		MetaEventType::TrackName |
		MetaEventType::Instrument |
		MetaEventType::Lyric => {
			let s = String::from_utf8(data.clone()).unwrap();
			let megt = MetaEventData::GenericText(MetaEventGenericText{str: s});
			MetaEvent{event_type: human_readable, event_data: megt}
			},
		MetaEventType::Tempo => {
			let ms_per_quarter_note = utils::vector_to_big_endian_u32(data);
			let bpm = 60000000f64 / ms_per_quarter_note as f64;
			let met = MetaEventData::Tempo(MetaEventTempo{ms_per_quarter_note: ms_per_quarter_note, bpm: bpm});
			MetaEvent{event_type: human_readable, event_data: met}							
		},
		MetaEventType::TimeSignature => {
			let numerator = data[0];
			let base:u8 = 2;
			let denominator:u8 = base.pow(data[1] as u32);
			let number_of_midi_clocks = data[2];
			let number_of_32_notes = data[3];
			let mets = MetaEventData::TimeSignature(MetaEventTimeSignature{numerator: numerator, denominator: denominator, 
						number_of_midi_clocks: number_of_midi_clocks, number_of_32_notes: number_of_32_notes});
			MetaEvent{event_type: human_readable, event_data: mets}							
		},
		MetaEventType::KeySignature => {
			let sf = data[0];
			let mi = data[1];
			let key = sf_mi_to_key_signature(sf,mi);
			let meks = MetaEventData::KeySignature(MetaEventKeySignature{sharp_flat: sf, major_minor: mi, key: key});
			MetaEvent{event_type: human_readable, event_data: meks}							
		},
		_ => {
			let megd = MetaEventData::GenericData(MetaEventGenericData{data: data.clone()});
			MetaEvent{event_type: human_readable, event_data: megd}				
		}
	};

	
	
	event
}

fn u8_to_midi_event_type(midi_type:u8) -> MIDIEventType {
	let met = match midi_type {
		0x8 => MIDIEventType::NoteOff, // 2 bytes note number, note velocity
		0x9 => MIDIEventType::NoteOn, // 2 bytes note number, note velocity
		0xa => MIDIEventType::PolyphonicAftertouch, // 2 bytes note number, pressure
		0xb => MIDIEventType::ControlOrModeChange, // 2 bytes
		0xc => MIDIEventType::ProgramChange, // 1 byte program
		0xd => MIDIEventType::Aftertouch, // 1 byte pressure
		0xe => MIDIEventType::PitchBendChange, // 2 bytes lsb msb
		_ => panic!("Unknown midi event {}", midi_type)
	};
	
	met
}

fn note_number_to_note_name(number:u8) -> String {

	let octave:i8 = -1 + (number / 12) as i8;
	let note = match number % 12 {
		0 => "C",
		1 => "C#",
		2 => "D",
		3 => "D#",
		4 => "E",
		5 => "F",
		6 => "F#",
		7 => "G",
		8 => "G#",
		9 => "A",
		10 => "A#",
		11 => "B",
		_ => panic!("modulo arithmetic failed on {}", number)
	};
	
	let note_name = format!("{}-{}",note,octave);
	
	note_name
}

fn sf_mi_to_key_signature(sf:u8, mi:u8) -> KeySignature {
	let val = ((sf as u16) << 8) | (mi as u16);
	let key = match val {
		0x0700 => KeySignature::CSharpMajor,
		0x0701 => KeySignature::ASharpminor,
		0x0600 => KeySignature::FSharpmajor,
		0x0601 => KeySignature::DSharpminor,
		0x0500 => KeySignature::BMajor,
		0x0501 => KeySignature::GSharpminor,
		0x0400 => KeySignature::EMajor,
		0x0401 => KeySignature::CSharpMinor,
		0x0300 => KeySignature::AMajor,
		0x0301 => KeySignature::FSharpMinor,
		0x0200 => KeySignature::DMajor,
		0x0201 => KeySignature::BMinor,
		0x0100 => KeySignature::GMajor,
		0x0101 => KeySignature::EMinor,
		0x0000 => KeySignature::CMajor,
		0x0001 => KeySignature::AMinor,
		0xff00 => KeySignature::FMajor,
		0xff01 => KeySignature::DMinor,
		0xfe00 => KeySignature::BFlatMajor,
		0xfe01 => KeySignature::GMinor,
		0xfd00 => KeySignature::EFlatMajor,
		0xfd01 => KeySignature::CMinor,
		0xfc00 => KeySignature::AFlatMajor,
		0xfc01 => KeySignature::FMinor,
		0xfb00 => KeySignature::DFlatMajor,
		0xfb01 => KeySignature::BFlatMinor,
		0xfa00 => KeySignature::GFlatMajor,
		0xfa01 => KeySignature::EFlatMinor,
		0xf900 => KeySignature::CFlatMajor,
		0xf901 => KeySignature::AFlatMinor,
		_ => panic!("Unknown key signature")
	};

	key
}

pub fn create_midi_event(status_byte: u8, track: &mut Vec<u8>) -> MIDIEvent {
	let status = (status_byte & 0xf0) >> 4;
	let channel = status_byte & 0x0f;
	
	let event_type = u8_to_midi_event_type(status);
	
	let event = match event_type {
		MIDIEventType::NoteOff => {
			let number = track.pop().unwrap();
			let velocity = track.pop().unwrap();
			let note_name = note_number_to_note_name(number);
			let med = MIDIEventData::NoteOff(MIDIEventNoteOff{number: number, velocity: velocity, note_name: note_name});
			MIDIEvent{status: event_type, channel: channel, data: med }
		},
		MIDIEventType::NoteOn => {
			let number = track.pop().unwrap();
			let velocity = track.pop().unwrap();
			let note_name = note_number_to_note_name(number);
			let med = MIDIEventData::NoteOn(MIDIEventNoteOn{number: number, velocity: velocity, note_name: note_name});
			MIDIEvent{status: event_type, channel: channel, data: med }
		},
		// 2 byte data
		MIDIEventType::PolyphonicAftertouch |
		MIDIEventType::ControlOrModeChange |
		MIDIEventType::PitchBendChange => {
			let data0 = track.pop().unwrap();
			let data1 = track.pop().unwrap();
			let med = MIDIEventData::GenericData(MIDIEventGenericData{data0: data0, data1: data1});
			MIDIEvent{status: event_type, channel: channel, data: med }
		}, 
		// 1 byte data
		MIDIEventType::ProgramChange |
		MIDIEventType::Aftertouch => {
			let data0 = track.pop().unwrap();
			let data1 = 0;
			let med = MIDIEventData::GenericData(MIDIEventGenericData{data0: data0, data1: data1});
			MIDIEvent{status: event_type, channel: channel, data: med }
		}
	};

	event
}
