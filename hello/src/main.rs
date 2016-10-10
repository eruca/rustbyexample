fn main(){
	let mut sum = 0f64;
	let mut term;
	
	for i in 0.. {
		term = 1.0/ ((2*i) +1) as f64;
		if i&1 == 0 {
			sum += term;
		} else {
			sum -= term;
		}

		if term < 1e-6 {
			break;
		}
	}

	println!("{:.6}", sum);
}


// use std::io;
// // use std::fmt;

// const INPUT_FORMAT :&'static str = "请输入三个整数，空格相隔";

// fn main() {
// 	let mut input = String::new();
// 	println!("{}",INPUT_FORMAT);
// 	match io::stdin().read_line(&mut input){
// 		Ok(count) =>{println!("共输入了{}个字符", count);},
// 		Err(_) => {
// 			println!("{}",INPUT_FORMAT);
// 			return;
// 		}
// 	}

// 	// let input :Vec<&str> = input.as_str().trim().split(char::is_whitespace).filter(|&s| s!="").collect();
// 	// let input :Vec<i32> = input.into_iter().map(|s| s.parse::<i32>().unwrap_or(0)).collect();
// 	if !input.as_str().trim().chars().all(|c| c.is_digit(10)) {
// 		println!("请输入数值摄氏度");
// 		return;
// 	}

// 	match input.as_str().trim().parse::<i32>().map(|f| (5*(f - 32)) as f64 / 9 as f64) {
// 		Ok(c) => println!("{:?}", c),
// 		Err(err) => println!("{}", err),
// 	}

// 	// input.sort();

// 	// let sum = input.iter().fold(0,|sum, &i| sum+i);
// 	// let average = sum as f64 / input.len() as f64;

// 	// println!("{:.*}",3, average); 
// }
