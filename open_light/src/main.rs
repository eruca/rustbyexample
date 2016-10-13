use std::io;

fn main() {
	let mut input = String::new();
	match io::stdin().read_line(&mut input){
		Ok(_) => {},
		Err(e) => {println!("{:?}", e);return;}
	}

	let inputs:Vec<i32> = input.as_str().trim().split(char::is_whitespace).filter_map(|s| s.parse().ok()).collect();
	if inputs.len() != 2 || inputs[0] > inputs[1] || inputs[0] > 1000 {
		println!("请输入k人, n栈灯, k<= n <= 1000");
		return;
	}

	let (k, n) = (inputs[0], inputs[1] as usize);

	let result:Vec<usize> = (2..k+1).fold(vec![true;n], sum)
								   .into_iter()
								   .enumerate()
								   .filter_map(|(k,v)|{
								   		if v {
								   			Some(k+1)
								   		} else{
								   			None
								   		}
								   }).collect();

	println!("{:?}", result);
}

fn sum(v:Vec<bool>, n:i32) -> Vec<bool> {
	v.into_iter().enumerate().map(|(k,v)|{
		let k = k +1;
		if k as i32 % n == 0 {
			return !v;
		}
		v
	}).collect()
}
