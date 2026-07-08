//! All the content in one place. Copy is data; data lives here.
//!
//! The trip dates are flexible (±1 day, hostage to flight prices); the cities
//! are not. When dates shift, this file is the only thing that changes.

pub const TRIP_YEAR: i32 = 2026;
pub const TRIP_MONTH: u32 = 9;

/// Her birthday, day-of-month within the trip. Falls mid-trip. Not a coincidence
/// the site keeps bringing it up.
pub const HER_BIRTHDAY: u32 = 26;

/// One leg of the trip. `days` is which September days the availability screen
/// files under this leg (boundary/travel days get assigned to exactly one leg);
/// the Quebec leg owns no days — nobody is asking her to be free in Quebec. Yet.
#[derive(Clone, Copy)]
pub struct Leg {
    pub emoji: &'static str,
    pub name: &'static str,
    pub dates: &'static str,
    pub days: &'static [u32],
    pub note: &'static str,
    pub tag: Option<&'static str>,
}

pub const LEGS: [Leg; 4] = [
    Leg {
        emoji: "🏔️",
        name: "monterrey",
        dates: "sept 18 – 21",
        days: &[18, 19, 20],
        note: "first stop. big mountains, bigger tacos, and me stretching like it's a competitive sport.",
        tag: None,
    },
    Leg {
        emoji: "🌆",
        name: "mexico city & puebla",
        dates: "sept 21 – 28",
        days: &[21, 22, 23, 24, 25, 26, 27],
        note: "the main event. word on the street is puebla has this one really cool pedestrian.",
        tag: Some("📍 your stop"),
    },
    Leg {
        emoji: "🌄",
        name: "san cristóbal, chiapas",
        dates: "sept 28 – 30",
        days: &[28, 29, 30],
        note: "cobblestones, coffee, morning mist. very cinematic. currently casting a co-star.",
        tag: None,
    },
    Leg {
        emoji: "🍁",
        name: "back to quebec",
        dates: "sept 30",
        days: &[],
        note: "retour à la base. re-entering canadian autumn against my will.",
        tag: None,
    },
];

#[derive(Clone, Copy, PartialEq)]
pub struct Choice {
    pub id: &'static str,
    pub emoji: &'static str,
    pub name: &'static str,
    pub note: &'static str,
}

pub const JOINS: [Choice; 5] = [
    Choice {
        id: "puebla",
        emoji: "🌮",
        name: "hangs in puebla",
        note: "i come to you. walking shoes, dad jokes, and a frankly irresponsible amount of enthusiasm.",
    },
    Choice {
        id: "cdmx",
        emoji: "🏙️",
        name: "a mexico city day",
        note: "museums, tacos al pastor, me mispronouncing chapultepec with total confidence.",
    },
    Choice {
        id: "mty",
        emoji: "🏔️",
        name: "the monterrey leg",
        note: "mountains that show off. i respect that in a landscape.",
    },
    Choice {
        id: "sancris",
        emoji: "🌄",
        name: "the san cristóbal leg",
        note: "48 hours of cobblestones and coffee in chiapas. very cinematic, remember.",
    },
    Choice {
        id: "elsewhere",
        emoji: "🗺️",
        name: "somewhere else entirely",
        note: "you name the place, i handle logistics. i'm excellent at logistics. this website is exhibit A.",
    },
];

pub const VIBES: [Choice; 6] = [
    Choice {
        id: "tacos",
        emoji: "🌮",
        name: "tacos, obviously",
        note: "zero seafood. fish are friends, not food.",
    },
    Choice {
        id: "walk",
        emoji: "🚶",
        name: "a long walk",
        note: "the classics are classics for a reason.",
    },
    Choice {
        id: "dance",
        emoji: "💃",
        name: "salsa & bachata",
        note: "i lead, you laugh at the state of my footwork.",
    },
    Choice {
        id: "pool",
        emoji: "🎱",
        name: "billiards",
        note: "i'll go easy on you. that's still a lie.",
    },
    Choice {
        id: "churros",
        emoji: "🍫",
        name: "churros & café",
        note: "dessert as a personality trait. golden, like someone's favourite color.",
    },
    Choice {
        id: "surprise",
        emoji: "🎲",
        name: "surprise me",
        note: "bold. i've been drafting hypothetical itineraries since july.",
    },
];
