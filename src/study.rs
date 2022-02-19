use std::{cell::Ref, lazy::OnceCell};

use dioxus::{prelude::*, fermi::{use_read, use_set}};
use rand::{prelude::SliceRandom, thread_rng};

use crate::{USER_DATA, CURRENT_PAGE, CurrentPage};


/// Study page
pub fn Study(cx: Scope) -> Element {
    let (time, set_time) = use_state(&cx, || 20u8);
    let sets = Ref::map(use_read(&cx, USER_DATA).borrow(), |u| &u.get().sets);
    let (chosen, set_chosen) = use_state(&cx, || sets[0].name.clone());
    rsx!(cx, div {
        class: "center-div",
        h1 {"Study"}
        "Study time"
        input {
            "type": "range",
            min: "5",
            max: "120",
            value: "{time}",
            class: "time-input",
            step: "5",
            oninput: move |e| {
                set_time(e.value.parse().unwrap());
            }
        }
        "{time} minutes"
        select {
            oninput: move |e| {
                set_chosen(e.value.clone());
            },
            sets.iter().map(|s|
                rsx! {
                    option {
                        value: "{s.name}",
                        key: "{s.name}",
                        "{s.name}"
                    }
                }
            )
        }
        button {
            "type": "button",
            onclick: move |_| {
                use_set(&cx, CURRENT_PAGE)(CurrentPage::FlashcardTester (FlashcardTesterProps {
                    set: chosen.clone()
                }));
            },
            "Go!"
        }
    })
}
#[derive(Props, PartialEq, Debug, Clone)]
pub struct FlashcardTesterProps {
    set: String
}
/// Tests the user on a flashcard set
pub fn FlashcardTester(cx: Scope<FlashcardTesterProps>) -> Element {
    let sets = Ref::map(use_read(&cx, USER_DATA).borrow(), |u| &u.get().sets);
    let target_set = sets.iter().find(|s| s.name == cx.props.set).unwrap();
    let queue = use_ref(&cx, || {
        let mut queue = target_set.flashcards.iter().map(|f| f.id()).collect::<Vec<u64>>();
        queue.shuffle(&mut thread_rng());
        let cell = OnceCell::new();
        cell.set(queue).unwrap();
        cell
    });
    let current_card_id = queue.read().get().unwrap()[0];
    let current_card = &target_set[current_card_id];
    rsx!(cx, div {
        class: "center-div",
        h1 {"{target_set.name}"}
        div {
            class: "row-flex",
            div {
                class: "test-flashcard",
                style: "color: {current_card.front.color};",
                "{current_card.front.text}"
            }
            div {
                class: "test-flashcard",
                style: "color: {current_card.back.color};",
                "{current_card.back.text}"
            }
        }
    })
}