use crate::sellable::Sellable;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub working_product: Sellable,
}

#[component]
pub fn MixStats(props: ComponentProps) -> Element {
    rsx! {
        div {
            class: "col-span-full",
            "Sell Price:"
        }
        div {
            "Baggie"
        }
        div { class: "justify-self-end", "${props.working_product.sell_price():.0}" }
        div {
            "Jar"
        }
        div { class: "justify-self-end", "${props.working_product.sell_price()*5.:.0}" }
        div {
            "Brick"
        }
        div { class: "justify-self-end", "${props.working_product.sell_price()*20.:.0}" }
        div { class: "border col-span-full" }
        div {
            "Addictiveness"
        }
        div { class: "justify-self-end", "{props.working_product.addictiveness():.0}%" }
        if !props.working_product.effects.is_empty() {
            div { class: "border col-span-full" }
            div { "Causes:" }
            div { class: "justify-self-end", "Multiplier:" }
            for effect in props.working_product.effects.iter() {
                div { "{effect:?}" }
                div { class: "justify-self-end", "x{effect.multiplier():.2}" }
            }
        }
    }
}
