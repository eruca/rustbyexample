use std::io;
use std::collections::HashMap;

fn main() {
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_) => {},
		Err(e) => {println!("{:?}", e);return;},
	}

	let mut map: HashMap<u32,u32> = HashMap::new();

	let trim_input = input.trim();
	if trim_input.len() > 10000 || !trim_input.chars().all(|c| {
		if !c.is_digit(10){
			return false;
		}

		let digit = match c.to_digit(10){
			Some(n) => n,
			None => 11u32,
		};

		if digit == 11u32 {
			return false;
		}

		let has = map.contains_key(&digit);
		if !has {
			map.insert(digit, 1);
			return true;
		}

		if let Some(x) = map.get_mut(&digit) {
			*x += 1;
		};

		true
	}){
		println!("input digit from 0..9, and the count must < 10000");
		return;
	}

	println!("{:?}", map);
}
