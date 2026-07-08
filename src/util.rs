//! Date math and randomness, from first principles.
//!
//! No `chrono`, no `rand` — house rules carried over from the last build.
//! A scheduling site that computes its own weekdays felt thematically correct.

/// A calendar date. Month is 1–12, day is 1–31.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Date {
    pub y: i32,
    pub m: u32,
    pub d: u32,
}

pub const WEEKDAYS_SHORT: [&str; 7] = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];

impl Date {
    /// Day of week, 0 = Sunday, via Zeller's congruence.
    ///
    /// The trick: treat Jan/Feb as months 13/14 of the *previous* year so the
    /// leap day lands at the end of the counting year, then everything reduces
    /// to integer arithmetic mod 7. Zeller's raw output is 0 = Saturday, so we
    /// rotate by 6 to get the civil convention.
    pub fn day_of_week(self) -> u32 {
        let (m, y) = if self.m < 3 {
            (self.m + 12, self.y - 1)
        } else {
            (self.m, self.y)
        };
        let k = y % 100; // year within century
        let j = y / 100; // century
        let h = (self.d as i32 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        ((h + 6) % 7) as u32
    }
}

/// How free a day is. A day absent from the map is simply unmarked.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Avail {
    Free,
    Maybe,
}

/// Compress sorted day numbers into human ranges: [18,19,20,25] → "18–20, 25".
pub fn compress_days(days: &[u32]) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    while i < days.len() {
        let start = days[i];
        let mut end = start;
        while i + 1 < days.len() && days[i + 1] == end + 1 {
            i += 1;
            end = days[i];
        }
        parts.push(if start == end {
            format!("{start}")
        } else {
            format!("{start}–{end}")
        });
        i += 1;
    }
    parts.join(", ")
}

/// xorshift64* — a full-period PRNG in three shifts and a multiply.
///
/// Each xorshift smears bits across the word; the odd multiplier scrambles
/// the linear structure. Not cryptographic — but the confetti doesn't need
/// to keep secrets.
pub struct Rng(u64);

impl Rng {
    pub fn from_clock() -> Self {
        // Seed from milliseconds since epoch; the |1 guards against a zero
        // state, which xorshift can never escape.
        Rng((js_sys::Date::now() as u64) | 1)
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    /// Uniform-ish f64 in [0, 1).
    pub fn f64(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Uniform-ish f64 in [lo, hi).
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.f64() * (hi - lo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeller_knows_the_trip() {
        // Known anchor:
        assert_eq!(Date { y: 2000, m: 1, d: 1 }.day_of_week(), 6); // Saturday
        // Landing day:
        assert_eq!(Date { y: 2026, m: 9, d: 18 }.day_of_week(), 5); // Friday
        // Her birthday, mid-trip. A Saturday. The universe is cooperating.
        assert_eq!(Date { y: 2026, m: 9, d: 26 }.day_of_week(), 6);
    }

    #[test]
    fn ranges_compress() {
        assert_eq!(compress_days(&[]), "");
        assert_eq!(compress_days(&[26]), "26");
        assert_eq!(compress_days(&[18, 19, 20, 25]), "18–20, 25");
        assert_eq!(compress_days(&[21, 22, 24, 25, 26, 30]), "21–22, 24–26, 30");
    }
}
