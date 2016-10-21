use std::io;

fn main() {
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_) => {},
		Err(e) => {println!("{:?}", e);return;}
	}

	if !input.as_str().trim().chars().all(|c| {
		match c {
			'a'| 'A' | 'c' | 'C' | 'g' | 'G' | 't' | 'T' => true,
			_ => false,
		}
	}){
		println!("请输入ACGT的环状序列");
		return;
	}


	let input  = input.as_str().trim().to_uppercase();

	let v8:&[u8] = input.as_ref();
	let length = v8.len();
	let mut ans = 0usize;

	for i in 1..length {
		ans = compair(ans,i, v8);
	}

	let result = String::from_utf8_lossy(&v8[ans..]).to_string() + &String::from_utf8_lossy(&v8[..ans]);
    println!("result: {}", result);
}

fn compair(ans:usize, curr: usize, v8: &[u8]) -> usize {
	let mut ans_mut:usize = ans;
	let mut curr_mut:usize = curr;
	let length = v8.len();

	while ans_mut < length && curr_mut < length {
		if v8[ans_mut] < v8[curr_mut] {
			return ans;
		} else if v8[ans_mut] > v8[curr_mut] {
			return curr;
		} else {
			ans_mut += 1;
			curr_mut += 1;
		}
	}

	if ans_mut == length {
		ans_mut = 0;
	}

	if curr_mut == length {
		curr_mut = 0;	
	}

	let result = compair(ans_mut, curr_mut, v8);
	if result == ans_mut {
		return ans;
	} else {
		return curr;
	}
}

#[test]
fn test_compair(){
	let v8:&[u8] = "11341".as_ref();
	assert_eq!(compair(0,4,v8), 4);
}