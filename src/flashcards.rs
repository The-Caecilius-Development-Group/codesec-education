use dioxus::{prelude::*, fermi::use_read};

use crate::{USER_DATA, data::FlashcardSet};

#[derive(Props)]
struct StudySetProps<'a> {
    /// A reference to the flashcard set to render
    set: &'a FlashcardSet
}
/// Render a preview of a study set
fn StudySet<'a>(cx: Scope<'a, StudySetProps<'a>>) -> Element<'a> {
    rsx!(cx, 
        div {
            class: "study-set-preview",
            h2 {"{cx.props.set.name}"}
        }
    )
}

pub fn Flashcards(cx: Scope) -> Element {
    let user_data = use_read(&cx, USER_DATA);
    let study_set_previews = user_data.sets.iter().map(|s|
        rsx!(cx, StudySet {
            set: s,
            key: "{s.name}"
        })
    );
    rsx!(cx,
        div {
            class: "center-div",
            h1 {"Flashcards"},
            div {
                class: "row-flex",
                div {
                    class: "col-23",
                    h2 {"Your study sets"},
                    div {
                        class: "study-set-preview study-set-create",
                        h2 {"Create a study set"}
                    },
                    study_set_previews
                },
                div {class: "divider"}
                div {
                    class: "col-13",
                    "hi"
                }
            }
        }
    )
}