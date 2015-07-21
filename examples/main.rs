use std::fs::File;
use std::io::BufReader;
use std::env;

extern crate midifile;

use midifile::smf;

fn main() {
	let args : Vec<String> = env::args().collect();

	for i in 1..args.len() {	
		let ref filename = args[i];
		println!("***** {} *****", filename);

		let file = match File::open(filename) {
			Ok(file) => file,
			Err(..) => panic!("Failed to open file"),
		};
		
		let mut reader = BufReader::new(&file);
		let midi_file = smf::read_file(&mut reader);

		println!("File header: {:?}",midi_file.header);
		for track in midi_file.tracks {
			println!("Track header: {:?}", track.header);
			for event in track.events {
				println!("Event: {:?}", event)
			}
		}
		
		println!("***** *****")
	}
}
