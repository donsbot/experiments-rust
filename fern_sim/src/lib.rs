// data is public
pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

// methods "attached" to Fern
impl Fern {
    // growth fun is public
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

// functional style seems simpler??
pub fn grow_f(fern: &mut Fern) {
    fern.size *= 1.0 + fern.growth_rate;
}

pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0 .. days {
        fern.grow();
    }
}
