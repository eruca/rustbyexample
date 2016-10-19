const MAX :i32 = 100000;

fn main() {
	// let mut result = vec[0;100000];
	let result = (0..MAX+1).fold(vec![0;(MAX +1) as usize],|mut v, n| {
		match fm(n) {
			Some(tuple) => {
				if v[tuple.0] == 0 || v[tuple.0] > tuple.1{
					v[tuple.0] = tuple.1
				}
			},
			None =>{},
		};
		v
	});

	println!("{},{},{}", result[216],result[121],result[2005]);
}

fn fm(n: i32) -> Option<(usize, i32)> {
	let sum = sum(n);
	if sum <= MAX {
		return Some((sum as usize, n));
	}
	None
}

fn sum(n: i32) -> i32 {
	let (mut sum, mut i) = (0, n);

	loop {
		let res = i % 10;
		if res == 0 && i == 0 {
			return sum + n;
		}
		sum += res;

		i = i / 10;
	}
}

#[test]
fn test_sum() {
	assert_eq!(sum(123), 6+123);
	assert_eq!(sum(112023), 9+112023);
}
