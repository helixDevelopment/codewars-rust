/* Josephus Survivor
   In this kata you have to correctly return who is the "survivor", ie: the last element of a Josephus permutation.

   Basically you have to assume that n people are put into a circle and that they are eliminated in steps of k elements, like this:

   n=7, k=3 => means 7 people in a circle
   one every 3 is eliminated until one remains
   [1,2,3,4,5,6,7] - initial sequence
   [1,2,4,5,6,7] => 3 is counted out
   [1,2,4,5,7] => 6 is counted out
   [1,4,5,7] => 2 is counted out
   [1,4,5] => 7 is counted out
   [1,4] => 5 is counted out
   [4] => 1 counted out, 4 is the last element - the survivor!
*/

use std::vec::Vec;

fn josephus_survivor(n: i32, k: i32) -> i32 {
    let mut nums: Vec<i32> = Vec::new();
    let mut index = 0;

    for num in 0..n {
        nums.push(num + 1);
    }

    while nums.len() >= 2 {
        index += k - 1;
        index = index % nums.len() as i32;

        nums.remove(index as usize);
    }

    return nums[0];
}

fn main() {
    println!("1. {} should be 4", josephus_survivor(7, 3));
    println!("2. {} should be 10", josephus_survivor(11, 19));
    println!("3. {} should be 28", josephus_survivor(40, 3));
    println!("4. {} should be 13", josephus_survivor(14, 2));
}
