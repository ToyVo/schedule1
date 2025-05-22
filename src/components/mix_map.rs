use crate::Effect;
use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use crate::sellable::Sellable;

const SCALE: f64 = 100.;

fn draw_axis(context: &web_sys::CanvasRenderingContext2d, width: f64, height: f64) {
    // start
    context.save();
    context.set_stroke_style_str("#888");
    context.set_line_width(1.0);
    // x axis
    context.begin_path();
    context.move_to(0.0, height / 2.0);
    context.line_to(width, height / 2.0);
    context.stroke();
    // y axis
    context.begin_path();
    context.move_to(width / 2.0, 0.0);
    context.line_to(width / 2.0, height);
    context.stroke();
    // finish
    context.restore();
}

fn draw_circle(
    context: &web_sys::CanvasRenderingContext2d,
    center: (f64, f64),
    radius: f64,
    color: &str,
) {
    context.save();
    context.begin_path();
    context
        .arc(center.0, center.1, radius, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();
    context.set_fill_style_str(color);
    context.fill();
    context.restore();
}

fn draw_vector(
    context: &web_sys::CanvasRenderingContext2d,
    origin: (f64, f64),
    magnitude: f64,
    direction: (f64, f64),
    color: &str,
) {
    context.save();
    context.begin_path();
    context.move_to(origin.0, origin.1);
    context.line_to(
        origin.0 + magnitude * direction.0,
        origin.1 + magnitude * direction.1,
    );
    context.set_stroke_style_str(color);
    context.set_line_width(2.0);
    context.stroke();
    context.restore();
}

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub added_effect: Signal<Option<Effect>>,
    pub previous_working_product: Signal<Sellable>,
}

#[component]
pub fn MixMap(props: ComponentProps) -> Element {
    use_effect(move || {
        let added_effect = props.added_effect.read();
        let previous_working_product = props.previous_working_product.read();
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("mix_map")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let center = (width / 2., height / 2.);
        context.clear_rect(0., 0., width, height);
        draw_axis(&context, width, height);
        let all_effects = [
            Effect::AntiGravity,
            Effect::Athletic,
            Effect::Balding,
            Effect::BrightEyed,
            Effect::Calming,
            Effect::CalorieDense,
            Effect::Cyclopean,
            Effect::Disorienting,
            Effect::Electrifying,
            Effect::Energizing,
            Effect::Euphoric,
            Effect::Explosive,
            Effect::Focused,
            Effect::Foggy,
            Effect::Gingeritis,
            Effect::Glowing,
            Effect::Jennerising,
            Effect::Laxative,
            Effect::Lethal,
            Effect::LongFaced,
            Effect::Munchies,
            Effect::Paranoia,
            Effect::Refreshing,
            Effect::Schizophrenic,
            Effect::Sedating,
            Effect::SeizureInducing,
            Effect::Shrinking,
            Effect::Slippery,
            Effect::Smelly,
            Effect::Sneaky,
            Effect::Spicy,
            Effect::ThoughtProvoking,
            Effect::Toxic,
            Effect::TropicThunder,
            Effect::Zombifying,
        ]; 
        for effect in all_effects {
            let direction = effect.direction();
            let magnitude = effect.magnitude();
            let circle_center = (
                center.0 + direction.0 * magnitude * SCALE,
                center.1 + direction.1 * magnitude * SCALE,
            );
            draw_circle(&context, circle_center, 0.5 * SCALE, &effect.color());
        }
        if let Some(added_effect) = *added_effect {
            previous_working_product.effects.iter().for_each(|effect| {
                let direction = effect.direction();
                let magnitude = effect.magnitude();
                let circle_center = (
                    center.0 + direction.0 * magnitude * SCALE,
                    center.1 + direction.1 * magnitude * SCALE,
                );
                draw_vector(&context, circle_center, added_effect.magnitude() * SCALE, added_effect.direction(), &effect.color());
            });
        }
        for effect in all_effects {
            let direction = effect.direction();
            let magnitude = effect.magnitude();
            let circle_center = (
                center.0 + direction.0 * magnitude * SCALE,
                center.1 + direction.1 * magnitude * SCALE,
            );
            context.set_text_align("center");
            context.set_text_baseline("middle");
            context.fill_text(format!("{effect:?}").as_str(), circle_center.0, circle_center.1).unwrap();
        }
    });

    rsx! {
        div {
            class: "flex flex-col justify-center col-span-full",
            canvas {
                id: "mix_map",
                width: "1000",
                height: "1000",
                class: "w-auto",
                style: "border: 1px solid #888;"
            }
        }
    }
}
