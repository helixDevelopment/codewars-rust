/* Sum of odd numbers
   Given the triangle of consecutive odd numbers:

						1
					3     5
				7     9    11
			13    15    17    19
		21    23    25    27    29
	31	 33	  35	37		39		41
   ...
   Calculate the sum of the numbers in the nth row of this triangle (starting at index 1) e.g.: (Input --> Output)

   1 -->  1
   2 --> 3 + 5 = 8
*/

fn row_sum_odd_numbers(n: i64) -> i64 {
	// turns out the sum of row is just "n * n * n" but here's my other solution:
	if n <= 1 { return n; }
	let mut sum: i64 = 1;
	let mut adder = 0;

	for _ in 0..(n-1) {
		adder += 2;
		sum += adder;
	}

	adder = sum;
	for _ in 0..(n-1) {
		adder += 2;
		sum += adder;
	}
	
	sum
}

fn main() {
    println!("{} should equal 1", row_sum_odd_numbers(1));
    println!("{} should equal 8", row_sum_odd_numbers(2));
    println!("{} should equal 27", row_sum_odd_numbers(3));
	println!("{} should equal 74088", row_sum_odd_numbers(42));
}
