use std::cell::Ref;

use dioxus::{prelude::*, fermi::use_read};
use std::time;
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

// fn SystemTimeComponent(cx: Scope) -> Element {
//     let user_data_borrow = use_read(&cx, USER_DATA).borrow();

//     if user_data.duration_since_last_visit.is_zero() {
//         println!("Duration is zero!");
//     }

//     if user_data.last_visit == 0 {               
//         match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
//             Ok(n) => {
//                 use_read(&cx, USER_DATA).borrow_mut().modify(|d| {
//                     d.last_visit = n.as_secs();
//                     d.last_sys_time = n;
//                 });
//             },
//             Err(_) => panic!("SystemTime before UNIX EPOCH!"),
//         }
//         println!("{}", user_data.last_visit); 
//     }

//     // let time_since_last_visit = time::SystemTime::now.duration_since(time::UNIX_EPOCH).as_secs() - user_data.last_visit; 

//     let sys_time = time::SystemTime::now().duration_since(time::UNIX_EPOCH);
//     let mut time_since_last_visit: u64 = 0; 

//     match sys_time {
//         Ok(n) => {
//             time_since_last_visit = n.as_secs() - user_data.last_visit;
//             use_read(&cx, USER_DATA).borrow_mut().modify(|d| {
//                 d.last_visit = n.as_secs();
//                 d.last_sys_time = n;
//             });
//         }
//         Err(e) => println!("{:?}", e)
//     }

//     if  time_since_last_visit != 0 {
//         println!("{time_since_last_visit} seconds since last visit");
//     }
//     rsx!(cx, div {})
// }

pub fn Flashcards(cx: Scope) -> Element {
    let user_data_borrow = use_read(&cx, USER_DATA).borrow();
    let user_data = user_data_borrow.get();

    let sets = &user_data.sets;
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

                PageLink {
                    class: "home-button",
                    name: "Home",
                    redirect: CurrentPage::HomePage
                }
            }
        }
    )
}