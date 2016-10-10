use std::io;

fn main() {
	println!("输入正整数<=20, 输出一个n层的倒三角形");

	let mut input = String::new();
	match io::stdin().read_line(&mut input){
		Ok(_) => {},
		Err(err) => {println!("{:?}",err);return;},
	}

	let num:i32 = match input.as_str().trim().parse(){
		Ok(n) => n,
		Err(err) => {println!("{:?}", err); return;}
	};

	let s = (1..num+1).rev().enumerate().map(|(i,n)|{
		let mut s = (0..i).map(|_|' ').collect::<String>();
		let mut a = (1..n*2).map(|_|'#').collect::<String>();
		a.push('\n');
		s.push_str(&a);
		s
	}).fold(String::new(),|mut o, s|{o.push_str(&s);o});

	println!("{}", s);
}

