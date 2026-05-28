use std::io::stdin;
pub fn input() -> String {
	let mut buf = String::new();
	stdin().read_line(&mut buf)
		.expect("input error");
	return buf.trim().into();
}