use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};

#[derive(PartialEq, Props, Clone)]
pub struct IconButtonProps<T: IconShape + Clone + PartialEq + 'static> {
    pub icon: T,
    pub children: Element,
    pub onclick: EventHandler<MouseEvent>,
}

#[component]
pub fn IconButton<T: IconShape + Clone + PartialEq + 'static>(
    props: IconButtonProps<T>,
) -> Element {
    rsx! {
        button {
            class:"hover:bg-neutral-800 hover:cursor-pointer p-2 rounded-full",
            onclick: move |evt| props.onclick.call(evt),
            {props.children}
            Icon {
                icon: props.icon,
                fill: "white",
            }
        }
    }
}
