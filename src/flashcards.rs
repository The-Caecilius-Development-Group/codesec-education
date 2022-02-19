use dioxus::{prelude::*, fermi::use_read};
use std::time;
use crate::{USER_DATA, data::FlashcardSet, CurrentPage, PageLink, data::RichText};


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
    let mut user_data = use_read(&cx, USER_DATA).borrow_mut();
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
        match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
            Ok(n) => {
                user_data.last_visit = n.as_secs();
                user_data.last_sys_time = n;
            },
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
        println!("{}", user_data.last_visit); 
    }

    // let time_since_last_visit = time::SystemTime::now.duration_since(time::UNIX_EPOCH).as_secs() - user_data.last_visit; 

    let sys_time = time::SystemTime::now().duration_since(time::UNIX_EPOCH);
    let mut time_since_last_visit: u64 = 0; 

    match sys_time {
        Ok(n) => {time_since_last_visit = n.as_secs() - user_data.last_visit; user_data.last_visit = n.as_secs(); user_data.last_sys_time = n;}
        Err(e) => println!("{:?}", e)
    }

    if  time_since_last_visit != 0 {
        println!("{time_since_last_visit} seconds since last visit");
    }

    let user_data = use_read(&cx, USER_DATA);
    // let study_set_previews: Vec<Element> = user_data.borrow().sets.iter().map(|s|
    //     rsx!(cx, StudySet {
    //         set: s,
    //         key: "{s.name}"
    //     })
    // ).collect();
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