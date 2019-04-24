/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // remove dash and blank space
	let raw_string = isbn.replace("-", "");
	if raw_string.len() != 10 {return false}
	
	let list_chars: Vec<char> = raw_string.chars().collect();
	
	let mut sum_of_product = 0;
	for i in (1..=list_chars.len()).rev() {
		let element = list_chars[10-i];
		if element.is_digit(10) {
			sum_of_product += element.to_string().parse::<u32>().unwrap() * i as u32;
		}		
		else if i == 1 && element == 'X' {
			sum_of_product += 10;
		}
		else {
			return false
		}
	}
	sum_of_product % 11 == 0
	
	/* chars() returns an iterator over 'character' of a string slice
	// enumerate() creates an iterator of count
	// that pairs with the current iteration, returns as usize
	let mut sum_of_product = 0;
	for (i, c) in raw_string.chars().enumerate() {
		if c.is_digit(10) {
			sum_of_product += (11-(i+1)) as u32 * c as u32;
		}
	}
	
	sum_of_product % 11 == 0
	*/
	//nimplemented!("Is {:?} a valid ISBN number?", isbn);
}
