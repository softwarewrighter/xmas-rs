use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq)]
enum LightColor {
    Red,
    Yellow,
    Blue,
    Green,
}

impl LightColor {
    fn to_class(&self) -> &str {
        match self {
            LightColor::Red => "red",
            LightColor::Yellow => "yellow",
            LightColor::Blue => "blue",
            LightColor::Green => "green",
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let is_playing = use_state(|| true);
    let speed = use_state(|| 1);
    let speed_input_ref = use_node_ref();

    // Define the 8 lights pattern
    let lights = vec![
        LightColor::Red,
        LightColor::Yellow,
        LightColor::Blue,
        LightColor::Green,
        LightColor::Red,
        LightColor::Yellow,
        LightColor::Blue,
        LightColor::Green,
    ];

    let on_play = {
        let is_playing = is_playing.clone();
        Callback::from(move |_| {
            is_playing.set(true);
        })
    };

    let on_stop = {
        let is_playing = is_playing.clone();
        Callback::from(move |_| {
            is_playing.set(false);
        })
    };

    let on_speed_change = {
        let speed = speed.clone();
        let speed_input_ref = speed_input_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(input) = speed_input_ref.cast::<HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<i32>() {
                    if (1..=5).contains(&value) {
                        speed.set(value);
                    }
                }
            }
        })
    };

    // Calculate animation duration based on speed (inverse relationship)
    let animation_duration = match *speed {
        1 => "2s",
        2 => "1.5s",
        3 => "1s",
        4 => "0.75s",
        5 => "0.5s",
        _ => "1s",
    };

    html! {
        <>
            <div class="main">
                {
                    lights.iter().map(|color| {
                        let class = format!("circle {}", color.to_class());
                        let paused_class = if !*is_playing { " paused" } else { "" };
                        let full_class = format!("{}{}", class, paused_class);
                        let style = format!("animation-duration: {}", animation_duration);
                        html! {
                            <div class={full_class} style={style}></div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="utilities">
                <div class="title">
                    <h1 id="title">{ "Christmas Lights" }</h1>
                </div>
                <div class="buttons">
                    <button id="play" onclick={on_play}>{ "On" }</button>
                    <button id="stop" onclick={on_stop}>{ "Off" }</button>
                    <label for="quantity">{ "Speed:" }</label>
                    <input
                        type="number"
                        id="quantity"
                        name="quantity"
                        min="1"
                        max="5"
                        placeholder="From 1 until 5 only."
                        ref={speed_input_ref.clone()}
                    />
                    <input
                        type="submit"
                        id="submit"
                        value="RUN"
                        onclick={on_speed_change}
                    />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
