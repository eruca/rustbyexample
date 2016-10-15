use std::io;
use std::collections::HashSet;

fn main() {
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_) => {},
		Err(e) => {println!("{:?}", e);return;}
	}

	let number:HashSet<u32> = input.as_str().trim().chars().filter_map(|c| c.to_digit(10)).collect();

	let result : Vec<Vec<String>> = (111..999).filter_map(move |abc| {
		if !num_in_hashset(abc, &number){
			return None;
		}
		let item : Vec<String> = (11..99).filter_map(|de| {
			let de = de as u32;

			if !num_in_hashset(de, &number){
				return None;
			}

			let (left, right) = (de / 10, de % 10);
			let sum1 = left * abc;
			if !num_in_hashset(sum1, &number) {
				return None;
			}
			let sum2 = right * abc;
			if !num_in_hashset(sum2, &number) {
				return None;
			}

			let sum = sum1 + sum2 * 10;
			if !num_in_hashset(sum, &number){
				return None;
			}

			let mut out = String::new();
			out.push_str(&format!("  {}\nX  {}\n-----\n {}\n{} \n-----\n{}", abc, de, sum1, sum2, sum));

			Some(out)
		}).collect();

		if item.len() == 0 {
			return None;
		}

		Some(item)
	}).collect();

	let (_, out) = result.into_iter().fold((0,String::new()),|(len, mut out), v|{
		let v_len = v.len();
		let out = v.into_iter().enumerate().fold(out,|mut out, (k, item)|{
			out.push_str(&format!("<{}>\n{}\n", len + k,item));
			out
		});


		(len + v_len, out)
	});

	println!("{}", out);
}

fn num_in_hashset(mut n:u32, set:&HashSet<u32>) -> bool {
	loop{
		if !set.contains(&(n % 10)) {
			return false;
		}
		n = n / 10;
		if n == 0{
			return true;
		}
	}
}

#[test]
fn test_num_in_hashset(){
	let mut set = HashSet::new();
	set.insert(2);
	set.insert(3);
	set.insert(4);
	assert!(num_in_hashset(432,&set));
}