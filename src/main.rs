use crate::components::{Button, IconButton};
use crate::sellable::{Ingredient, Product, Quality, Sellable};
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::{HiBookmark, HiBookmarkAlt};
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
    let mut saved_recipes: Signal<HashMap<String,Sellable>> = use_signal(|| HashMap::new());
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "grid grid-cols-3 gap-4",
            div {
                class: "grid grid-cols-3 gap-4",
                div { class: "col-span-full", "Base Product" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::OGKush)), "OG Kush" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::SourDiesel)), "Sour Diesel" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::GreenCrack)), "Green Crack" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::GranddaddyPurple)), "Granddaddy Purple" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::Meth(Quality::Low))), "Meth (Low)" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::Meth(Quality::Medium))), "Meth (Med)" }
                Button { onclick: move |_| working_product.set(Sellable::from_product(Product::Meth(Quality::High))), "Meth (High)" }
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
                    for recipe in saved_recipes.read().iter() {
                        Button {
                            onclick: move |_| {
                                // working_product.set(Sellable::from_product(recipe.1.base));
                            },
                            "{recipe.1.base:?}"
                        }
                    }
                }
            }
            div {
                class: "grid grid-cols-2 gap-4 content-start",
                div {
                    class: "col-span-full",
                    if saved_recipes.read().contains_key(&working_product.read().key()) {
                        IconButton {
                            icon: HiBookmarkAlt,
                            onclick: move |_| {
                                saved_recipes.write().remove(&working_product.read().key());
                            }
                        }
                    } else {
                        IconButton {
                            icon: HiBookmark,
                            onclick: move |_| {
                                saved_recipes.write().insert(working_product.read().key(), working_product.read().clone());
                            }
                        }
                    },
                }
                div { "Based on:" }
                div { class: "justify-self-end", "Price:" }
                div {"{working_product.read().base:?}"}
                div { class: "justify-self-end", "{working_product.read().base.price()}" }
                if !working_product.read().ingredients.is_empty() {
                    div { class: "col-span-full", "With:" }
                    for ingredient in working_product.read().ingredients.iter() {
                        div {"{ingredient:?}"}
                        div { class: "justify-self-end", "{ingredient.price()}" }
                    }
                    div { class: "border col-span-full" }
                    div {
                        "Total Price:"
                    }
                    div { class: "justify-self-end", "{working_product.read().price()}" }
                }
            }
            div {
                class: "grid grid-cols-2 gap-4 content-start",
                div { class: "col-span-full border", "Warning: Column in progress, has inaccuracies"}
                if !working_product.read().effects.is_empty() {
                    div { "Causes:" }
                    div { class: "justify-self-end", "Multiplier:" }
                    for effect in working_product.read().effects.iter() {
                        div { "{effect:?}" }
                        div { class: "justify-self-end", "{effect.multiplier():.2}" }
                    }
                    div { class: "border col-span-full" }
                }
                div {
                    "Sell Price:"
                }
                div { class: "justify-self-end", "{working_product.read().sell_price():.0}" }
            }
        }
    }
}
