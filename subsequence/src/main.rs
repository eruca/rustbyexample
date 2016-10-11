use std::io;

const TIPS: &'static str = "输入两个正整数n<m<10^6";

fn main() {
    println!("{}",TIPS);
    let mut inputs = Vec::new();
    loop{
    	let mut input = String::new();
    	match io::stdin().read_line(&mut input) {
    		Ok(_) => {},
    		Err(err) => {println!("{:?}", err);return;}
    	}

    	let input:Vec<i64> = input.as_str().trim().split(char::is_whitespace)
    							  .filter_map(|s| s.parse().ok()).collect();

    	println!("{:?}", input);

    	if input.len()!= 2 || !input.iter().all(|&n| n>=0 && n <1000000) 
    		|| ((input[0] * input[1]) == 0 && input[0] != input[1]){

    		println!("{}",TIPS);
    		return;
    	}

    	if input[0] == 0 && input[1] == 0 {
    		break;
    	}

    	inputs.push(input);
    }

    let outputs:Vec<String> = inputs.into_iter().enumerate().map(help).collect();
    print!("{}", outputs.concat());
}

fn help(kv: (usize, Vec<i64>)) -> String {
	let (k, v) = kv;
	let (n, m) = (v[0],v[1]);
	let sum = (n..m+1).fold(0f64, |mut sum, i| {
		sum += 1 as f64 / i.pow(2) as f64;
		sum
	});
	format!("Case {}: {:.5}\n", k+1, sum)
}