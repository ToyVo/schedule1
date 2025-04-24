use crate::components::Button;
use crate::sellable::Product;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub set_working_product: EventHandler<Product>,
}

#[component]
pub fn BaseProducts(props: ComponentProps) -> Element {
    use Product::*;
    rsx! {
        div { class: "col-span-full", "Base Product" }
        Button { onclick: move |_| props.set_working_product.call(OGKush), "OG Kush" }
        Button { onclick: move |_| props.set_working_product.call(SourDiesel), "Sour Diesel" }
        Button { onclick: move |_| props.set_working_product.call(GreenCrack), "Green Crack" }
        Button { onclick: move |_| props.set_working_product.call(GranddaddyPurple), "Granddaddy Purple" }
        Button { onclick: move |_| props.set_working_product.call(Meth), "Meth" }
        Button { onclick: move |_| props.set_working_product.call(Cocaine), "Cocaine" }
    }
}
