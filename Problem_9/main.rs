fn main() {
	for i in 32..1000 {
		let num:u32 = i * i;
		let den:u32 = 1000 - i;
		if num % den == 0 {
			let delta: u32 = num / den;
			let b:u32 = (1_000 - delta - i) / 2;
			let c:u32 = (1_000 + delta - i) / 2;
			println!("{}", i * b * c);
			return;
		}
	}
}
