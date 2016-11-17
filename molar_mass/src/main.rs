use std::io;

const C:f64 = 12.01;
const H:f64 = 1.008;
const O:f64 = 16.00;
const N:f64 = 14.01;

const UNIT:&'static str = "g/mol";

#[derive(Debug)]
struct Tuple (char,Vec<u32>);

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
    	Ok(_) => {},
    	Err(e) => {println!("{:?}", e);return;},
    }

    let trim_input = input.trim();

    if !trim_input.chars().all(|c| c=='C'|| c=='H' || c == 'O' || c == 'N' || c.is_digit(10)) {
    	println!("输入必须是CHON或者是数字");
    	return;
    }

    let result = trim_input.chars().fold(Vec::new(), |mut v, c|{
    	if c == 'H' || c == 'O' || c == 'N' || c == 'C' {
    		v.push(Tuple(c,Vec::new()));
    		return v;
    	}

    	let length = v.len();
    	if c.is_digit(10) {
    		let digit =match c.to_digit(10) {
    			Some(d) => d,
    			None => 0,
    		};

    		v[length-1].1.push(digit);
    	}
    	v
    });

    let sum = result.into_iter().fold(0.0,|sum, t|{
    	let Tuple(c, v) = t;

    	let count = match v.len() {
    		0 => 1,
    		_ => get_sum(&v),
    	};

    	sum + match c {
    		'C' => C ,
    		'H' => H,
    		'O' => O,
    		'N' => N,
    		_ => 0.0,
    	} * count as f64
    });

    println!("{}{}", sum, UNIT);
}

fn get_sum(v:&Vec<u32>) -> u32 {
	v.iter().rev().enumerate().fold(0, |mut sum,(k,&v)| {sum += v * 10u32.pow(k as u32);sum})
}

#[test]
fn test_get_sum() {
	let v = vec![1,2,3];
	assert_eq!(get_sum(&v),123);
}