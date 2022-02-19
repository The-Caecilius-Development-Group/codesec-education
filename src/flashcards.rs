use dioxus::{prelude::*, fermi::use_read};
use std::time;
use crate::{USER_DATA, data::FlashcardSet, CurrentPage, PageLink};

#[derive(Props)]
struct StudySetProps<'a> {
    /// A reference to the flashcard set to render
    set: &'a FlashcardSet
}
/// Render a preview of a study set
fn StudySet<'a>(cx: Scope<'a, StudySetProps<'a>>) -> Element<'a> {
    rsx!(cx, 
        button {
            "type": "button",
            class: "study-set-preview",
            h2 {"{cx.props.set.name}"}
        }
    )
}

pub fn Flashcards(cx: Scope) -> Element {
<<<<<<< HEAD
    let user_data = use_read(&cx, USER_DATA).borrow_mut();
    let study_set_previews = user_data.sets.iter().map(|s|
        rsx!(cx, StudySet {
            set: s,
            key: "{s.name}"
        })
    );

    if user_data.duration_since_last_visit.is_zero() {
        println!("Duration is zero!");
    }

    if user_data.last_visit == 0 {
        user_data.last_visit = time::SystemTime::now();
        println!("{}", user_data.last_visit); 
    }

=======
    let user_data = use_read(&cx, USER_DATA);
    // let study_set_previews: Vec<Element> = user_data.borrow().sets.iter().map(|s|
    //     rsx!(cx, StudySet {
    //         set: s,
    //         key: "{s.name}"
    //     })
    // ).collect();
>>>>>>> fe5b1809b0a9945bbbd0b4178aedb5858d03d113
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
                    // study_set_previews
                },
                div {class: "divider"}
                div {
                    class: "col-13",
                    "hi"
                }

                PageLink {
                    class: "home-button",
                    name: "Home",
                    redirect: CurrentPage::HomePage
                }
            }
        }
    )
}