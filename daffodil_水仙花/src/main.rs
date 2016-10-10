fn main() {
	let result :Vec<i32> = (100..1000)
		// .map(split_to_abc)
		// .filter(|s| s.is_some())
		.filter_map(split_to_abc)
		// .flat_map(|o| o)
		.collect();

	for i in 0.. result.len() {
		print!("{} ", result[i]);
		if i == 10 {
			println!("");
		}
	}
	println!("");
}

fn split_to_abc(num :i32) -> Option<i32> {
	if num < 100 || num >= 1000 {
		return None;
	}

	let a = num/100;
	let b = (num%100) / 10;
	let c = num % 10;

	if num == a.pow(3) + b.pow(3) + c.pow(3){
		return Some(num);
	}
	None
}

#[test]
fn test_split() {
	assert_eq!(split_to_abc(99), None);
	assert_eq!(split_to_abc(153),Some(153));
}