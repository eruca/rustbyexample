use std::io;

fn main() {
	let mut input = String::new();
	match io::stdin().read_line(&mut input){
		Ok(_) =>{},
		Err(err) => { println!("{:?}", err);return;}
	}

	let upper_str = input.trim().to_uppercase();
	let input_length = upper_str.len();
	println!("{}", input_length);

	let mut error = false;
	if input_length < 1 || input_length > 80 {
		error = true;
	}

	let (sum, _,  all_xo) = upper_str.chars().fold((0, 0, true), |tuple, c| {
		let (sum, pre, all_xo) = tuple;
		if !all_xo{
			return tuple;
		}

		if c == 'X' {
			return (sum, 0, true);
		}

		if c == 'O' {
			if pre == 0 {
				return (sum +1, 1, true);
			}
			return (sum + pre + 1, pre+1, true);
		}

		(0,0,false)
	});

	if error || !all_xo {
		println!("请输入OX组成的序列，长度为1~80");
		return;
	}

	println!("{} - sum: {}", upper_str, sum);
}
