use dioxus::prelude::*;
mod note_input;

fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "center-div",
            h1 {"CodeSec"}
        }
    })
}

fn App(cx: Scope) -> Element {
    rsx! (cx, div {
        style {[include_str!("style.css")]},
        link {
            rel: "preconnect", href: "https://fonts.googleapis.com"
        },
        link {
            rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "true"
        },
        link {
            rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Source+Sans+Pro:wght@300&display=swap"
        },
        link{
            rel: "styelsheet", href: "https://fonts.googleapis.com/css2?family=Orbitron&display=swap"
        }
        HomePage {

        }
        note_input::input_flashcards {

        }
    })
}


fn main() {
    dioxus::desktop::launch(App);
}
