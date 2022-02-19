#![allow(non_snake_case)]
#![deny(unsafe_code)]

mod flashcards;
mod data;
mod note_input;

use data::UserData;
use dioxus::prelude::*;
use dioxus::fermi::{Atom, use_set, use_read};
use log::error;
use simplelog::*;

/// An atom containing the current showed main page
static CURRENT_PAGE: Atom<CurrentPage> = |_| CurrentPage::HomePage;
/// An atom containing the global user data
static USER_DATA: Atom<UserData> = |_| UserData::load().unwrap_or_else(|e| {
    error!("Could not load existing data: {}", e);
    UserData::default()
});

/// Represents current page states - matched on in the main app
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum CurrentPage {
    HomePage, Flashcards, NoteInput
}

#[derive(Props, PartialEq)]
struct PageLinkProps {
    /// Name of the button and page
    name: &'static str,
    /// What page to change to
    redirect: CurrentPage,
    /// Style of this link
    class: &'static str
}
/// A page link - a button to open another current page
fn PageLink(cx: Scope<PageLinkProps>) -> Element {
    let set_page = use_set(&cx, CURRENT_PAGE);
    cx.render(rsx! {
        button {
            "type": "button",
            class: "{cx.props.class}",
            onclick: move |_| {
                set_page(cx.props.redirect);
            },
            "{cx.props.name}"
        }
    })
}

/// The home page page - links to other pages
fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "center-div",
            h1 {"Magistrax (wip name)"},
            PageLink {
                class: "pagelink",
                name: "Flashcards",
                redirect: CurrentPage::Flashcards
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct FontProps {
    /// Link (given on fonts.google.com) to the font
    link: &'static str
}
/// Load a font from google fonts
fn Font(cx: Scope<FontProps>) -> Element {
    rsx!(cx,
        link {
            rel: "preconnect", href: "https://fonts.googleapis.com"
        },
        link {
            rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "true"
        },
        link {
            rel: "stylesheet", href: "{cx.props.link}"
        }
    )
}

/// Main app component - renders current page + basic frame
fn App(cx: Scope) -> Element {
    let read_page = use_read(&cx, CURRENT_PAGE);
    rsx! (cx, div {
        style {[include_str!("style.css")]},
        Font {link: "https://fonts.googleapis.com/css2?family=Source+Sans+Pro:wght@300&display=swap"},
        Font {link: "https://fonts.googleapis.com/css2?family=Orbitron:wght@700&display=swap"},
        match read_page {
            CurrentPage::HomePage => rsx!(cx, HomePage {}),
            CurrentPage::Flashcards => rsx!(cx, flashcards::Flashcards {}),
            CurrentPage::NoteInput => rsx!(cx, note_input::InputFlashcards {})
        }
    })
}


fn main() {
    // Initialising log
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
    // Launch the app!
    dioxus::desktop::launch_cfg(App, |c|
        // Some configuration
        c.with_window(|w|
            w.with_title("Magistrax")
                .with_maximized(true)
        )
    );
}
