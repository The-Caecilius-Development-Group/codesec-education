use dioxus::prelude::*;
use crate::{USER_DATA, data::FlashcardSet, CurrentPage, PageLink};


/// The flashcard note input page
pub fn InputFlashcards(cx: Scope) -> Element {

    let number_of_flashcards = 0..5;
    
    let flashcard_list = number_of_flashcards.map(|number| rsx!(
        div {
            "class": "center-div",
            span{
                "Front of card {number}",
                input {
                    "type": "text",
                    "id": "front-card-{number}",
                    "class": "card-{number}"
                }
            }

            span{
                "Back of card {number}",
                input {
                    "type": "text",
                    "id": "back-card-{number}",
                    "class": "card-{number}"
                }
            }
            div {
                PageLink {
                    class: "home-button",
                    name: "Home",
                    redirect: CurrentPage::HomePage
                }
            }
        }
    ));

    cx.render(
        rsx!(
            ul {
                flashcard_list
            }
        )
    )
}