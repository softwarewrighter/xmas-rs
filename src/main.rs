use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum LightColor {
    Red,
    Green,
    White,
    Yellow,
}

#[derive(Clone, PartialEq)]
enum LightShape {
    Circle,
    Star,
    CandyCane,
    Bell,
    Angel,
    Reindeer,
    Tree,
}

#[derive(Clone, PartialEq)]
enum DisplayMode {
    Mixed,
    Circle,
    Star,
    CandyCane,
    Bell,
    Angel,
    Reindeer,
    Tree,
}

impl LightColor {
    fn to_class(&self) -> &str {
        match self {
            LightColor::Red => "red",
            LightColor::Green => "green",
            LightColor::White => "white",
            LightColor::Yellow => "yellow",
        }
    }

    fn to_rgb(&self) -> &str {
        match self {
            LightColor::Red => "#ff0000",
            LightColor::Green => "#00cc00",
            LightColor::White => "#ffffff",
            LightColor::Yellow => "#ffdd00",
        }
    }

    fn animation_delay(&self, index: usize) -> &str {
        match index % 4 {
            0 => "0s",
            1 => "0.25s",
            2 => "0.5s",
            3 => "0.75s",
            _ => "0s",
        }
    }
}

#[derive(Properties, PartialEq)]
struct LightProps {
    color: LightColor,
    shape: LightShape,
    is_playing: bool,
    animation_duration: String,
    delay: String,
}

#[function_component(Light)]
fn light(props: &LightProps) -> Html {
    let color_rgb = props.color.to_rgb().to_string();
    let color_class = props.color.to_class();
    let class = format!("light {}", color_class);
    let paused = if !props.is_playing { " paused" } else { "" };
    let full_class = format!("{}{}", class, paused);

    // Render SVG inline based on shape
    let svg = match props.shape {
        LightShape::Circle => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
                <circle cx="50" cy="50" r="35" stroke={color_rgb.clone()} stroke-width="3" fill={color_rgb.clone()} fill-opacity="0.8" />
            </svg>
        },
        LightShape::Star => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
                <polygon points="50,10 61,38 90,38 66,57 76,85 50,69 24,85 34,57 10,38 39,38"
                         stroke={color_rgb.clone()} stroke-width="3" fill={color_rgb.clone()} fill-opacity="0.8" />
            </svg>
        },
        LightShape::CandyCane => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
                <path d="M 60 15 A 15 15 0 0 0 30 15 L 30 60 Q 30 70 38 70 L 45 70"
                      stroke={color_rgb.clone()} stroke-width="8" fill="none" stroke-linecap="round" />
                <path d="M 35 25 L 55 25 M 35 35 L 55 35 M 32 45 L 52 45 M 31 55 L 51 55"
                      stroke="white" stroke-width="3" fill="none" stroke-linecap="round" />
            </svg>
        },
        LightShape::Bell => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
                <path d="M 50 15 C 42 15, 38 20, 38 26 L 38 35 C 38 45, 30 52, 30 63 L 30 70 Q 30 75, 35 75 L 65 75 Q 70 75, 70 70 L 70 63 C 70 52, 62 45, 62 35 L 62 26 C 62 20, 58 15, 50 15 Z"
                      stroke={color_rgb.clone()} stroke-width="3" fill={color_rgb.clone()} fill-opacity="0.8" />
                <path d="M45 13 Q50 8 55 13" stroke="#444" stroke-width="2" fill="none" />
                <circle cx="50" cy="77" r="4" stroke="#FFD700" stroke-width="2" fill="#FFD700" />
            </svg>
        },
        LightShape::Angel => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
                <circle cx="50" cy="22" r="7" stroke={color_rgb.clone()} stroke-width="2" fill={color_rgb.clone()} fill-opacity="0.8" />
                <ellipse cx="50" cy="14" rx="10" ry="3" stroke="#FFD700" stroke-width="2" fill="none" />
                <path d="M 50 29 L 35 38 Q 25 44 25 52 Q 25 60 33 62 L 40 63 L 30 85 L 50 85 L 70 85 L 60 63 L 67 62 Q 75 60 75 52 Q 75 44 65 38 L 50 29 Z"
                      stroke={color_rgb.clone()} stroke-width="2" fill={color_rgb.clone()} fill-opacity="0.8" />
            </svg>
        },
        LightShape::Reindeer => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
                <path d="M 30 60 Q 28 50 32 42 L 35 35 Q 33 30 35 25 L 38 22 L 42 22 L 45 14 L 47 16 L 44 22 L 48 16 L 50 18 L 46 24 L 42 22 L 50 25 Q 58 28 60 35 L 61 40 Q 60 48 56 52 L 55 60 L 65 60 Q 72 60 74 67 L 74 80 L 68 80 L 68 72 L 62 72 L 62 80 L 56 80 L 52 80 L 52 72 L 46 72 L 46 80 L 40 80 Q 34 78 30 72 Z"
                      stroke={color_rgb.clone()} stroke-width="2" fill={color_rgb.clone()} fill-opacity="0.8" />
            </svg>
        },
        LightShape::Tree => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
                <path d="M 50 10 L 35 30 L 42 30 L 28 50 L 38 50 L 22 72 L 45 72 L 45 85 L 55 85 L 55 72 L 78 72 L 62 50 L 72 50 L 58 30 L 65 30 Z"
                      stroke={color_rgb.clone()} stroke-width="2" fill={color_rgb.clone()} fill-opacity="0.8" />
            </svg>
        },
    };

    html! {
        <div
            class={full_class}
            style={format!("animation-duration: {}; animation-delay: {}", props.animation_duration, props.delay)}
        >
            {svg}
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let is_playing = use_state(|| true);
    let speed = use_state(|| 1);
    let mode = use_state(|| DisplayMode::Mixed);

    // Define the 8 lights pattern based on mode
    let lights = match *mode {
        DisplayMode::Mixed => vec![
            (LightColor::Red, LightShape::Circle),
            (LightColor::Green, LightShape::Star),
            (LightColor::Yellow, LightShape::CandyCane),
            (LightColor::White, LightShape::Bell),
            (LightColor::Red, LightShape::Angel),
            (LightColor::Green, LightShape::Reindeer),
            (LightColor::Yellow, LightShape::Tree),
            (LightColor::White, LightShape::Circle),
        ],
        DisplayMode::Circle => vec![
            (LightColor::Red, LightShape::Circle),
            (LightColor::Green, LightShape::Circle),
            (LightColor::Red, LightShape::Circle),
            (LightColor::Green, LightShape::Circle),
            (LightColor::Yellow, LightShape::Circle),
            (LightColor::White, LightShape::Circle),
            (LightColor::Red, LightShape::Circle),
            (LightColor::Green, LightShape::Circle),
        ],
        DisplayMode::Star => vec![
            (LightColor::Red, LightShape::Star),
            (LightColor::Green, LightShape::Star),
            (LightColor::Red, LightShape::Star),
            (LightColor::Green, LightShape::Star),
            (LightColor::Yellow, LightShape::Star),
            (LightColor::White, LightShape::Star),
            (LightColor::Red, LightShape::Star),
            (LightColor::Green, LightShape::Star),
        ],
        DisplayMode::CandyCane => vec![
            (LightColor::Red, LightShape::CandyCane),
            (LightColor::Green, LightShape::CandyCane),
            (LightColor::Red, LightShape::CandyCane),
            (LightColor::Green, LightShape::CandyCane),
            (LightColor::Yellow, LightShape::CandyCane),
            (LightColor::White, LightShape::CandyCane),
            (LightColor::Red, LightShape::CandyCane),
            (LightColor::Green, LightShape::CandyCane),
        ],
        DisplayMode::Bell => vec![
            (LightColor::Red, LightShape::Bell),
            (LightColor::Green, LightShape::Bell),
            (LightColor::Red, LightShape::Bell),
            (LightColor::Green, LightShape::Bell),
            (LightColor::Yellow, LightShape::Bell),
            (LightColor::White, LightShape::Bell),
            (LightColor::Red, LightShape::Bell),
            (LightColor::Green, LightShape::Bell),
        ],
        DisplayMode::Angel => vec![
            (LightColor::Red, LightShape::Angel),
            (LightColor::Green, LightShape::Angel),
            (LightColor::Red, LightShape::Angel),
            (LightColor::Green, LightShape::Angel),
            (LightColor::Yellow, LightShape::Angel),
            (LightColor::White, LightShape::Angel),
            (LightColor::Red, LightShape::Angel),
            (LightColor::Green, LightShape::Angel),
        ],
        DisplayMode::Reindeer => vec![
            (LightColor::Red, LightShape::Reindeer),
            (LightColor::Green, LightShape::Reindeer),
            (LightColor::Red, LightShape::Reindeer),
            (LightColor::Green, LightShape::Reindeer),
            (LightColor::Yellow, LightShape::Reindeer),
            (LightColor::White, LightShape::Reindeer),
            (LightColor::Red, LightShape::Reindeer),
            (LightColor::Green, LightShape::Reindeer),
        ],
        DisplayMode::Tree => vec![
            (LightColor::Red, LightShape::Tree),
            (LightColor::Green, LightShape::Tree),
            (LightColor::Red, LightShape::Tree),
            (LightColor::Green, LightShape::Tree),
            (LightColor::Yellow, LightShape::Tree),
            (LightColor::White, LightShape::Tree),
            (LightColor::Red, LightShape::Tree),
            (LightColor::Green, LightShape::Tree),
        ],
    };

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
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>()
                && let Ok(value) = input.value().parse::<i32>()
                && (1..=5).contains(&value)
            {
                speed.set(value);
            }
        })
    };

    let on_mode_change = {
        let mode = mode.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let new_mode = match select.value().as_str() {
                    "Circle" => DisplayMode::Circle,
                    "Star" => DisplayMode::Star,
                    "CandyCane" => DisplayMode::CandyCane,
                    "Bell" => DisplayMode::Bell,
                    "Angel" => DisplayMode::Angel,
                    "Reindeer" => DisplayMode::Reindeer,
                    "Tree" => DisplayMode::Tree,
                    _ => DisplayMode::Mixed,
                };
                mode.set(new_mode);
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
                    lights.iter().enumerate().map(|(i, (color, shape))| {
                        let delay = color.animation_delay(i);
                        html! {
                            <Light
                                color={color.clone()}
                                shape={shape.clone()}
                                is_playing={*is_playing}
                                animation_duration={animation_duration.to_string()}
                                delay={delay.to_string()}
                            />
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
                    <div class="speed-control">
                        <label for="quantity">{ "Speed:" }</label>
                        <input
                            type="number"
                            id="quantity"
                            name="quantity"
                            min="1"
                            max="5"
                            value={speed.to_string()}
                            oninput={on_speed_change}
                        />
                        <div class="speed-display">
                            { format!("{}", *speed) }
                        </div>
                    </div>
                    <div class="mode-control">
                        <label for="mode">{ "Mode:" }</label>
                        <select id="mode" onchange={on_mode_change}>
                            <option value="Mixed" selected={matches!(*mode, DisplayMode::Mixed)}>{ "Mixed" }</option>
                            <option value="Circle" selected={matches!(*mode, DisplayMode::Circle)}>{ "Circle" }</option>
                            <option value="Star" selected={matches!(*mode, DisplayMode::Star)}>{ "Star" }</option>
                            <option value="CandyCane" selected={matches!(*mode, DisplayMode::CandyCane)}>{ "Candy Cane" }</option>
                            <option value="Bell" selected={matches!(*mode, DisplayMode::Bell)}>{ "Bell" }</option>
                            <option value="Angel" selected={matches!(*mode, DisplayMode::Angel)}>{ "Angel" }</option>
                            <option value="Reindeer" selected={matches!(*mode, DisplayMode::Reindeer)}>{ "Reindeer" }</option>
                            <option value="Tree" selected={matches!(*mode, DisplayMode::Tree)}>{ "Tree" }</option>
                        </select>
                    </div>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
