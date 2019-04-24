pub struct PascalsTriangle{
	row_count: u32,
}
	
impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
		PascalsTriangle{row_count: row_count}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut pascal_triangle: Vec<Vec<u32>> = Vec::with_capacity(self.row_count as usize);
		
		// iterate through every line
		for line in 1..=self.row_count {
		
			let mut row: Vec<u32> = Vec::with_capacity(line as usize);
			let mut x = 1; // first element of every line is always 1
			
			// iterate through every element in row
			for i in 1..=line {
				row.push(x);
				x = x * (line - i) / i;
			}
			
			pascal_triangle.push(row)
		}
		pascal_triangle
    }
}
