#[derive(Debug, Clone, Copy)]
pub struct Clock {
    pub name: &'static str,
}

#[derive(Debug)]
pub struct Precedes {
    left: Clock,
    right: Clock,
    delta: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Alternates {
    pub left: Clock,
    pub right: Clock,
    last_left: bool,
    last_right: bool,
}

// TODO: define a macro to build an state-machine
// from these relationships
impl Precedes {
    pub fn new(left: &'static str, right: &'static str) -> Self {
        Self {
            left: Clock { name: left },
            right: Clock { name: right },
            delta: 0,
        }
    }

    // precedes relationship does not allow both left and right nth tick happening at the same the
    // time
    pub fn evaluate(&mut self, left: bool, right: bool) -> bool {
        if self.delta == 0 && right {
            return false;
        }
        if left {
            self.delta += 1;
        }
        if right {
            self.delta -= 1;
        }
        true
    }
}

impl Alternates {
    pub fn new(left: &'static str, right: &'static str) -> Self {
        Self {
            left: Clock { name: left },
            right: Clock { name: right },
            last_left: false,
            // in the past the last tick was at the right
            last_right: true,
        }
    }

    pub fn evaluate(&mut self, left: bool, right: bool) -> bool {
        // If both clocks tick at the same time, the alternatesWith condition is violated
        if left && right {
            return false;
        }

        // Check if clock_a ticks after clock_b, or vice versa
        if (left && !self.last_left && self.last_right)
            || (right && !self.last_right && self.last_left)
        {
            self.last_left = left; // Update the last state of clock a
            self.last_right = right; // Update the last state of clock b
            return true;
        }

        // If no valid alternation is detected, update states but return false
        self.last_left = left; // Update the last state of clock a
        self.last_right = right; // Update the last state of clock b
        return false;
    }
}
