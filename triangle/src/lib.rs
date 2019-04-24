pub struct Triangle {
	a: u64,
	b: u64,
	c: u64,
}
	
impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
		if sides[0] == 0 || sides[1] == 0 || sides[2] == 0 {None}
		else if sides[0] + sides[1] < sides[2] || sides[1] + sides[2] < sides[0] || sides[0] + sides[1] < sides[2] {None}
		else {
			Some(Triangle{a: sides[0], b: sides[1], c: sides[2]})
		}
        //unimplemented!("Construct new Triangle from following sides: {:?}. Return None if the sides are invalid.", sides);
    }

    pub fn is_equilateral(&self) -> bool {
		self.a == self.b && self.b == self.c && self.c == self.a
		// unimplemented!("Determine if the Triangle is equilateral.");
    }

    pub fn is_scalene(&self) -> bool {
		self.a != self.b && self.b != self.c && self.c != self.a
        // unimplemented!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.c == self.a
		// unimplemented!("Determine if the Triangle is isosceles.");
    }
}
