pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

pub trait Planet {
    fn year_in_earth_years() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEAR_IN_SECONDS * Self::year_in_earth_years())
    }
}

macro_rules! planet {
    ($name:ident, $year_in_earth_years:expr) => {
        pub struct $name;
        impl Planet for $name {
            fn year_in_earth_years() -> f64 {
                $year_in_earth_years
            }
        }
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
