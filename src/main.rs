use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {"Hello world!"}
    })
}

fn main() {
    dioxus::desktop::launch(app);
}
