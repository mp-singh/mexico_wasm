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

pub const JOINS: [Choice; 4] = [
    Choice {
        id: "puebla",
        emoji: "🌮",
        name: "hangs in puebla",
        note: "i come to you. you've been telling me about your world since quebec — i'd like to see it at walking pace.",
    },
    Choice {
        id: "cdmx",
        emoji: "🏙️",
        name: "a mexico city day",
        note: "nine million people, and i'd still only be watching one pedestrian cross the street.",
    },
    Choice {
        id: "sancris",
        emoji: "🌄",
        name: "the san cristóbal leg",
        note: "the closing leg. the sunsets are supposedly unreal. i wasn't planning on watching them alone.",
    },
    Choice {
        id: "elsewhere",
        emoji: "🗺️",
        name: "somewhere else entirely",
        note: "you pick. the last time i followed you somewhere random i ended up with a nickname and mild kidney trauma. i'd do it again.",
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
        note: "you have a documented history of kidney violence. i'll risk it.",
    },
    Choice {
        id: "dance",
        emoji: "💃",
        name: "salsa & bachata",
        note: "i lead. you laugh. the system works.",
    },
    Choice {
        id: "guide",
        emoji: "🧭",
        name: "i'll be the guide",
        note:
            "your country, your picks — show me what google doesn't know about. i'll gasp on cue.",
    },
    Choice {
        id: "churros",
        emoji: "🍫",
        name: "churros & café",
        note: "golden. someone's favourite color. i notice things.",
    },
    Choice {
        id: "surprise",
        emoji: "🎲",
        name: "surprise me",
        note: "bold. i've been drafting hypothetical itineraries since july.",
    },
];
