use std::{cell::RefCell};

use dioxus::{prelude::*, fermi::{Atom, use_read}};

use crate::{data::{FlashcardSet, RichText}};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum CardSide {Front, Back}

static ACTIVE_SET: Atom<RefCell<FlashcardSet>> = |_| RefCell::new(FlashcardSet::new("".into()));

#[derive(Props, PartialEq)]
struct FlashcardInputProps {
    id: u64,
    side: CardSide
}
fn FlashcardInput(cx: Scope<FlashcardInputProps>) -> Element {
    let active_set = use_read(&cx, ACTIVE_SET);
    let card = &active_set.borrow()[cx.props.id];
    let text = match cx.props.side {
        CardSide::Front => &card.front,
        CardSide::Back => &card.back
    };
    rsx!(cx, textarea {
        rows: "4", cols: "50",
        style: "color: {text.color};",
        onchange: move |env| {
            let card = &mut active_set.borrow_mut()[cx.props.id];
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
    set.borrow_mut().add(RichText::empty(), RichText::empty());
    let set_ = set.borrow();
    let flashcard_list = set_.flashcards
    .iter().map(|f| rsx!(cx, 
        div {
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
    ));

    cx.render(
        rsx!(
            div {
                class: "center-div",
                flashcard_list
            }
        )
    )
}