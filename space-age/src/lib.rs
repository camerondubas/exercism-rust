// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        const SECONDS: f64 = 60.;
        const MINUTES: f64 = 60.;
        const HOURS: f64 = 24.;
        const DAYS: f64 = 365.25;

        Duration {
            years: s as f64 / (SECONDS * MINUTES * HOURS * DAYS),
        }
    }
}

pub trait Planet {
    const EARTH_YEARS: f64 = 1.;

    fn years_during(d: &Duration) -> f64 {
        d.years / Self::EARTH_YEARS
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! planet {
    ($planet:ident, $year_length:expr) => {
        impl Planet for $planet {
            const EARTH_YEARS: f64 = $year_length;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
