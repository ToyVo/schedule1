use crate::components::Button;
use crate::sellable::{MixState, Quality};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub set_psudo_quality: EventHandler<Quality>,
    pub mix_state: MixState,
}

#[component]
pub fn PsudoOptions(props: ComponentProps) -> Element {
    rsx! {
        div {
            class: "flex gap-2",
            Button {
                onclick: move |_| {
                    props.set_psudo_quality.call(Quality::Low);
                },
                active: props.mix_state.psudo_quality == Quality::Low,
                "Low"
            }
            Button {
                onclick: move |_| {
                    props.set_psudo_quality.call(Quality::Medium);
                },
                active: props.mix_state.psudo_quality == Quality::Medium,
                "Medium"
            }
            Button {
                onclick: move |_| {
                    props.set_psudo_quality.call(Quality::High);
                },
                active: props.mix_state.psudo_quality == Quality::High,
                "High"
            }
        }
        div {
            class: "justify-self-end",
            "/10"
        }
    }
}
