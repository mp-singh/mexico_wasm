//! Reusable pieces: the tappable day strip and the confetti layer.

use std::collections::BTreeMap;

use leptos::prelude::*;

use crate::data::{HER_BIRTHDAY, TRIP_MONTH, TRIP_YEAR};
use crate::util::{Avail, Date, Rng, WEEKDAYS_SHORT};

/// A row of tappable days. Tap cycles unmarked → free → maybe → unmarked, so
/// nothing she does is ever more than one tap from undone. State is lifted to
/// the parent's map; the strip owns nothing.
#[component]
pub fn DayStrip(days: &'static [u32], avail: RwSignal<BTreeMap<u32, Avail>>) -> impl IntoView {
    let cells = days
        .iter()
        .map(|&d| {
            let dw = WEEKDAYS_SHORT[Date { y: TRIP_YEAR, m: TRIP_MONTH, d }.day_of_week() as usize];
            let bday = d == HER_BIRTHDAY;
            view! {
                <button
                    class="day"
                    class:free=move || avail.with(|m| m.get(&d) == Some(&Avail::Free))
                    class:maybe=move || avail.with(|m| m.get(&d) == Some(&Avail::Maybe))
                    class:birthday=bday
                    title=if bday { Some("your birthday 🎂") } else { None }
                    on:click=move |_| {
                        avail.update(|m| match m.get(&d).copied() {
                            None => {
                                m.insert(d, Avail::Free);
                            }
                            Some(Avail::Free) => {
                                m.insert(d, Avail::Maybe);
                            }
                            Some(Avail::Maybe) => {
                                m.remove(&d);
                            }
                        })
                    }
                >
                    <span class="dw">{dw}</span>
                    <span class="dn">{d}</span>
                    {bday.then(|| view! { <span class="cake" aria-hidden="true">"🎂"</span> })}
                </button>
            }
        })
        .collect_view();

    view! { <div class="dstrip">{cells}</div> }
}

/// Falling sparkles and dots. Generated once with the xorshift RNG; CSS does
/// the animation, so after mount this costs the CPU nothing.
#[component]
pub fn Confetti() -> impl IntoView {
    const HUES: [&str; 4] = ["#D23369", "#2E9E9B", "#F4A9C4", "#C9A24B"];
    let mut rng = Rng::from_clock();

    let bits = (0..36)
        .map(|i| {
            let left = rng.range(0.0, 100.0);
            let delay = rng.range(0.0, 1.6);
            let dur = rng.range(2.6, 4.6);
            let size = rng.range(6.0, 14.0);
            if i % 6 == 0 {
                view! {
                    <span
                        class="bit"
                        style=format!(
                            "left:{left:.1}%;animation-delay:{delay:.2}s;animation-duration:{dur:.2}s;font-size:{:.0}px",
                            size + 8.0
                        )
                    >
                        "✨"
                    </span>
                }
                .into_any()
            } else {
                let hue = HUES[i % 4];
                view! {
                    <span
                        class="bit dot"
                        style=format!(
                            "left:{left:.1}%;animation-delay:{delay:.2}s;animation-duration:{dur:.2}s;width:{size:.0}px;height:{size:.0}px;background:{hue}"
                        )
                    ></span>
                }
                .into_any()
            }
        })
        .collect_view();

    view! { <div class="confetti" aria-hidden="true">{bits}</div> }
}
