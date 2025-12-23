use web_sys::HtmlInputElement;
use yew::prelude::*;

// Build information injected at compile time
const BUILD_GIT_SHA: &str = env!("BUILD_GIT_SHA");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
const BUILD_HOST: &str = env!("BUILD_HOST");
const GITHUB_REPO: &str = "https://github.com/softwarewrighter/xmas-rs";

// Embed SVG content at compile time
const SVG_STOCKING: &str = include_str!("../images/stocking.svg");
const SVG_XMAS_TREE: &str = include_str!("../images/xmas-tree.svg");
const SVG_SNOWMAN: &str = include_str!("../images/snowman.svg");
const SVG_STAR: &str = include_str!("../images/star.svg");
const SVG_REINDEER: &str = include_str!("../images/reindeer.svg");
const SVG_SNOWFLAKE: &str = include_str!("../images/snowflake-1.svg");
const SVG_LIGHTBULB: &str = include_str!("../images/lightbulb.svg");
const SVG_ICICLE: &str = include_str!("../images/icicle.svg");
const SVG_CANDY_CANE: &str = include_str!("../images/candy-cane.svg");
const SVG_ORNAMENT: &str = include_str!("../images/ornament.svg");
const SVG_SNOWMAN2: &str = include_str!("../images/snowman-2.svg");
const SVG_SANTA_SLEIGH: &str = include_str!("../images/santa-sleigh.svg");
const SVG_WREATH: &str = include_str!("../images/xmas-wreath.svg");

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum LightColor {
    Red,
    Green,
    White,
    Yellow,
    Blue,
    Purple,
    Orange,
    Pink,
}

impl LightColor {
    fn as_class(self) -> &'static str {
        match self {
            LightColor::Red => "red",
            LightColor::Green => "green",
            LightColor::White => "white",
            LightColor::Yellow => "yellow",
            LightColor::Blue => "blue",
            LightColor::Purple => "purple",
            LightColor::Orange => "orange",
            LightColor::Pink => "pink",
        }
    }

    fn as_rgb(self) -> &'static str {
        match self {
            LightColor::Red => "#ff0000",
            LightColor::Green => "#00cc00",
            LightColor::White => "#ffffff",
            LightColor::Yellow => "#ffdd00",
            LightColor::Blue => "#0088ff",
            LightColor::Purple => "#aa00ff",
            LightColor::Orange => "#ff8800",
            LightColor::Pink => "#ff66cc",
        }
    }

    fn all() -> Vec<LightColor> {
        vec![
            LightColor::Red,
            LightColor::Green,
            LightColor::White,
            LightColor::Yellow,
            LightColor::Blue,
            LightColor::Purple,
            LightColor::Orange,
            LightColor::Pink,
        ]
    }

    fn name(&self) -> &str {
        match self {
            LightColor::Red => "Red",
            LightColor::Green => "Green",
            LightColor::White => "White",
            LightColor::Yellow => "Yellow",
            LightColor::Blue => "Blue",
            LightColor::Purple => "Purple",
            LightColor::Orange => "Orange",
            LightColor::Pink => "Pink",
        }
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum ImageShape {
    Stocking,
    XmasTree,
    Snowman,
    Star,
    Reindeer,
    Snowflake,
    Lightbulb,
    Icicle,
    CandyCane,
    Ornament,
    Snowman2,
    SantaSleigh,
    Wreath,
}

impl ImageShape {
    fn all() -> Vec<ImageShape> {
        // Alphabetical order by display name
        vec![
            ImageShape::CandyCane,
            ImageShape::XmasTree,
            ImageShape::Icicle,
            ImageShape::Lightbulb,
            ImageShape::Ornament,
            ImageShape::Reindeer,
            ImageShape::SantaSleigh,
            ImageShape::Snowflake,
            ImageShape::Snowman,
            ImageShape::Snowman2,
            ImageShape::Star,
            ImageShape::Stocking,
            ImageShape::Wreath,
        ]
    }

    fn name(&self) -> &str {
        match self {
            ImageShape::Stocking => "Stocking",
            ImageShape::XmasTree => "Christmas Tree",
            ImageShape::Snowman => "Snowman",
            ImageShape::Star => "Star",
            ImageShape::Reindeer => "Reindeer",
            ImageShape::Snowflake => "Snowflake",
            ImageShape::Lightbulb => "Light Bulb",
            ImageShape::Icicle => "Icicle",
            ImageShape::CandyCane => "Candy Cane",
            ImageShape::Ornament => "Ornament",
            ImageShape::Snowman2 => "Snowman 2",
            ImageShape::SantaSleigh => "Santa's Sleigh",
            ImageShape::Wreath => "Wreath",
        }
    }

    fn svg_content(&self) -> &'static str {
        match self {
            ImageShape::Stocking => SVG_STOCKING,
            ImageShape::XmasTree => SVG_XMAS_TREE,
            ImageShape::Snowman => SVG_SNOWMAN,
            ImageShape::Star => SVG_STAR,
            ImageShape::Reindeer => SVG_REINDEER,
            ImageShape::Snowflake => SVG_SNOWFLAKE,
            ImageShape::Lightbulb => SVG_LIGHTBULB,
            ImageShape::Icicle => SVG_ICICLE,
            ImageShape::CandyCane => SVG_CANDY_CANE,
            ImageShape::Ornament => SVG_ORNAMENT,
            ImageShape::Snowman2 => SVG_SNOWMAN2,
            ImageShape::SantaSleigh => SVG_SANTA_SLEIGH,
            ImageShape::Wreath => SVG_WREATH,
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum SelectionMode {
    Repeat1, // 1 image repeated 8 times
    Repeat2, // 2 images alternating
    Repeat3, // 3 images repeating
    Repeat4, // 4 images repeating
    Manual8, // 8 images selected manually
    Random8, // 8 random images
}

impl SelectionMode {
    fn name(&self) -> &str {
        match self {
            SelectionMode::Repeat1 => "1 Image (Repeat)",
            SelectionMode::Repeat2 => "2 Images (Alternate)",
            SelectionMode::Repeat3 => "3 Images (Cycle)",
            SelectionMode::Repeat4 => "4 Images (Cycle)",
            SelectionMode::Manual8 => "8 Images (Manual)",
            SelectionMode::Random8 => "8 Images (Random)",
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum ColorMode {
    PerImage,    // Each position has its own color
    AllSame,     // All images same color
    Alternating, // Even/odd alternate between two colors
    Rainbow,     // Cycle through all colors
    RepeatMatch, // Repeated images get same color
}

impl ColorMode {
    fn name(&self) -> &str {
        match self {
            ColorMode::PerImage => "Per Image",
            ColorMode::AllSame => "All Same",
            ColorMode::Alternating => "Alternating",
            ColorMode::Rainbow => "Rainbow",
            ColorMode::RepeatMatch => "Match Repeats",
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum AnimationMode {
    Solid,    // Always on
    Blink,    // Classic blink
    Fade,     // Fade in and out
    Sequence, // Lights turn on/off in sequence
    Wave,     // Color wave through lights
    Chase,    // Chasing pattern
}

impl AnimationMode {
    fn name(&self) -> &str {
        match self {
            AnimationMode::Solid => "Solid On",
            AnimationMode::Blink => "Blink",
            AnimationMode::Fade => "Fade",
            AnimationMode::Sequence => "Sequence",
            AnimationMode::Wave => "Wave",
            AnimationMode::Chase => "Chase",
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum ViewMode {
    Settings,
    Demo,
}

impl ViewMode {
    fn name(&self) -> &str {
        match self {
            ViewMode::Settings => "Settings",
            ViewMode::Demo => "Demo",
        }
    }
}

#[derive(Properties, PartialEq)]
struct ChainLightProps {
    shape: ImageShape,
    color: LightColor,
    index: usize,
    animation_mode: AnimationMode,
    speed: i32,
    is_playing: bool,
    #[prop_or(0)]
    rotation: i32, // Rotation in degrees for side lights
}

#[function_component(ChainLight)]
fn chain_light(props: &ChainLightProps) -> Html {
    let color_rgb = props.color.as_rgb();
    let color_class = props.color.as_class();

    // Get animation class based on mode
    let animation_class = match props.animation_mode {
        AnimationMode::Solid => "anim-solid",
        AnimationMode::Blink => "anim-blink",
        AnimationMode::Fade => "anim-fade",
        AnimationMode::Sequence => "anim-sequence",
        AnimationMode::Wave => "anim-wave",
        AnimationMode::Chase => "anim-chase",
    };

    let paused = if !props.is_playing { " paused" } else { "" };
    let full_class = format!("chain-light {color_class} {animation_class}{paused}");

    // Calculate animation delay based on index for sequence effects
    let delay = match props.animation_mode {
        AnimationMode::Sequence | AnimationMode::Chase | AnimationMode::Wave => {
            format!("{}ms", props.index * 150)
        }
        AnimationMode::Blink | AnimationMode::Fade => {
            format!("{}ms", (props.index % 4) * 250)
        }
        AnimationMode::Solid => "0ms".to_string(),
    };

    // Calculate animation duration based on speed
    let duration = match props.speed {
        1 => "2000ms",
        2 => "1500ms",
        3 => "1000ms",
        4 => "750ms",
        5 => "500ms",
        _ => "1000ms",
    };

    // Modify SVG to apply color - replace fill="#000000" with the selected color
    let svg_content = props.shape.svg_content();
    let colored_svg = svg_content
        .replace("fill=\"#000000\"", &format!("fill=\"{color_rgb}\""))
        .replace("fill:#000000", &format!("fill:{color_rgb}"));

    // Parse the SVG as raw HTML
    let svg_node = Html::from_html_unchecked(AttrValue::from(colored_svg));

    let rotation_style = if props.rotation != 0 {
        format!("; transform: rotate({}deg)", props.rotation)
    } else {
        String::new()
    };

    html! {
        <div
            class={full_class}
            style={format!("--anim-delay: {delay}; --anim-duration: {duration}{rotation_style}")}
        >
            <div class="svg-container">
                {svg_node}
            </div>
        </div>
    }
}

// Simple pseudo-random using index as seed
fn pseudo_random(seed: usize, max: usize) -> usize {
    ((seed.wrapping_mul(1103515245).wrapping_add(12345)) / 65536) % max
}

#[function_component(App)]
fn app() -> Html {
    // State for view mode (Settings vs Demo)
    let view_mode = use_state(|| ViewMode::Settings);
    let christmas_message = use_state(|| "Merry Christmas!".to_string());

    // State for light chain configuration
    let selection_mode = use_state(|| SelectionMode::Repeat4);
    let color_mode = use_state(|| ColorMode::Rainbow);
    let animation_mode = use_state(|| AnimationMode::Fade);
    let speed = use_state(|| 3);
    let is_playing = use_state(|| true);

    // Selected images for each slot (up to 8)
    let selected_images = use_state(|| {
        vec![
            ImageShape::Stocking,
            ImageShape::XmasTree,
            ImageShape::Snowman,
            ImageShape::Star,
            ImageShape::Reindeer,
            ImageShape::Snowflake,
            ImageShape::Lightbulb,
            ImageShape::Icicle,
        ]
    });

    // Selected colors
    let primary_color = use_state(|| LightColor::Red);
    let secondary_color = use_state(|| LightColor::Green);

    // Random seed for random mode
    let random_seed = use_state(|| 42_usize);

    // Build the light chain based on selection mode
    let chain: Vec<(ImageShape, LightColor)> = {
        let images = (*selected_images).clone();
        let all_shapes = ImageShape::all();

        // Get 8 shapes based on selection mode
        let shapes: Vec<ImageShape> = match *selection_mode {
            SelectionMode::Repeat1 => {
                vec![images[0]; 8]
            }
            SelectionMode::Repeat2 => (0..8).map(|i| images[i % 2]).collect(),
            SelectionMode::Repeat3 => (0..8).map(|i| images[i % 3]).collect(),
            SelectionMode::Repeat4 => (0..8).map(|i| images[i % 4]).collect(),
            SelectionMode::Manual8 => images.iter().take(8).cloned().collect(),
            SelectionMode::Random8 => {
                let seed = *random_seed;
                (0..8)
                    .map(|i| {
                        let idx = pseudo_random(seed + i * 7, all_shapes.len());
                        all_shapes[idx]
                    })
                    .collect()
            }
        };

        // Apply colors based on color mode
        let all_colors = LightColor::all();
        shapes
            .iter()
            .enumerate()
            .map(|(i, shape)| {
                let color = match *color_mode {
                    ColorMode::PerImage => all_colors[i % all_colors.len()],
                    ColorMode::AllSame => *primary_color,
                    ColorMode::Alternating => {
                        if i.is_multiple_of(2) {
                            *primary_color
                        } else {
                            *secondary_color
                        }
                    }
                    ColorMode::Rainbow => all_colors[i % all_colors.len()],
                    ColorMode::RepeatMatch => {
                        // Same shape gets same color
                        let shape_idx = all_shapes.iter().position(|s| s == shape).unwrap_or(0);
                        all_colors[shape_idx % all_colors.len()]
                    }
                };
                (*shape, color)
            })
            .collect()
    };

    // Event handlers
    let on_view_mode_change = {
        let view_mode = view_mode.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let mode = match select.value().as_str() {
                    "Settings" => ViewMode::Settings,
                    "Demo" => ViewMode::Demo,
                    _ => ViewMode::Settings,
                };
                view_mode.set(mode);
            }
        })
    };

    let on_message_change = {
        let christmas_message = christmas_message.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                christmas_message.set(input.value());
            }
        })
    };

    let on_play = {
        let is_playing = is_playing.clone();
        Callback::from(move |_| is_playing.set(true))
    };

    let on_stop = {
        let is_playing = is_playing.clone();
        Callback::from(move |_| is_playing.set(false))
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

    let on_selection_mode_change = {
        let selection_mode = selection_mode.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let mode = match select.value().as_str() {
                    "Repeat1" => SelectionMode::Repeat1,
                    "Repeat2" => SelectionMode::Repeat2,
                    "Repeat3" => SelectionMode::Repeat3,
                    "Repeat4" => SelectionMode::Repeat4,
                    "Manual8" => SelectionMode::Manual8,
                    "Random8" => SelectionMode::Random8,
                    _ => SelectionMode::Repeat4,
                };
                selection_mode.set(mode);
            }
        })
    };

    let on_color_mode_change = {
        let color_mode = color_mode.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let mode = match select.value().as_str() {
                    "PerImage" => ColorMode::PerImage,
                    "AllSame" => ColorMode::AllSame,
                    "Alternating" => ColorMode::Alternating,
                    "Rainbow" => ColorMode::Rainbow,
                    "RepeatMatch" => ColorMode::RepeatMatch,
                    _ => ColorMode::Rainbow,
                };
                color_mode.set(mode);
            }
        })
    };

    let on_animation_mode_change = {
        let animation_mode = animation_mode.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let mode = match select.value().as_str() {
                    "Solid" => AnimationMode::Solid,
                    "Blink" => AnimationMode::Blink,
                    "Fade" => AnimationMode::Fade,
                    "Sequence" => AnimationMode::Sequence,
                    "Wave" => AnimationMode::Wave,
                    "Chase" => AnimationMode::Chase,
                    _ => AnimationMode::Fade,
                };
                animation_mode.set(mode);
            }
        })
    };

    let on_primary_color_change = {
        let primary_color = primary_color.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let color = match select.value().as_str() {
                    "Red" => LightColor::Red,
                    "Green" => LightColor::Green,
                    "White" => LightColor::White,
                    "Yellow" => LightColor::Yellow,
                    "Blue" => LightColor::Blue,
                    "Purple" => LightColor::Purple,
                    "Orange" => LightColor::Orange,
                    "Pink" => LightColor::Pink,
                    _ => LightColor::Red,
                };
                primary_color.set(color);
            }
        })
    };

    let on_secondary_color_change = {
        let secondary_color = secondary_color.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let color = match select.value().as_str() {
                    "Red" => LightColor::Red,
                    "Green" => LightColor::Green,
                    "White" => LightColor::White,
                    "Yellow" => LightColor::Yellow,
                    "Blue" => LightColor::Blue,
                    "Purple" => LightColor::Purple,
                    "Orange" => LightColor::Orange,
                    "Pink" => LightColor::Pink,
                    _ => LightColor::Green,
                };
                secondary_color.set(color);
            }
        })
    };

    let on_randomize = {
        let random_seed = random_seed.clone();
        Callback::from(move |_| {
            random_seed.set((*random_seed).wrapping_add(1));
        })
    };

    // Image selection handlers (for slots 0-7)
    let make_image_change_handler = |slot: usize| {
        let selected_images = selected_images.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let shape = match select.value().as_str() {
                    "Stocking" => ImageShape::Stocking,
                    "XmasTree" => ImageShape::XmasTree,
                    "Snowman" => ImageShape::Snowman,
                    "Star" => ImageShape::Star,
                    "Reindeer" => ImageShape::Reindeer,
                    "Snowflake" => ImageShape::Snowflake,
                    "Lightbulb" => ImageShape::Lightbulb,
                    "Icicle" => ImageShape::Icicle,
                    "CandyCane" => ImageShape::CandyCane,
                    "Ornament" => ImageShape::Ornament,
                    "Snowman2" => ImageShape::Snowman2,
                    "SantaSleigh" => ImageShape::SantaSleigh,
                    "Wreath" => ImageShape::Wreath,
                    _ => ImageShape::Stocking,
                };
                let mut new_images = (*selected_images).clone();
                if slot < new_images.len() {
                    new_images[slot] = shape;
                }
                selected_images.set(new_images);
            }
        })
    };

    // Determine how many image selectors to show
    let num_selectors = match *selection_mode {
        SelectionMode::Repeat1 => 1,
        SelectionMode::Repeat2 => 2,
        SelectionMode::Repeat3 => 3,
        SelectionMode::Repeat4 => 4,
        SelectionMode::Manual8 => 8,
        SelectionMode::Random8 => 0,
    };

    // Build extended chain for demo mode (40 lights: 15 top + 5 right + 15 bottom + 5 left)
    let demo_chain: Vec<(ImageShape, LightColor)> = {
        let images = (*selected_images).clone();
        let all_shapes = ImageShape::all();

        // Generate 40 shapes for demo mode frame
        let shapes: Vec<ImageShape> = match *selection_mode {
            SelectionMode::Repeat1 => vec![images[0]; 40],
            SelectionMode::Repeat2 => (0..40).map(|i| images[i % 2]).collect(),
            SelectionMode::Repeat3 => (0..40).map(|i| images[i % 3]).collect(),
            SelectionMode::Repeat4 => (0..40).map(|i| images[i % 4]).collect(),
            SelectionMode::Manual8 => (0..40).map(|i| images[i % 8]).collect(),
            SelectionMode::Random8 => {
                let seed = *random_seed;
                (0..40)
                    .map(|i| {
                        let idx = pseudo_random(seed + i * 7, all_shapes.len());
                        all_shapes[idx]
                    })
                    .collect()
            }
        };

        // Apply colors
        let all_colors = LightColor::all();
        shapes
            .iter()
            .enumerate()
            .map(|(i, shape)| {
                let color = match *color_mode {
                    ColorMode::PerImage => all_colors[i % all_colors.len()],
                    ColorMode::AllSame => *primary_color,
                    ColorMode::Alternating => {
                        if i.is_multiple_of(2) {
                            *primary_color
                        } else {
                            *secondary_color
                        }
                    }
                    ColorMode::Rainbow => all_colors[i % all_colors.len()],
                    ColorMode::RepeatMatch => {
                        let shape_idx = all_shapes.iter().position(|s| s == shape).unwrap_or(0);
                        all_colors[shape_idx % all_colors.len()]
                    }
                };
                (*shape, color)
            })
            .collect()
    };

    html! {
        <>
            // Header with view mode toggle
            <header class={classes!("app-header", matches!(*view_mode, ViewMode::Demo).then_some("demo-mode"))}>
                <h1 id="title">{ "Christmas Light Chain" }</h1>
                <div class="mode-control view-toggle">
                    <label for="view-mode">{ "View:" }</label>
                    <select id="view-mode" onchange={on_view_mode_change}>
                        <option value="Settings" selected={matches!(*view_mode, ViewMode::Settings)}>
                            { ViewMode::Settings.name() }
                        </option>
                        <option value="Demo" selected={matches!(*view_mode, ViewMode::Demo)}>
                            { ViewMode::Demo.name() }
                        </option>
                    </select>
                </div>
            </header>

            {
                if matches!(*view_mode, ViewMode::Demo) {
                    // Demo mode: message framed by lights
                    html! {
                        <div class="demo-container">
                            <div class="demo-frame">
                                // Top row (15 lights)
                                <div class="demo-row demo-top">
                                    { for demo_chain.iter().take(15).enumerate().map(|(i, (shape, color))| {
                                        html! {
                                            <ChainLight
                                                shape={*shape}
                                                color={*color}
                                                index={i}
                                                animation_mode={*animation_mode}
                                                speed={*speed}
                                                is_playing={*is_playing}
                                            />
                                        }
                                    })}
                                </div>

                                // Middle section with side columns and message
                                <div class="demo-middle">
                                    // Left column (5 lights) - indices go bottom-to-top for clockwise
                                    <div class="demo-column demo-left">
                                        { for demo_chain.iter().skip(35).take(5).enumerate().map(|(i, (shape, color))| {
                                            html! {
                                                <ChainLight
                                                    shape={*shape}
                                                    color={*color}
                                                    index={39 - i}
                                                    animation_mode={*animation_mode}
                                                    speed={*speed}
                                                    is_playing={*is_playing}
                                                    rotation={90}
                                                />
                                            }
                                        })}
                                    </div>

                                    // Message in the center
                                    <div class="demo-message">
                                        <span>{ (*christmas_message).clone() }</span>
                                    </div>

                                    // Right column (5 lights, top to bottom)
                                    <div class="demo-column demo-right">
                                        { for demo_chain.iter().skip(15).take(5).enumerate().map(|(i, (shape, color))| {
                                            html! {
                                                <ChainLight
                                                    shape={*shape}
                                                    color={*color}
                                                    index={15 + i}
                                                    animation_mode={*animation_mode}
                                                    speed={*speed}
                                                    is_playing={*is_playing}
                                                    rotation={-90}
                                                />
                                            }
                                        })}
                                    </div>
                                </div>

                                // Bottom row (15 lights) - indices go right-to-left for clockwise
                                <div class="demo-row demo-bottom">
                                    { for demo_chain.iter().skip(20).take(15).enumerate().map(|(i, (shape, color))| {
                                        html! {
                                            <ChainLight
                                                shape={*shape}
                                                color={*color}
                                                index={34 - i}
                                                animation_mode={*animation_mode}
                                                speed={*speed}
                                                is_playing={*is_playing}
                                                rotation={180}
                                            />
                                        }
                                    })}
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    // Settings mode: original UI
                    html! {
                        <>
                            <div class="main">
                                {
                                    chain.iter().enumerate().map(|(i, (shape, color))| {
                                        html! {
                                            <ChainLight
                                                shape={*shape}
                                                color={*color}
                                                index={i}
                                                animation_mode={*animation_mode}
                                                speed={*speed}
                                                is_playing={*is_playing}
                                            />
                                        }
                                    }).collect::<Html>()
                                }
                            </div>

                            <div class="utilities">
                                <div class="controls-grid">
                                    // Row 0: Message input
                                    <div class="control-group">
                                        <div class="message-control">
                                            <label for="christmas-message">{ "Demo Message:" }</label>
                                            <input
                                                type="text"
                                                id="christmas-message"
                                                value={(*christmas_message).clone()}
                                                oninput={on_message_change}
                                                placeholder="Enter your Christmas message"
                                            />
                                        </div>
                                    </div>

                                    // Row 1: On/Off and Speed
                                    <div class="control-group">
                                        <button id="play" onclick={on_play}>{ "On" }</button>
                                        <button id="stop" onclick={on_stop}>{ "Off" }</button>
                                        <div class="speed-control">
                                            <label for="speed">{ "Speed:" }</label>
                                            <input
                                                type="number"
                                                id="speed"
                                                min="1"
                                                max="5"
                                                value={speed.to_string()}
                                                oninput={on_speed_change}
                                            />
                                        </div>
                                    </div>

                                    // Row 2: Selection Mode and Animation Mode
                                    <div class="control-group">
                                        <div class="mode-control">
                                            <label for="selection-mode">{ "Chain:" }</label>
                                            <select id="selection-mode" onchange={on_selection_mode_change}>
                                                <option value="Repeat1" selected={matches!(*selection_mode, SelectionMode::Repeat1)}>
                                                    { SelectionMode::Repeat1.name() }
                                                </option>
                                                <option value="Repeat2" selected={matches!(*selection_mode, SelectionMode::Repeat2)}>
                                                    { SelectionMode::Repeat2.name() }
                                                </option>
                                                <option value="Repeat3" selected={matches!(*selection_mode, SelectionMode::Repeat3)}>
                                                    { SelectionMode::Repeat3.name() }
                                                </option>
                                                <option value="Repeat4" selected={matches!(*selection_mode, SelectionMode::Repeat4)}>
                                                    { SelectionMode::Repeat4.name() }
                                                </option>
                                                <option value="Manual8" selected={matches!(*selection_mode, SelectionMode::Manual8)}>
                                                    { SelectionMode::Manual8.name() }
                                                </option>
                                                <option value="Random8" selected={matches!(*selection_mode, SelectionMode::Random8)}>
                                                    { SelectionMode::Random8.name() }
                                                </option>
                                            </select>
                                        </div>

                                        <div class="mode-control">
                                            <label for="animation-mode">{ "Animation:" }</label>
                                            <select id="animation-mode" onchange={on_animation_mode_change}>
                                                <option value="Solid" selected={matches!(*animation_mode, AnimationMode::Solid)}>
                                                    { AnimationMode::Solid.name() }
                                                </option>
                                                <option value="Blink" selected={matches!(*animation_mode, AnimationMode::Blink)}>
                                                    { AnimationMode::Blink.name() }
                                                </option>
                                                <option value="Fade" selected={matches!(*animation_mode, AnimationMode::Fade)}>
                                                    { AnimationMode::Fade.name() }
                                                </option>
                                                <option value="Sequence" selected={matches!(*animation_mode, AnimationMode::Sequence)}>
                                                    { AnimationMode::Sequence.name() }
                                                </option>
                                                <option value="Wave" selected={matches!(*animation_mode, AnimationMode::Wave)}>
                                                    { AnimationMode::Wave.name() }
                                                </option>
                                                <option value="Chase" selected={matches!(*animation_mode, AnimationMode::Chase)}>
                                                    { AnimationMode::Chase.name() }
                                                </option>
                                            </select>
                                        </div>
                                    </div>

                                    // Row 3: Color Mode and Color Selection
                                    <div class="control-group">
                                        <div class="mode-control">
                                            <label for="color-mode">{ "Colors:" }</label>
                                            <select id="color-mode" onchange={on_color_mode_change}>
                                                <option value="Rainbow" selected={matches!(*color_mode, ColorMode::Rainbow)}>
                                                    { ColorMode::Rainbow.name() }
                                                </option>
                                                <option value="PerImage" selected={matches!(*color_mode, ColorMode::PerImage)}>
                                                    { ColorMode::PerImage.name() }
                                                </option>
                                                <option value="AllSame" selected={matches!(*color_mode, ColorMode::AllSame)}>
                                                    { ColorMode::AllSame.name() }
                                                </option>
                                                <option value="Alternating" selected={matches!(*color_mode, ColorMode::Alternating)}>
                                                    { ColorMode::Alternating.name() }
                                                </option>
                                                <option value="RepeatMatch" selected={matches!(*color_mode, ColorMode::RepeatMatch)}>
                                                    { ColorMode::RepeatMatch.name() }
                                                </option>
                                            </select>
                                        </div>

                                        {
                                            if matches!(*color_mode, ColorMode::AllSame | ColorMode::Alternating) {
                                                html! {
                                                    <div class="mode-control">
                                                        <label for="primary-color">{ "Color 1:" }</label>
                                                        <select id="primary-color" onchange={on_primary_color_change}>
                                                            { for LightColor::all().iter().map(|c| {
                                                                html! {
                                                                    <option
                                                                        value={format!("{c:?}").replace("LightColor::", "")}
                                                                        selected={*primary_color == *c}
                                                                    >
                                                                        { c.name() }
                                                                    </option>
                                                                }
                                                            })}
                                                        </select>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }

                                        {
                                            if matches!(*color_mode, ColorMode::Alternating) {
                                                html! {
                                                    <div class="mode-control">
                                                        <label for="secondary-color">{ "Color 2:" }</label>
                                                        <select id="secondary-color" onchange={on_secondary_color_change}>
                                                            { for LightColor::all().iter().map(|c| {
                                                                html! {
                                                                    <option
                                                                        value={format!("{c:?}").replace("LightColor::", "")}
                                                                        selected={*secondary_color == *c}
                                                                    >
                                                                        { c.name() }
                                                                    </option>
                                                                }
                                                            })}
                                                        </select>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>

                                    // Row 4: Image Selection
                                    {
                                        if num_selectors > 0 {
                                            html! {
                                                <div class="control-group image-selectors">
                                                    <label class="section-label">{ "Select Images:" }</label>
                                                    { for (0..num_selectors).map(|i| {
                                                        let handler = make_image_change_handler(i);
                                                        let current = selected_images.get(i).cloned().unwrap_or(ImageShape::Stocking);
                                                        html! {
                                                            <div class="mode-control compact">
                                                                <label>{ format!("#{}", i + 1) }</label>
                                                                <select onchange={handler}>
                                                                    { for ImageShape::all().iter().map(|s| {
                                                                        html! {
                                                                            <option
                                                                                value={format!("{s:?}")}
                                                                                selected={current == *s}
                                                                            >
                                                                                { s.name() }
                                                                            </option>
                                                                        }
                                                                    })}
                                                                </select>
                                                            </div>
                                                        }
                                                    })}
                                                </div>
                                            }
                                        } else if matches!(*selection_mode, SelectionMode::Random8) {
                                            html! {
                                                <div class="control-group">
                                                    <button class="randomize-btn" onclick={on_randomize}>
                                                        { "Randomize" }
                                                    </button>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                            </div>
                        </>
                    }
                }
            }

            <footer class="footer">
                <div class="footer-content">
                    <div class="copyright">
                        { "Copyright © 2025 Michael A Wright | " }
                        <a href={format!("{GITHUB_REPO}/blob/main/LICENSE")} target="_blank" rel="noopener noreferrer">
                            { "MIT License" }
                        </a>
                    </div>
                    <div class="build-info">
                        <span title="Build Host">{ format!("Host: {} | ", BUILD_HOST) }</span>
                        <span title="Git Commit SHA">{ format!("Commit: {} | ", BUILD_GIT_SHA) }</span>
                        <span title="Build Timestamp">{ format!("Built: {}", BUILD_TIMESTAMP) }</span>
                    </div>
                </div>
            </footer>

            <a href={GITHUB_REPO} class="github-corner" target="_blank" rel="noopener noreferrer" title="Source Code">
                <svg width="80" height="80" viewBox="0 0 250 250" aria-hidden="true">
                    <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z" fill="#563260"></path>
                    <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
                    <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
                </svg>
            </a>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
