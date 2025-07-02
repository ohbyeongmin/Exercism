pub fn raindrops(n: u32) -> String {
    RainDrops::new(n).pling().plang().plong().finish()
}

struct RainDrops {
    n: u32,
    sounds: String,
}

impl RainDrops {
    fn new(n: u32) -> Self {
        Self {
            n,
            sounds: String::from(""),
        }
    }

    fn pling(mut self) -> Self {
        if self.n % 3 == 0 {
            self.sounds.push_str("Pling");
        }
        self
    }

    fn plang(mut self) -> Self {
        if self.n % 5 == 0 {
            self.sounds.push_str("Plang");
        }
        self
    }

    fn plong(mut self) -> Self {
        if self.n % 7 == 0 {
            self.sounds.push_str("Plong");
        }
        self
    }

    fn finish(self) -> String {
        if self.sounds.is_empty() {
            self.n.to_string()
        } else {
            self.sounds
        }
    }
}
