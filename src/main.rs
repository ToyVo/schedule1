use crate::components::{Button, IconButton};
use crate::sellable::{Ingredient, MixState, Product, Quality, Sellable};
use dioxus::prelude::*;
use dioxus_free_icons::icons::go_icons::{GoBookmark, GoBookmarkSlash};
use sellable::OneTimeIngredient;
use std::collections::HashMap;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod components;
mod sellable;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut working_product = use_signal(|| Sellable::from_product(Product::OGKush));
    let mut saved_recipes = use_signal(HashMap::<String, Sellable>::new);
    let mut mix_state = use_signal(MixState::default);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "grid gap-4",
            style: "grid-template-columns: minmax(365px, 1fr) minmax(240px, 1fr) minmax(150px, 1fr)",
            div {
                class: "grid grid-cols-3 gap-4",
                div { class: "col-span-full", "Base Product" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::OGKush)), "OG Kush" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::SourDiesel)), "Sour Diesel" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::GreenCrack)), "Green Crack" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::GranddaddyPurple)), "Granddaddy Purple" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::Meth)), "Meth" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::Cocaine)), "Cocaine" }
                div { class: "col-span-full", "Add Ingredient" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Cuke)), "Cuke" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Banana)), "Banana" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Paracetamol)), "Paracetamol" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Donut)), "Donut" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Viagra)), "Viagra" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::MouthWash)), "Mouth Wash" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::FluMedicine)), "Flu Medicine" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Gasoline)), "Gasoline" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::EnergyDrink)), "Energy Drink" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::MotorOil)), "Motor Oil" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::MegaBean)), "Mega Bean" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Chili)), "Chili" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Battery)), "Battery" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Iodine)), "Iodine" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::Addy)), "Addy" }
                Button { onclick: move |_| working_product.set(working_product().add_ingredient(Ingredient::HorseSemen)), "Horse Semen" }
                if !saved_recipes.read().is_empty() {
                    div { class: "col-span-full", "Saved Recipes" }
                    {saved_recipes.read().iter().map(|(key, recipe)| {
                        let recipe_clone = recipe.clone();
                        rsx! {
                            Button {
                                key: "{key}",
                                onclick: move |_| {
                                    working_product.set(recipe_clone.clone());
                                },
                                "{recipe.name}"
                            }
                        }
                    })}
                }
            }
            div {
                class: "grid grid-cols-2 gap-4 content-start",
                div {
                    class: "col-span-full flex gap-2",
                    if saved_recipes.read().contains_key(&working_product.read().key()) {
                        IconButton {
                            icon: GoBookmarkSlash,
                            onclick: move |_| {
                                saved_recipes.write().remove(&working_product.read().key());
                            }
                        }
                    } else {
                        IconButton {
                            icon: GoBookmark,
                            onclick: move |_| {
                                saved_recipes.write().insert(working_product.read().key(), working_product.read().clone());
                            }
                        }
                    },
                    input {
                        class: "grow",
                        value: "{working_product.read().name}",
                        oninput: move |event| working_product.set(working_product().with_name(event.value())),
                    }
                }
                div { "Based on:" }
                div { class: "justify-self-end", "Price:" }
                div {"{working_product.read().base:?}"}
                div { class: "justify-self-end", "${working_product.read().base.price(mix_state()):.2}" }
                match (working_product.read().base, mix_state.read().soil_quality) {
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
                match (working_product.read().base, mix_state.read().ingredients.contains(&OneTimeIngredient::PGR)) {
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
                match (working_product.read().base, mix_state.read().ingredients.contains(&OneTimeIngredient::Fertilizer)) {
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
                match (working_product.read().base, mix_state.read().ingredients.contains(&OneTimeIngredient::SpeedGrow)) {
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
                    match working_product.read().base {
                        Product::Meth => rsx! {
                            Button {
                                onclick: move |_| {
                                    mix_state.write().psudo_quality = Quality::Low;
                                },
                                active: mix_state.read().psudo_quality == Quality::Low,
                                "Low"
                            }
                            Button {
                                onclick: move |_| {
                                    mix_state.write().psudo_quality = Quality::Medium;
                                },
                                active: mix_state.read().psudo_quality == Quality::Medium,
                                "Medium"
                            }
                            Button {
                                onclick: move |_| {
                                    mix_state.write().psudo_quality = Quality::High;
                                },
                                active: mix_state.read().psudo_quality == Quality::High,
                                "High"
                            }
                        },
                        _ => rsx! {
                            label {
                                class: "flex gap-2 whitespace-nowrap items-center",
                                "Use PGR",
                                input {
                                    r#type: "checkbox",
                                    checked: "{mix_state.read().ingredients.contains(&OneTimeIngredient::PGR)}",
                                    onchange: move |_| {
                                        if mix_state.read().ingredients.contains(&OneTimeIngredient::PGR) {
                                            mix_state.write().ingredients.remove(&OneTimeIngredient::PGR);
                                        } else {
                                            mix_state.write().ingredients.insert(OneTimeIngredient::PGR);
                                        }
                                    }
                                }
                            }
                            Button {
                                onclick: move |_| {
                                    mix_state.write().use_pot = false;
                                },
                                active: !mix_state.read().use_pot,
                                "Tent"
                            }
                            Button {
                                onclick: move |_| {
                                    mix_state.write().use_pot = true;
                                },
                                active: mix_state.read().use_pot,
                                "Pot"
                            }
                        }
                    }
                }
                div {
                    class: "justify-self-end",
                    "/{working_product.read().yield_amount(mix_state())}"
                }
                div { class: "col-span-full",
                    match working_product.read().base {
                        Product::Meth => rsx! {},
                        _ => rsx! {
                            label {
                                class: "flex gap-2 whitespace-nowrap items-center",
                                "Use Fertilizer",
                                input {
                                    r#type: "checkbox",
                                    checked: "{mix_state.read().ingredients.contains(&OneTimeIngredient::Fertilizer)}",
                                    onchange: move |_| {
                                        if mix_state.read().ingredients.contains(&OneTimeIngredient::Fertilizer) {
                                            mix_state.write().ingredients.remove(&OneTimeIngredient::Fertilizer);
                                        } else {
                                            mix_state.write().ingredients.insert(OneTimeIngredient::Fertilizer);
                                        }
                                    }
                                }
                            }
                            label {
                                class: "flex gap-2 whitespace-nowrap items-center",
                                "Use Speed Grow",
                                input {
                                    r#type: "checkbox",
                                    checked: "{mix_state.read().ingredients.contains(&OneTimeIngredient::SpeedGrow)}",
                                    onchange: move |_| {
                                        if mix_state.read().ingredients.contains(&OneTimeIngredient::SpeedGrow) {
                                            mix_state.write().ingredients.remove(&OneTimeIngredient::SpeedGrow);
                                        } else {
                                            mix_state.write().ingredients.insert(OneTimeIngredient::SpeedGrow);
                                        }
                                    }
                                }
                            }
                            Button {
                                onclick: move |_| {
                                    mix_state.write().soil_quality = Quality::Low;
                                },
                                active: mix_state.read().soil_quality == Quality::Low,
                                "Soil"
                            }
                            Button {
                                onclick: move |_| {
                                    mix_state.write().soil_quality = Quality::Medium;
                                },
                                active: mix_state.read().soil_quality == Quality::Medium,
                                "Long-Life Soil"
                            }
                            Button {
                                onclick: move |_| {
                                    mix_state.write().soil_quality = Quality::High;
                                },
                                active: mix_state.read().soil_quality == Quality::High,
                                "Extra Long-Life Soil"
                            }
                        }
                    }
                }
                div { class: "col-span-full", "Price per unit:" }
                div {"{working_product.read().base}"}
                div { class: "justify-self-end", "${working_product.read().unit_price(mix_state()):.2}" }
                match (working_product.read().base, mix_state.read().soil_quality) {
                    (
                        Product::OGKush
                        | Product::GreenCrack
                        | Product::SourDiesel
                        | Product::GranddaddyPurple
                        | Product::Cocaine,
                        Quality::Low,
                    ) => rsx! {
                        div { "Soil" }
                        div { class: "justify-self-end", "${10./working_product.read().yield_amount(mix_state()):.2}" }
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
                        div { class: "justify-self-end", "${30./(working_product.read().yield_amount(mix_state())*2.):.2}" }
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
                        div { class: "justify-self-end", "${60./(working_product.read().yield_amount(mix_state())*3.):.2}" }
                    },
                    _ => rsx! {},
                }
                match (working_product.read().base, mix_state.read().ingredients.contains(&OneTimeIngredient::PGR)) {
                    (
                        Product::OGKush
                        | Product::GreenCrack
                        | Product::SourDiesel
                        | Product::GranddaddyPurple
                        | Product::Cocaine,
                        true,
                    ) => rsx! {
                        div { "PGR" }
                        div { class: "justify-self-end", "${30./working_product.read().yield_amount(mix_state()):.2}" }
                    },
                    _ => rsx! {},
                }
                match (working_product.read().base, mix_state.read().ingredients.contains(&OneTimeIngredient::Fertilizer)) {
                    (
                        Product::OGKush
                        | Product::GreenCrack
                        | Product::SourDiesel
                        | Product::GranddaddyPurple
                        | Product::Cocaine,
                        true,
                    ) => rsx! {
                        div { "Fertilizer" }
                        div { class: "justify-self-end", "${30./working_product.read().yield_amount(mix_state()):.2}" }
                    },
                    _ => rsx! {},
                }
                match (working_product.read().base, mix_state.read().ingredients.contains(&OneTimeIngredient::SpeedGrow)) {
                    (
                        Product::OGKush
                        | Product::GreenCrack
                        | Product::SourDiesel
                        | Product::GranddaddyPurple
                        | Product::Cocaine,
                        true,
                    ) => rsx! {
                        div { "Speed Grow" }
                        div { class: "justify-self-end", "${30./working_product.read().yield_amount(mix_state()):.2}" }
                    },
                    _ => rsx! {},
                }
                for ingredient in working_product.read().ingredients.iter() {
                    div {"{ingredient:?}"}
                    div { class: "justify-self-end", "${ingredient.price():.2}" }
                }
                div { class: "border col-span-full" }
                div {
                    "Total Price:"
                }
                div { class: "justify-self-end", "${working_product.read().price(mix_state()):.2}" }
            }
            div {
                class: "grid grid-cols-2 gap-4 content-start",
                div { class: "col-span-full border", "Warning: Column in progress, has inaccuracies"}
                div {
                    class: "col-span-full",
                    "Sell Price:"
                }
                div {
                    "Baggie"
                }
                div { class: "justify-self-end", "${working_product.read().sell_price():.0}" }
                div {
                    "Jar"
                }
                div { class: "justify-self-end", "${working_product.read().sell_price()*5.:.0}" }
                div {
                    "Brick"
                }
                div { class: "justify-self-end", "${working_product.read().sell_price()*20.:.0}" }
                div { class: "border col-span-full" }
                div {
                    "Addictiveness"
                }
                div { class: "justify-self-end", "{working_product.read().addictiveness():.0}%" }
                if !working_product.read().effects.is_empty() {
                    div { class: "border col-span-full" }
                    div { "Causes:" }
                    div { class: "justify-self-end", "Multiplier:" }
                    for effect in working_product.read().effects.iter() {
                        div { "{effect:?}" }
                        div { class: "justify-self-end", "x{effect.multiplier():.2}" }
                    }
                }
            }
        }
    }
}
