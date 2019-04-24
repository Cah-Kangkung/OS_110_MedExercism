/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
	
	let word = word.to_lowercase();
	let mut result = 0;
	for i in word.chars() {
		match i {
			'a' => result += 1,
			'e' => result += 1, 
			'i' => result += 1, 
			'o' => result += 1, 
			'u' => result += 1, 
			'l' => result += 1, 
			'n' => result += 1, 
			'r' => result += 1, 
			's' => result += 1, 
			't' => result += 1,
			'd' => result += 2,
			'g' => result += 2,
			'b' => result += 3, 
			'c' => result += 3,
			'm' => result += 3, 
			'p' => result += 3,
			'f' => result += 4,
			'h' => result += 4,
			'v' => result += 4,
			'w' => result += 4,
			'y' => result += 4,
			'k' => result += 5,
			'j' => result += 8,
			'x' => result += 8,
			'q' => result += 10,
			'z' => result += 10,
			_ => result += 0,
		}
	}
	result
}


