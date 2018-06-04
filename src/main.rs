#![feature(plugin)]
#![feature(nll)]
#![feature(iterator_step_by)]

extern crate png;

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let decoder = png::Decoder::new(File::open(&args[1]).unwrap());
	let (info, mut reader) = decoder.read_info().unwrap();
	let mut buf = vec![0; info.buffer_size()];
	reader.next_frame(&mut buf).unwrap();
	// print!("{:?}x{:?}, per pixel: {:?}\n", info.width, info.height, buf.len() / (info.width * info.height) as usize);
	let mut tiles = 0;
	let tiles_wide = info.width / 8;
	let tiles_tall = info.height / 8;
	let total_tiles = tiles_wide * tiles_tall;
	for tile in 0 .. total_tiles {
		let mut tile_empty = true;
		'alpha_check: for y in 0 .. 8 {
			for x in 0 .. 8 {
				let row = tile / tiles_wide;
				let column = tile % tiles_wide;
				let row_pixel = ((row * 8) + y) * info.width;
				let pixel_index = row_pixel + (column * 8) + x;
				let offset = (pixel_index * 4) as usize;
				let a = buf[offset + 3];
				if a > 0 {
					tile_empty = false;
					break 'alpha_check;
				}
			}
		}
		if !tile_empty { tiles += 1; }
	}
	print!("{:?}\n", tiles);
	io::stdout().flush().unwrap();
	let _ = io::stdin().read(&mut [0u8]).unwrap();
}
