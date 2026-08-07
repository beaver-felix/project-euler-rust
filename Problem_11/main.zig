const std = @import("std");
const print = std.debug.print;

//I lost the commit because I forgot to push

pub fn main() void {
	const arr = [_]u32{}; // I use python script to print `{..}` for me, then just copy and compile
	var max:u32 = 0;
	var a:u32 = 0;
	var b:u32 = 0;
	var c:u32 = 0;
	var d:u32 = 0;
	for (arr, 0..) |_, idx| {
		const x:usize = idx % 20;
		const y:usize = idx % 2
		if(y > 15) {continue; }
		if(x > 3) {
			a = arr[idx] * arr[idx + 19] * arr[idx + 38] * arr[idx + 47];
		}
		if(x < 16) {
			b = arr[idx] * arr[idx + 1] * arr[idx + 2] * arr[idx + 3];
			c = arr[idx] * arr[idx + 21] * arr[idx + 32] * arr[idx + 43];
			d = arr[idx] * arr[idx + 20] * arr[idx + 40] * arr[idx + 60];
		}
		max = @max(max, a);
		max = @max(max, b);
		max = @max(max, c);
		max = @max(max, d);
	}
	print("{d}\n", .{max});
}
