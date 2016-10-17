use std::io;
use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();
	map.insert('E','3');
	map.insert('A','A');
	map.insert('H','H');
	map.insert('I','I');
	map.insert('J','L');
	map.insert('M','M');
	map.insert('O','O');
	map.insert('S','2');
	map.insert('T','T');
	map.insert('U','U');
	map.insert('V','V');
	map.insert('W','W');
	map.insert('X','X');
	map.insert('Y','Y');
	map.insert('2','S');
	map.insert('1','1');
	map.insert('3','E');
	map.insert('Z','5');
	map.insert('5','Z');
	map.insert('8','8');

	let out_words = vec!["is not a palindrome.","is a regular palindrome.","is a mirrored string","is a mirrored palindrome"];


	let mut inputs = Vec::new();

	loop{
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(_) => {},
			Err(e) => {println!("{:?}", e);return;}
		}

		if input.as_str().trim() == "quit"{
			break;
		}

		// println!("{}", is_palindrome(&input));
		// println!("{}", is_mirrored(&input, &map));

		inputs.push(input);
	}

	let inputs:Vec<_> = inputs.into_iter().map(|s| {
		let a = is_palindrome(&s);
		let b = is_mirrored(&s, &map);

		if a {
			if b {
				format!("{} -- {}\n",s.as_str().trim(), out_words[3])
			} else {
				format!("{} -- {}\n",s.as_str().trim(), out_words[1])			
			}
		} else {
			if b{
				format!("{} -- {}\n",s.as_str().trim(), out_words[2])
			} else {
				format!("{} -- {}\n",s.as_str().trim(), out_words[0])
			}
		}
	}).collect();

	println!("{}", inputs.join("\n"));
}

fn is_palindrome(s: &str) -> bool {
	let s = s.trim();
	// let rev :String = s.chars().rev().collect();
	// rev.eq(s)
	let length = s.len();
	if length == 1 {
		return true;
	}

	if length &1 == 0 {
		let(a, b) = s.split_at(length/2);
		let b:String =b.chars().rev().collect();
		b.eq(a)
	} else {
		let (a, b) = (&s[0..length/2],&s[(length+1)/2..length]);
		let b:String = b.chars().rev().collect();
		b.eq(a)
	}
}

fn is_mirrored(s: &str, map:&HashMap<char, char>) -> bool {
	let s = s.trim();
	let length = s.len();
	if length == 1 {
		return s.chars().all(|c|map.get(&c).map_or(false, |_|true));
	}

	if length & 1 == 0 {
		let(a, b) = s.split_at(length/2);
		let c:String = b.chars().rev().filter_map(|c|{
			match map.get(&c) {
				Some(&v) => Some(v),
				None => None,
			}
		}).collect();

		if c.len() != b.len() {
			return false;
		}

		c.eq(a)
	} else {
		let (a, b) = (&s[..length/2], &s[(length+1)/2..]);
		if !&s[length/2..(length+1)/2].chars().all(|c| map.get(&c).map_or(false,|&v| v == c)){
			return false;
		}

		let c:String = b.chars().rev().filter_map(|c|{
			match map.get(&c) {
				Some(&v) => Some(v),
				None => None,
			}
		}).collect();

		if c.len() != b.len() {
			return false;
		}

		c.eq(a)
	}
}
