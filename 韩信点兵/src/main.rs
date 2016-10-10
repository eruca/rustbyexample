use std::io;

fn main() {
	let mut inputs = Vec::new();
	loop {
		let mut input = String::new();
		match io::stdin().read_line(&mut input){
			Ok(_) =>{},
			Err(err) => {println!("{:?}", err);return;},
		}

		if input.as_str().trim() == "end" {
			break;
		}

		inputs.push(input);
	}

	let outputs : Vec<Vec<i32>> = inputs.into_iter()
										.map(str_trans)
										.collect();

    if !outputs.iter().all(|v| v.len() == 3 && v[0] < 3 && v[1] < 5 && v[2] < 7){
    	println!("请输入3个整数(a b c 并且 a < 3, b < 5, c < 7)，以空格隔开");
    	return;
    }

    let result :Vec<String> = outputs.into_iter().enumerate().map(|(k,v)|{
    	let result :Vec<i32> = (10..101).filter_map(|c| find(c,v[0],v[1],v[2])).collect();
    	join_to_string(result, k+1, ",")
    }).collect();

    let s = result.into_iter().fold(String::new(),|mut output, s|{
    	output.push_str(&s);
    	output
    });

    print!("{}", s);
}

fn join_to_string<T:ToString>(v: Vec<T>, i: usize, sep: &str) -> String {
	let length = v.len();
	if length == 0 {
		return format!("Case {}: No answer\n", i);
	}

	v.into_iter().enumerate().fold(format!("Case {}: ", i), move |mut output, (k, t)|{
		output.push_str(&t.to_string());
		if k == length -1 {
			output.push('\n');
		} else {
			output.push_str(sep);
		}
		output
	})
}

fn str_trans(s :String) -> Vec<i32> {
	s.as_str().trim().split(char::is_whitespace).filter_map(|s| s.parse().ok()).collect()
}


fn find(num: i32, a :i32, b :i32, c :i32) -> Option<i32> {
	if (num - a) % 3 == 0 && (num - b) % 5 == 0 && (num - c) % 7 == 0 {
		return Some(num);
	}
	None
}