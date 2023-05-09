/* Moving Zeros To The End
   Write an algorithm that takes an array and moves all of the zeros to the end, preserving the order of the other elements.
*/

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut non_zero:Vec<u8> = arr.iter().filter(|i| i != (&&0u8)).cloned().collect();
	let mut zeroes: Vec<u8> = arr.iter().filter(|i| i == (&&0u8)).cloned().collect();

	non_zero.append(&mut zeroes);

	non_zero
}

fn dotest(a: &[u8], expected: &[u8]) {
    let actual = move_zeros(a);
	if actual == expected {
		println!("OK");
	} else {
		println!("Expected {expected:?} but got {actual:?}");
	}
}

fn main() {
    dotest(
        &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1],
        &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0],
    );
    dotest(
        &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1],
        &[9, 9, 1, 2, 1, 1, 3, 1, 0, 0, 0, 0, 0, 0],
    );
    dotest(
        &[0, 1, 2, 0, 3, 0, 0, 0, 0, 0, 0],
        &[1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0],
    );
    dotest(&[0, 0], &[0, 0]);
    dotest(&[], &[]);
}
