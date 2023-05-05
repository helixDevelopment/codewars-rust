/* Take a 10 Minutes Walk
	You live in the city of Cartesia where all roads are laid out in a perfect grid. You arrived ten
	minutes too early to an appointment, so you decided to take the opportunity to go for a short walk.
	The city provides its citizens with a Walk Generating App on their phones -- everytime you press
	the button it sends you an array of one-letter strings representing directions
	to walk (eg. ['n', 's', 'w', 'e']). You always walk only a single block for each letter (direction)
	and you know it takes you one minute to traverse one city block, so create a function that will
	return true if the walk the app gives you will take you exactly ten minutes (you don't want to be
	early or late!) and will, of course, return you to your starting point. Return false otherwise.
 */

fn is_valid_walk(walk: &[char]) -> bool {
	let mut vert = 0;
	let mut horz = 0;

	for c in walk {
		match c {
			'n' => vert += 1,
			's' => vert -= 1,
			'e' => horz -= 1,
			'w' => horz += 1,
			_ => {
				// ignore
			}
		}
	}

	vert == 0 && horz == 0 && walk.len() == 10
}

fn main() {
	println!("1. {} should be true", is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));
	println!("2. {} should be false", is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
}
