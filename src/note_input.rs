use crate::{data::FlashcardSet, data::RichText, CurrentPage, CURRENT_PAGE, USER_DATA};
use dioxus::{
    fermi::{use_read, use_set, Atom},
    prelude::*,
};
use std::cell::RefCell;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum CardSide {
    Front,
    Back,
}

static ACTIVE_SET: Atom<RefCell<Option<FlashcardSet>>> = |_| RefCell::new(None);

#[derive(Props, PartialEq)]
struct FlashcardInputProps {
    id: u64,
    side: CardSide,
}
fn FlashcardInput(cx: Scope<FlashcardInputProps>) -> Element {
    let active_set = use_read(&cx, ACTIVE_SET);
    let set_borrow = active_set.borrow();
    let card = &set_borrow.as_ref().unwrap()[cx.props.id];
    let text = match cx.props.side {
        CardSide::Front => &card.front,
        CardSide::Back => &card.back,
    };
    rsx!(cx, textarea {
        rows: "4", cols: "50",
        style: "color: {text.color};",
        onchange: move |env| {
            let mut set_borrow = active_set.borrow_mut();
            let card = &mut set_borrow.as_mut().unwrap()[cx.props.id];
            let text = match cx.props.side {
                CardSide::Front => &mut card.front,
                CardSide::Back => &mut card.back
            };
            text.text = env.data.value.clone();
        },
        "{text.text}"
    })
}

/// The flashcard note input page
pub fn InputFlashcards(cx: Scope) -> Element {
    let set = use_read(&cx, ACTIVE_SET);
    if set.borrow().is_none() {
        // just created the component
        *set.borrow_mut() = Some(FlashcardSet::new("".into()));
    }
    set.borrow_mut()
        .as_mut()
        .unwrap()
        .add(RichText::empty(), RichText::empty());
    let set_borrow = set.borrow();
    let set_ = set_borrow.as_ref().unwrap();
    let flashcard_list = set_.flashcards.iter().map(|f| {
        let key = f.id();
        rsx!(cx,
            div {
                key: "{key}",
                class: "flashcard-input-flex",
                FlashcardInput {
                    id: f.id(),
                    side: CardSide::Front
                },
                FlashcardInput {
                    id: f.id(),
                    side: CardSide::Back
                },
            }
        )
    });

    let (warning, set_warning) = use_state(&cx, || false);

    cx.render(
        rsx!(
            div {
                class: "center-div",
                input {
                    "type": "input",
                    onchange: move |e| {
                        set.borrow_mut().as_mut().unwrap().name = e.value.clone();
                    }
                }
                flashcard_list
                // Submit button
                button {
                    "type": "button",
                    onclick: move |_| {
                        let user_data = use_read(&cx, USER_DATA);
                        let set_borrow = set.borrow();
                        let name = &set_borrow.as_ref().unwrap().name;
                        // Check if a set with this name already exists
                        if user_data.borrow().get().sets.iter().any(|s| &s.name == name) {
                            // little warning
                            set_warning(true);
                        } else {
                            // It doesn't - success
                            user_data.borrow_mut().modify(|d| d.sets.push(
                                set.borrow_mut().take().unwrap()
                            ));
                            (use_set(&cx, CURRENT_PAGE))(CurrentPage::Flashcards);
                        }
                    },
                    "Save"
                }
                warning.then(|| rsx!(cx,
                    p { class: "warning",
                        "Please use a unique name"
                    }
                ))
            }
        )
    )
}
