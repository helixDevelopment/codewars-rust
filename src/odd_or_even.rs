/* Odd or Even
   Given a list of integers, determine whether the sum of its elements is odd or even.

   Give your answer as a string matching "odd" or "even".

   If the input array is empty consider it as: [0] (array with a zero).
*/

fn odd_or_even(numbers: Vec<i32>) -> String {
	let sum: i32 = numbers.iter().sum();

	if sum % 2 == 0 {
		return "even".to_string();
	} else {
		return "odd".to_string();
	}
}

fn main() {
    println!("1. {} should be even", odd_or_even(vec![]));
    println!("1. {} should be even", odd_or_even(vec![0]));
    println!("2. {} should be odd", odd_or_even(vec![1]));
    println!("3. {} should be even", odd_or_even(vec![0, 1, 5]));
}
