/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
	if s1.len() != s2.len() {return None}
	
	// convert string to list of char for easy access (indexing)
	let s1: Vec<char> = s1.chars().collect();
	let s2: Vec<char> = s2.chars().collect();
	
	// iterate through list of char and add 1
	// to the result for every unsimiliarity
	// between s1 and s2
	let mut result = 0;
	for i in 0..(s1.len()) {
		if s1[i] != s2[i] {
			result += 1;
		}
	}
	Some(result)
}
