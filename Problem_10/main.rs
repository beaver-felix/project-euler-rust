fn main() {
	let mut sum:u64 = 0;
	let n:usize = 2_000_000;
	
	let mut arr = vec![true; n as usize];
	arr[0] = false; arr[1] = false;
	
	for i in 2..n {
		if !arr[i] {continue; }
		sum += i as u64;
		if let Some(start) = i.checked_mul(i) {
			if start >= n {continue; }
			for j in (start..n).step_by(i) {arr[j] = false; }
		}
	}
	println!("{}", sum);
}
