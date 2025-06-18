// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const ONE_EARTH_YEAR: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury; // 7600543.8
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let one_mercury_year = ONE_EARTH_YEAR * 0.2408467;
        d.seconds / one_mercury_year
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let one_venus_year = ONE_EARTH_YEAR * 0.61519726;
        d.seconds / one_venus_year
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / ONE_EARTH_YEAR
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let one_mars_year = ONE_EARTH_YEAR * 1.8808158;
        d.seconds / one_mars_year
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let one_jupiter_year = ONE_EARTH_YEAR * 11.862615;
        d.seconds / one_jupiter_year
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let one_saturn_year = ONE_EARTH_YEAR * 29.447498;
        d.seconds / one_saturn_year
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let one_uranus_year = ONE_EARTH_YEAR * 84.016846;
        d.seconds / one_uranus_year
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let one_neptune_year = ONE_EARTH_YEAR * 164.79132;
        d.seconds / one_neptune_year
    }
}
