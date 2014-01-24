// Babak Pourkazemi
// bp5xj

use std::io::buffered::BufferedReader;
use std::io::File;
use std::os;
use std::str;

fn main() {
	let args: ~[~str] = os::args();
	if args.len() != 3 {
		println!("Usage: {:s} <file1> <file2>", args[0]);
	} else {
		let fname1 = args[1].clone();
		let fname2 = args[2].clone();
		joiner(fname1, fname2);
	}
}

fn joiner(fn1: &str, fn2: &str) {
	let share1 = File::open(&Path::new(fn1.clone()));
	let share2 = File::open(&Path::new(fn2.clone()));
	let mut share1_bytes: ~[u8] = ~[];
	let mut share2_bytes: ~[u8] = ~[];

	// Read the random bytes from msg.share1
	match (share1) {
		Some(msg) => {
			let mut reader = BufferedReader::new(msg);
			let mut next = reader.read_byte();
			while(next != None) {
				share1_bytes.push(next.unwrap());
				next = reader.read_byte();
			}
		}
		None => {
			println("Opening " + fn1 + " failed!");
		}
	}

	// Read the encrypted bytes from msg.share2
	match (share2) {
		Some(msg) => {
			let mut reader = BufferedReader::new(msg);
			let mut next = reader.read_byte();
			while(next != None) {
				share2_bytes.push(next.unwrap());
				next = reader.read_byte();
			}
		}
		None => {
			println("Opening " + fn2 + " failed!");
		}
	}

	// Decrypt the bytes and print the secret
	let result = xor(share1_bytes, share2_bytes);
	for i in range(0, result.len()) {
		print!("{}", str::from_byte(result[i]));
	}
}

// From tutorial
fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
	let mut ret = ~[];
	for i in range(0, a.len()) {
		ret.push(a[i] ^ b[i]);
	}
	ret
}

