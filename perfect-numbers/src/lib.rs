#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}
use Classification::*;

pub fn classify(num: u64) -> Option<Classification> {
	// special case for 0 and 1
	if num == 0 {return None}
    if num == 1 {return Some(Deficient)}
	
	// Again, using brute force as my approach.
	// As it is simple and easy to understand at
	// the cost of high time complexity on
	// BIG integer number
	let mut aliquot_sum = 0;
	for i in 1..num {
		if num % i == 0 {
			aliquot_sum += i;
		}
	}
	
	match aliquot_sum {
		alqs if alqs == num => Some(Perfect),
		alqs if alqs > num => Some(Abundant),
		alqs if alqs < num => Some(Deficient),
		_ => None,
	}
} 	
