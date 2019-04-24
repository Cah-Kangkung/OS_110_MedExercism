# OS_110_MedExercism
This repository is for my assignment on Operating System course at State University of Jakarta. It is about completing five medium problem in Exercism's rust.

i have completed about 7 medium problems for now, here is the list:
- clock
- hamming
- isbn-verifier
- pascals-triangle
- perfect-numbers
- scrabble-score
- triangle

Now i will explain on how i solved ISBN Verifier problem
___

# ISBN Verifier

## The Problem
ISBN Verifier as explained on exercism, we have to determines wheter a string is a valid isbn number or not using this formula:
`(x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11 == 0`  

Some constraints applied, such as the string must be exactly 10 characters in lenght (without dashes), we could only take integer and the last character could be an 'X' wich represent 10.

## My Solution
I've managed to solve the problem with something like:
1. Remove all the "-" (dashes) with `.replace()`.
2. Define the boundaries, wich is `string.len() == 10`.
3. Declare variable `called sum_of_multiplie` that acts as a result.
4. `.chars()` and `.enumerate()` to iterate over the string wich produce (i, c) where `i` is the index and `c` is the char element.
5. Next is to check whether our char element is a legit decimal(10) digit or not using `.is_digit()`. if it is valid, implement our formula.
6. If the last element is `'X'`, simply add `10` to the result
7. return true if `result % 11 == 0`.

Here is the code:
```Rust
pub fn is_valid_isbn(isbn: &str) -> bool {
	// remove dash and blank space
	let raw_string = isbn.replace("-", "");
	if raw_string.len() != 10 {return false}
	
	// .chars() returns an iterator over 'character' of a string slice
	// .enumerate() creates an iterator of count
	// that pairs with the current iteration, returns as usize
	let mut sum_of_product = 0;
	for (i, c) in raw_string.chars().enumerate() {
		if c.is_digit(10) {
			sum_of_product += (11-(i+1)) as u32 * c.to_string() // these method convert our char element into int
							       .parse::<u32>()
							       .unwrap();
		}
		else if i == 9 && c == 'X' {
			sum_of_product += 10;
		}
		else {
			return false
		}
	}
	sum_of_product % 11 == 0
}
```

We could also use approach like converting the string into a list of chars first, then iterate over them if you ever wish to use array indexing instead

Like this:
```Rust
pub fn is_valid_isbn(isbn: &str) -> bool {
  	// remove dash and blank space
	let raw_string = isbn.replace("-", "");
	if raw_string.len() != 10 {return false}
	
	// convert string into list of chars
	let list_chars: Vec<char> = raw_string.chars().collect();
	
	let mut sum_of_product = 0;
	for i in (1..=list_chars.len()).rev() {
		let element = list_chars[10-i]; // bind char element into variable
		if element.is_digit(10) { // determine if the element is a legit decimal(10) digit or not
			sum_of_product += element.to_string()
						 .parse::<u32>()
						 .unwrap() 
						 * i as u32;
		}		
		else if i == 1 && element == 'X' {
			sum_of_product += 10;
		}
		else {
			return false
		}
	}
	sum_of_product % 11 == 0
```

## Summary
For summary, i choosed this problem because i found it's the most fun out of all the problems i solved. Playing with string in rust is not easy (based on my amateur experience on python and java) so this challange do really play part on my experience understanding rust better. (Ps : sorry for my broken english, believe me rust isn't the only thing i'm improving :) )
