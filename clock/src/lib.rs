use std::fmt;

pub struct Clock {
	hours: i32,
	minutes: i32,
}

impl Clock {
	
    pub fn new(hours: i32, minutes: i32) -> Self {	
		let convert_minutes = (hours * 60) + minutes;
		let mut hours = (convert_minutes / 60) % 24;
		let mut minutes = convert_minutes % 60;
		
		if hours < 0 || minutes < 0 {
			hours = 23 - hours.abs();
			minutes = 60 - minutes.abs();
			Clock::new(hours, minutes)
		} else {
			Clock{hours: hours, minutes: minutes}
		}
		
    }
	
    pub fn add_minutes(&self, minutes: i32) -> Self {
		Clock::new(self.hours, self.minutes + minutes)
		// unimplemented!("Add {} minutes to existing Clock time", minutes);
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}