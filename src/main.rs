use crate::sellable::{MixState, Product, Sellable};
use components::{AddIngredients, BaseProducts, MixOptions, MixStats, PricePerUnit, SavedRecipes};
use dioxus::prelude::*;
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
                class: "grid grid-cols-3 gap-4 content-start",
                BaseProducts { set_working_product: move |product| working_product.set(Sellable::from_product(product)) }
                AddIngredients { add_ingredient: move |ingredient| working_product.set(working_product().add_ingredient(ingredient)) }
                SavedRecipes {
                    set_working_product: move |recipe| working_product.set(recipe),
                    working_product: working_product(),
                    saved_recipes: saved_recipes(),
                }
            }
            div {
                class: "grid grid-cols-2 gap-4 content-start",
                MixOptions {
                    mix_state: mix_state(),
                    saved_recipes: saved_recipes(),
                    working_product: working_product(),
                    set_working_product: move |recipe| working_product.set(recipe),
                    set_psudo_quality: move |quality| mix_state.write().soil_quality = quality,
                    set_soil_quality: move |quality| mix_state.write().soil_quality = quality,
                    toggle_ingredient: move |ingredient| {
                        if mix_state.read().ingredients.contains(&ingredient) {
                            mix_state.write().ingredients.remove(&ingredient);
                        } else {
                            mix_state.write().ingredients.insert(ingredient);
                        }
                    },
                    toggle_save: move |recipe: Sellable| {
                        let key = recipe.key();
                        if saved_recipes.read().contains_key(&key) {
                            saved_recipes.write().remove(&key);
                        } else {
                            saved_recipes.write().insert(key, recipe);
                        }
                    },
                    set_use_pot: move |use_pot| mix_state.write().use_pot = use_pot,
                }
                PricePerUnit {
                    working_product: working_product(),
                    mix_state: mix_state(),
                }
            }
            div {
                class: "grid grid-cols-2 gap-4 content-start",
                div { class: "col-span-full border", "Warning: Column in progress, has inaccuracies"}
                MixStats { working_product: working_product() }
            }
        }
    }
}
