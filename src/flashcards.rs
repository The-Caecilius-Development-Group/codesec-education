use std::cell::Ref;

use dioxus::{prelude::*, fermi::use_read};

use crate::{USER_DATA, CurrentPage, PageLink};

#[derive(Props, PartialEq)]
struct StudySetProps {
    /// A reference to the flashcard set to render
    set: String
}
/// Render a preview of a study set
fn StudySet(cx: Scope<StudySetProps>) -> Element {
    let user_data = use_read(&cx, USER_DATA);
    let sets = 
        Ref::map(user_data.borrow(), |d| &d.get().sets);
    let set = sets.iter().find(|s| s.name == cx.props.set).unwrap();
    rsx!(cx, 
        button {
            "type": "button",
            class: "study-set-preview",
            h2 {"{set.name}"}
        }
    )
}

pub fn Flashcards(cx: Scope) -> Element {
    let user_data = use_read(&cx, USER_DATA);
    let sets = 
    Ref::map(user_data.borrow(), |d| &d.get().sets);
    let study_set_previews: Vec<Element> = sets.iter().map(|s|
        cx.render(rsx!(StudySet {
            set: s.name.clone(),
            key: "{s.name}"
        }))
    ).collect();
    rsx!(cx,
        div {
            class: "center-div",
            h1 {"Flashcards"},
            div {
                class: "row-flex",
                div {
                    class: "col-23",
                    h2 {"Your study sets"},
                    PageLink {
                        class: "study-set-preview study-set-create",
                        name: "Create a study set",
                        redirect: CurrentPage::NoteInput
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