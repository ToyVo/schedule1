use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    children: Element,
    onclick: EventHandler<MouseEvent>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class:"border border-solid border-white hover:bg-neutral-800 hover:cursor-pointer p-3 rounded-md text-white",
            onclick: move |evt| props.onclick.call(evt),
            {props.children}
        }
    }
}
