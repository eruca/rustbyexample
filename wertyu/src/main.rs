use std::io;
use std::collections::HashMap;

const LINETOP: &'static str = "QWERTYUIOP[]\\";
const LINEMID: &'static str = "ASDFGHJKL;,";
const LINEBOT: &'static str = "ZXCVBNM,./";

fn main() {
	// let mut map = HashMap::new();

	let (mut map, _) = LINETOP.chars().fold((HashMap::new(),0 as char),init);
	let (map1, _) = LINEMID.chars().fold((HashMap::new(),0 as char),init);
	let (map2, _) = LINEBOT.chars().fold((HashMap::new(),0 as char), init);
	map.extend(map1);
	map.extend(map2);

	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_) => {},
		Err(e) => {println!("{:?}", e);return;}
	}

	let out: String = input.as_str().trim().chars().map(move |c|{
		if c == ' ' {
			return c;
		}
		let &mc = map.get(&c).unwrap();
		mc
	}).collect();
	println!("{}", out);
}

fn init(sum:(HashMap<char,char>,char), curr:char) -> (HashMap<char,char>, char) {
	let (mut map, last) = sum;
	if last != 0 as char {
		map.insert(curr, last);
	}

	(map, curr)
}