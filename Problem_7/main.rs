fn main() {
	let mut count = 0;
	let n:f64 = 10_001_f64;
	let higher = (n * (n.ln() + n.ln().ln())) as usize;

	let mut arr = vec![true; higher + 1];
	arr[0] = false; arr[1] = false;

	for i in 2..=higher {
		if !arr[i] {continue; }
		count += 1;
		if count == 10_001 {
			println!("{}", i);
			return;
		}
		if let Some(start) = i.checked_mul(i) {
			if start >= higher {continue; }
			for j in (start..=higher).step_by(i) {arr[j] = false; }
		}
	}
}
