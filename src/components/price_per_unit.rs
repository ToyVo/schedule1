use crate::sellable::{MixState, OneTimeIngredient, Product, Quality, Sellable};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ComponentProps {
    working_product: Sellable,
    mix_state: MixState,
}

#[component]
pub fn PricePerUnit(props: ComponentProps) -> Element {
    rsx! {
        div { class: "col-span-full", "Price per unit:" }
        div {"{props.working_product.base}"}
        div { class: "justify-self-end", "${props.working_product.unit_price(props.mix_state.clone()):.2}" }
        match (props.working_product.base, props.mix_state.soil_quality) {
            (
                Product::OGKush
                | Product::GreenCrack
                | Product::SourDiesel
                | Product::GranddaddyPurple
                | Product::Cocaine,
                Quality::Low,
            ) => rsx! {
                div { "Soil" }
                div { class: "justify-self-end", "${10./props.working_product.yield_amount(props.mix_state.clone()):.2}" }
            },
            (
                Product::OGKush
                | Product::GreenCrack
                | Product::SourDiesel
                | Product::GranddaddyPurple
                | Product::Cocaine,
                Quality::Medium,
            ) => rsx! {
                div { "Long-Life Soil" }
                div { class: "justify-self-end", "${30./(props.working_product.yield_amount(props.mix_state.clone())*2.):.2}" }
            },
            (
                Product::OGKush
                | Product::GreenCrack
                | Product::SourDiesel
                | Product::GranddaddyPurple
                | Product::Cocaine,
                Quality::High,
            ) => rsx! {
                div { "Extra Long-Life Soil" }
                div { class: "justify-self-end", "${60./(props.working_product.yield_amount(props.mix_state.clone())*3.):.2}" }
            },
            _ => rsx! {},
        }
        match (props.working_product.base, props.mix_state.ingredients.contains(&OneTimeIngredient::PGR)) {
            (
                Product::OGKush
                | Product::GreenCrack
                | Product::SourDiesel
                | Product::GranddaddyPurple
                | Product::Cocaine,
                true,
            ) => rsx! {
                div { "PGR" }
                div { class: "justify-self-end", "${30./props.working_product.yield_amount(props.mix_state.clone()):.2}" }
            },
            _ => rsx! {},
        }
        match (props.working_product.base, props.mix_state.ingredients.contains(&OneTimeIngredient::Fertilizer)) {
            (
                Product::OGKush
                | Product::GreenCrack
                | Product::SourDiesel
                | Product::GranddaddyPurple
                | Product::Cocaine,
                true,
            ) => rsx! {
                div { "Fertilizer" }
                div { class: "justify-self-end", "${30./props.working_product.yield_amount(props.mix_state.clone()):.2}" }
            },
            _ => rsx! {},
        }
        match (props.working_product.base, props.mix_state.ingredients.contains(&OneTimeIngredient::SpeedGrow)) {
            (
                Product::OGKush
                | Product::GreenCrack
                | Product::SourDiesel
                | Product::GranddaddyPurple
                | Product::Cocaine,
                true,
            ) => rsx! {
                div { "Speed Grow" }
                div { class: "justify-self-end", "${30./props.working_product.yield_amount(props.mix_state.clone()):.2}" }
            },
            _ => rsx! {},
        }
        for ingredient in props.working_product.ingredients.iter() {
            div {"{ingredient:?}"}
            div { class: "justify-self-end", "${ingredient.price():.2}" }
        }
        div { class: "border col-span-full" }
        div {
            "Total Price:"
        }
        div { class: "justify-self-end", "${props.working_product.price(props.mix_state.clone()):.2}" }
    }
}
