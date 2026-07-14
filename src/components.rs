//! Reusable pieces: the tappable day strip and the confetti layer.

use std::collections::BTreeMap;

use leptos::prelude::*;

use crate::data::{BUSY_DAYS, BUSY_NOTE, HER_BIRTHDAY, TRIP_MONTH, TRIP_YEAR};
use crate::util::{Avail, Date, Rng, WEEKDAYS_SHORT};

/// A row of tappable days. Tap cycles unmarked → free → maybe → unmarked, so
/// nothing she does is ever more than one tap from undone. State is lifted to
/// the parent's map; the strip owns nothing. Days in `BUSY_DAYS` render greyed
/// and ignore taps; hovering the strip's busy cells reveals `BUSY_NOTE`
/// (a CSS tooltip fed by the `data-busy` attribute).
#[component]
pub fn DayStrip(days: &'static [u32], avail: RwSignal<BTreeMap<u32, Avail>>) -> impl IntoView {
    let cells = days
        .iter()
        .map(|&d| {
            let dw = WEEKDAYS_SHORT[Date {
                y: TRIP_YEAR,
                m: TRIP_MONTH,
                d,
            }
            .day_of_week() as usize];
            let bday = d == HER_BIRTHDAY;
            let busy = BUSY_DAYS.contains(&d);
            view! {
                <button
                    class="day"
                    class:free=move || avail.with(|m| m.get(&d) == Some(&Avail::Free))
                    class:maybe=move || avail.with(|m| m.get(&d) == Some(&Avail::Maybe))
                    class:birthday=bday
                    class:busy=busy
                    aria-disabled=busy.then_some("true")
                    title=if bday { Some("your birthday 🎂") } else { None }
                    on:click=move |_| {
                        if busy {
                            return;
                        }
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

    let has_busy = days.iter().any(|d| BUSY_DAYS.contains(d));
    view! { <div class="dstrip" data-busy=has_busy.then_some(BUSY_NOTE)>{cells}</div> }
}

/// The saved override if one exists, else the OS preference.
fn initial_dark() -> bool {
    let win = web_sys::window().expect("no window? concerning");
    if let Ok(Some(store)) = win.local_storage() {
        if let Ok(Some(saved)) = store.get_item("theme") {
            return saved == "dark";
        }
    }
    win.match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false)
}

fn apply_theme(dark: bool) {
    let win = web_sys::window().expect("no window? concerning");
    let theme = if dark { "dark" } else { "light" };
    if let Some(root) = win.document().and_then(|d| d.document_element()) {
        let _ = root.set_attribute("data-theme", theme);
    }
    if let Ok(Some(store)) = win.local_storage() {
        let _ = store.set_item("theme", theme);
    }
}

/// Sun/moon switch, fixed in the corner. Follows the OS theme until the first
/// click; after that the choice sticks (localStorage). The CSS `data-theme`
/// blocks do the actual re-painting.
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let dark = RwSignal::new(initial_dark());

    // A returning visitor with a saved choice gets it re-stamped on load;
    // everyone else keeps following the OS until they click.
    let win = web_sys::window().expect("no window? concerning");
    if let Ok(Some(store)) = win.local_storage() {
        if let Ok(Some(_)) = store.get_item("theme") {
            apply_theme(dark.get_untracked());
        }
    }

    view! {
        <button
            class="theme-btn"
            title="lights"
            aria-label="toggle dark mode"
            on:click=move |_| {
                let next = !dark.get_untracked();
                dark.set(next);
                apply_theme(next);
            }
        >
            {move || if dark.get() { "☀️" } else { "🌙" }}
        </button>
    }
}

/// Falling sparkles and dots. Generated once with the xorshift RNG; CSS does
/// the animation, so after mount this costs the CPU nothing.
#[component]
pub fn Confetti() -> impl IntoView {
    // Talavera mid-tones — inline styles can't follow the CSS theme, so these
    // are picked to read on both the glaze-white and cobalt-night grounds.
    const HUES: [&str; 4] = ["#3A6CC4", "#6FAE8B", "#9BB8E3", "#D9A62E"];
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
