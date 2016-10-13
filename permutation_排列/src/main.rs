fn main() {
	let result :Vec<String> = (123..329).filter_map(help).map(|(a,b,c)| format!("{} {} {}\n", a, b, c)).collect();
	print!("{}", result.concat());
}

fn help(a: i32) -> Option<(i32,i32,i32)>{
	let b = 2 * a;
	let c = 3 * a;

	let (suma,muta) = split(a);
	let (sumb, mutb) = split(b);
	let (sumc, mutc) = split(c);

	if suma + sumb +sumc == 45 && muta * mutb*mutc == 362880 {
		return Some((a,b,c));
	}

	None
}

fn split(num :i32) -> (i32, i32) {
	let a = num / 100;
	let b = num /10 % 10;
	let c = num % 10;

	let sum = a + b + c;
	let mul = a*b*c;

	(sum, mul)
}