use crate::components::{Button, IconButton};
use crate::sellable::{MixState, OneTimeIngredient, Product, Quality, Sellable};
use dioxus::prelude::*;
use dioxus_free_icons::icons::go_icons::{GoBookmark, GoBookmarkSlash};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Props)]
pub struct MixProps {
    pub set_working_product: EventHandler<Sellable>,
    pub set_psudo_quality: EventHandler<Quality>,
    pub set_soil_quality: EventHandler<Quality>,
    pub toggle_ingredient: EventHandler<OneTimeIngredient>,
    pub toggle_save: EventHandler<Sellable>,
    pub set_use_pot: EventHandler<bool>,
    pub working_product: Sellable,
    pub mix_state: MixState,
    pub saved_recipes: HashMap<String, Sellable>,
}

#[component]
pub fn MixOptions(props: MixProps) -> Element {
    let working_product_clone = props.working_product.clone();
    rsx! {
        div {
            class: "col-span-full flex gap-2",
            if props.saved_recipes.contains_key(&props.working_product.key()) {
                IconButton {
                    icon: GoBookmarkSlash,
                    onclick: move |_| {
                        props.toggle_save.call(working_product_clone.clone());
                    }
                }
            } else {
                IconButton {
                    icon: GoBookmark,
                    onclick: move |_| {
                        props.toggle_save.call(working_product_clone.clone());
                    }
                }
            },
            input {
                class: "grow",
                value: "{props.working_product.name}",
                oninput: move |event| props.set_working_product.call(props.working_product.with_name(event.value())),
            }
        }
        div { "Based on:" }
        div { class: "justify-self-end", "Price:" }
        div {"{props.working_product.base:?}"}
        div { class: "justify-self-end", "${props.working_product.base.price(props.mix_state.clone()):.2}" }
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
                div { class: "justify-self-end", "${10.:.2}" }
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
                div { class: "justify-self-end", "${30.:.2}" }
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
                div { class: "justify-self-end", "${60.:.2}" }
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
                div { class: "justify-self-end", "${30.:.2}" }
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
                div { class: "justify-self-end", "${30.:.2}" }
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
                div { class: "justify-self-end", "${30.:.2}" }
            },
            _ => rsx! {},
        }
        div { class: "border col-span-full" }
        div {
            class: "flex gap-2",
            match props.working_product.base {
                Product::Meth => rsx! {
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
                },
                _ => rsx! {
                    label {
                        class: "flex gap-2 whitespace-nowrap items-center",
                        "Use PGR",
                        input {
                            r#type: "checkbox",
                            checked: "{props.mix_state.ingredients.contains(&OneTimeIngredient::PGR)}",
                            onchange: move |_| {
                                props.toggle_ingredient.call(OneTimeIngredient::PGR);
                            }
                        }
                    }
                    Button {
                        onclick: move |_| {
                            props.set_use_pot.call(false);
                        },
                        active: !props.mix_state.use_pot,
                        "Tent"
                    }
                    Button {
                        onclick: move |_| {
                            props.set_use_pot.call(true);
                        },
                        active: props.mix_state.use_pot,
                        "Pot"
                    }
                }
            }
        }
        div {
            class: "justify-self-end",
            "/{props.working_product.yield_amount(props.mix_state.clone())}"
        }
        match props.working_product.base {
            Product::Meth => rsx! {},
            _ => rsx! {
                div { class: "col-span-full flex gap-2",
                    div {
                        class: "flex flex-col",
                        label {
                            class: "flex gap-2 whitespace-nowrap items-center",
                            "Use Fertilizer",
                            input {
                                r#type: "checkbox",
                                checked: "{props.mix_state.ingredients.contains(&OneTimeIngredient::Fertilizer)}",
                                onchange: move |_| {
                                    props.toggle_ingredient.call(OneTimeIngredient::Fertilizer);
                                }
                            }
                        }
                        label {
                            class: "flex gap-2 whitespace-nowrap items-center",
                            "Use Speed Grow",
                            input {
                                r#type: "checkbox",
                                checked: "{props.mix_state.ingredients.contains(&OneTimeIngredient::SpeedGrow)}",
                                onchange: move |_| {
                                    props.toggle_ingredient.call(OneTimeIngredient::SpeedGrow);
                                }
                            }
                        }
                    }
                    Button {
                        onclick: move |_| {
                            props.set_soil_quality.call(Quality::Low);
                        },
                        active: props.mix_state.soil_quality == Quality::Low,
                        "Soil"
                    }
                    Button {
                        onclick: move |_| {
                            props.set_soil_quality.call(Quality::Medium);
                        },
                        active: props.mix_state.soil_quality == Quality::Medium,
                        "Long-Life Soil"
                    }
                    Button {
                        onclick: move |_| {
                            props.set_soil_quality.call(Quality::High);
                        },
                        active: props.mix_state.soil_quality == Quality::High,
                        "Extra Long-Life Soil"
                    }
                }
            }
        }
    }
}
