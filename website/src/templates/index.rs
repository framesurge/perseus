use crate::components::container::Container;
use crate::components::github_svg::GITHUB_SVG;
use crate::components::header::HeaderProps;
#[perseus::engine]
use perseus::RenderFnResultWithCause;
use perseus::{link, t, Html, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

#[derive(Prop)]
struct IndexTileProps<G: Html> {
    /// The HTML ID of this tile.
    id: String,
    /// Any additional styling classes (used for the background). These will be
    /// applied to the outer tile wrapper `<div>`.
    classes: String,
    /// The contents of the block containing text.
    text_block: View<G>,
    /// The contents of the tile's code example.
    code: Example,
    /// The language of the code example, for syntax highlighting.
    code_lang: String,
    /// The order of the code example and text.
    order: TileOrder,
    /// A custom replacement for the supplement that would usually store code.
    /// This can be used to show an image or the like instead.
    custom_supplement: Option<View<G>>,
    /// Any extra elements to be placed below the text and supplement blocks.
    extra: Option<View<G>>,
    /// The type of navigation buttons between sections that this section should
    /// have.
    #[allow(dead_code)] // Pending further work
    nav_buttons: NavButtons,
}
#[derive(Clone)]
enum TileOrder {
    TextLeft,
    TextRight,
}
// Each of the attached strings here is an HTML ID of the section to scroll to
// on a click
enum NavButtons {
    Top(String),
    Bottom(String),
    Both(String, String),
}

/// A responsive tile component for the index page. This uses a wrapper
/// component for background images as `::before`s, for enabling filter
/// application for dark mode.
#[component]
fn IndexTile<G: Html>(cx: Scope, props: IndexTileProps<G>) -> View<G> {
    let order = create_ref(cx, props.order);

    // This would usually store the code example, but that can be overridden
    let supplement_view = if let Some(supplement) = props.custom_supplement {
        supplement
    } else {
        // If we have excerpts, we'll display those preferentially, and let the user
        // switch to the full version with a button
        match props.code {
            Example::Simple(example) => view! { cx,
                pre(class = "!rounded-2xl !p-8 !text-sm permadark max-h-[80vh]") {
                    code(class = format!("language-{}", props.code_lang)) {
                        (example)
                    }
                }
            },
            Example::WithExcerpts { full, excerpts } => {
                // We use a separate signal for the button states so they don't lag with the
                // blur transition
                let show_full_button = create_signal(cx, false);
                let show_full_rc = create_rc_signal(false);
                let show_full = create_ref(cx, show_full_rc.clone());
                #[allow(unused_variables)] // Wasm-only
                let show_full_1 = show_full_rc.clone();
                #[allow(unused_variables)] // Wasm-only
                let show_full_2 = show_full_rc;
                let example = create_memo(cx, move || {
                    if *show_full.get() {
                        full.to_string()
                    } else {
                        excerpts.to_string()
                    }
                });
                let pre = NodeRef::new();
                let pre_noderef = pre.clone();
                let pre_noderef_2 = pre.clone();

                view! { cx,
                    div(
                        class = "bg-[#313346] rounded-2xl h-[80vh] flex flex-col overflow-hidden"
                    ) {
                        div(
                            class = "w-full flex justify-around"
                        ) {
                            // Shows excerpts
                            button(
                                class = format!(
                                    "rounded-full p-3 {} min-w-[30%] text-lg transition-colors duration-200 font-semibold",
                                    if *show_full_button.get() { "bg-indigo-500 dark:bg-indigo-600" } else { "bg-indigo-600 dark:bg-indigo-800" }
                                ),
                                on:click = move |_| {
                                   // Only do anything if we aren't already showing the right thing (we care because we re-highlight everything)
                                    if *show_full.get_untracked() {
                                        show_full_button.set(false);
                                        // Blur the code
                                        let pre_node = pre_noderef.get::<DomNode>();
                                        pre_node.add_class("blur");
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            let show_full_1 = show_full_1.clone();
                                            let timer = gloo_timers::callback::Timeout::new(150, move || {
                                                show_full_1.set(false);
                                                // Changing the text breaks syntax highlighting, so re-initialize it with PrismJS
                                                // This shouldn't break the app if it fails
                                                let _ = js_sys::eval("window.Prism.highlightAll()");
                                                pre_node.remove_class("blur");
                                            });
                                            timer.forget();
                                        }
                                    }
                                }
                            ) { (t!("index-example-switcher.excerpts", cx)) }
                            // Shows full text
                            button(
                                class = format!(
                                    "rounded-full p-3 {} min-w-[30%] text-lg transition-colors duration-200 font-semibold",
                                    if *show_full_button.get() { "bg-indigo-600 dark:bg-indigo-800" } else { "bg-indigo-500 dark:bg-indigo-600" }
                                ),
                                on:click = move |_| {
                                    // Only do anything if we aren't already showing the right thing (we care because we re-highlight everything)
                                    if !*show_full.get_untracked() {
                                        show_full_button.set(true);
                                        // Blur the code
                                        let pre_node = pre_noderef_2.get::<DomNode>();
                                        pre_node.add_class("blur");
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            let show_full_2 = show_full_2.clone();
                                            let timer = gloo_timers::callback::Timeout::new(150, move || {
                                                show_full_2.set(true);
                                                // Changing the text breaks syntax highlighting, so re-initialize it with PrismJS
                                                // This shouldn't break the app if it fails
                                                let _ = js_sys::eval("window.Prism.highlightAll()");
                                                pre_node.remove_class("blur");
                                            });
                                            timer.forget();
                                        }
                                    }
                                }
                            ) { (t!("index-example-switcher.full", cx)) }
                        }
                        // We need this div so our styles can apply it to the `.code-toolbar` created by Prism
                        pre(ref = pre, class = "!rounded-2xl !p-8 !text-[0.85rem] !m-0 permadark overflow-y-auto h-full transition-[filter] duration-100") {
                            code(class = format!("language-{}", props.code_lang)) {
                                (example.get())
                            }
                        }
                    }
                }
            }
        }
    };
    let has_extra = props.extra.is_some();
    let extra_view = match props.extra {
        Some(view) => view,
        None => View::empty(),
    };

    // Each of these tiles will be one screen high on desktop, and two on mobile
    // (the second for the code example)
    view! { cx,
        div(
            class = format!(
                "tile-outer relative {}",
                props.classes,
            )
        ) {
            div(
                // Generic styles go here (the Flexbox here just centers everything vertically/horizontally)
                class = format!(
                    "{} relative lg:h-[102vh] lg:px-8 xl:px-12 2xl:px-24 3xl:px-40 text-white",
                    // If this tile has extra content at the bottom, we need three screens on mobile, otherwise two
                    if has_extra {
                        "h-[306vh]"
                    } else {
                        "h-[204vh]"
                    }
                ),
                id = format!("{}-tile", props.id)
            ) {
                div(
                    // This grid is responsible for aligning the text block and supplement block next to each other, with the extra stuff below them
                    class = format!(
                        "grid w-full h-full items-center {}",
                        if has_extra {
                            // We need a fancy grid alignment on desktop if we have extra content
                            "grid-cols-1 grid-rows-3 lg:grid-rows-[75%_25%] lg:grid-cols-2"
                        } else {
                            "grid-cols-1 grid-rows-2 lg:grid-rows-1 lg:grid-cols-2"
                        }
                    )
                ) {
                    // Text block
                    div(
                        class = format!(
                            "col-span-1 row-span-1 row-start-1 h-[102vh] lg:h-auto lg:bg-none flex lg:block justify-center text-center items-center flex-col lg:max-w-[50rem] {}",
                            // We need to apply padding in the right direction based on the order of items
                            // We also assign the grid column to use based on this, which allows us to invert the item order
                            if let TileOrder::TextLeft = order {
                                "lg:pr-4 xl:pr-8 lg:text-left lg:col-start-1 lg:justify-self-start"
                            } else {
                                "lg:pl-4 xl:pl-8 lg:text-right lg:col-start-2 lg:justify-self-end"
                            }
                        )
                    ) {
                        (props.text_block)
                    }
                    // Supplement block (usu. code example)
                    div(
                        class = format!(
                            "col-span-1 row-span-1 row-start-2 w-full lg:row-start-1 overflow-auto h-[102vh] lg:h-auto flex lg:block items-center p-6 lg:p-0 {}",
                            // As above, we assign the grid column based on the direction of text
                            if let TileOrder::TextLeft = order {
                                "lg:col-start-2"
                            } else {
                                "lg:col-start-1"
                            }
                        )
                    ) {
                        (supplement_view)
                    }
                    // Any extra content would be placed below the text and supplement blocks
                    // if it doesn't exist, it should be hidden
                    div(
                        class = format!(
                            "col-span-1 lg:col-span-2 {}",
                            if !has_extra {
                                "hidden"
                            } else {
                                ""
                            }
                        )
                    ) {
                        (extra_view)
                    }
                }
            }
        }
    }
}

#[derive(Prop)]
struct AnimatedCircularProgressBarProps {
    percent: u32,
    label: String,
}

/// A circular progress bar that will animate from 0 to the given value
/// automatically when it comes into the user's view.
#[component]
fn AnimatedCircularProgressBar<G: Html>(
    cx: Scope,
    props: AnimatedCircularProgressBarProps,
) -> View<G> {
    const STROKE: f32 = 8.0;
    const RADIUS: f32 = 60.0;

    let normalized_radius = RADIUS - STROKE * 2.0;
    let circumference = normalized_radius * 2.0 * std::f32::consts::PI;

    // This represents the percentage around the circle we've gotten to (we'll
    // animate from nothing to here)
    let offset = circumference - (props.percent as f32 / 100.0) * circumference;

    let elem = create_node_ref(cx);

    // Define a callback to be executed on scroll that will check if this element
    // has passed into view If it has, then we'll re-play the animation
    // We only do this after the component has been mounted (`NodeRef` usage)
    // BUG This doesn't work in Chrome...
    #[cfg(target_arch = "wasm32")]
    on_mount(cx, || {
        use wasm_bindgen::prelude::Closure;
        use wasm_bindgen::JsCast;

        let raw_elem: Element = elem.get::<DomNode>().unchecked_into();
        // Get the `<circle>` element to animate
        let svg_elem = raw_elem
            .children()
            .item(0)
            .unwrap()
            .children()
            .item(1)
            .unwrap();
        let svg_elem_clone = svg_elem.clone(); // We need this for observation

        let document = web_sys::window().unwrap().document().unwrap();
        // // This is the container on the page that actually scrolls (everything else
        // has height 100%)
        let scroll_container = document.get_element_by_id("scroll-container").unwrap();

        let intersection_handler = Closure::<dyn Fn(Vec<IntersectionObserverEntry>)>::new(
            move |entries: Vec<IntersectionObserverEntry>| {
                // Get the right entry, and make sure it's currently intersecting (this callback
                // will fire if it starts or stops intersecting)
                for entry in entries.iter() {
                    if entry.target() == svg_elem && entry.is_intersecting() {
                        // Check if the animation has already been started; if not, then start it
                        let old_classes = svg_elem.get_attribute("class").unwrap();
                        if !old_classes.contains("lh-gauge-arc-animation") {
                            let new_classes = format!("{} lh-gauge-arc-animation", &old_classes);
                            svg_elem.set_attribute("class", &new_classes).unwrap();
                        }
                    }
                }
            },
        );

        let intersection_observer = IntersectionObserver::new_with_options(
            intersection_handler.as_ref().unchecked_ref(),
            IntersectionObserverInit::new()
                .root(Some(&scroll_container))
                .threshold(&"1.0".into()),
        )
        .unwrap();

        // We need this to run forever (leaks memory)
        intersection_handler.forget();

        intersection_observer.observe(&svg_elem_clone);
    });

    view! { cx,
        div(
            class = "flex flex-col justify-center text-center max-w-min self-center",
            // We need to be able to track whether or not this is in the viewport
            ref = elem
        ) {
            svg(
                height = (RADIUS * 2.0).to_string(),
                width = (RADIUS * 2.0).to_string()
            ) {
                // Base background
                circle(
                    class = "opacity-10 fill-emerald-600 dark:fill-emerald-400 stroke-emerald-600 dark:stroke-emerald-400",
                    stroke-width = STROKE.to_string(),
                    r = normalized_radius.to_string(),
                    cx = RADIUS.to_string(),
                    cy = RADIUS.to_string()
                ) {}
                // Arc for the meter itself
                circle(
                    class = "stroke-emerald-600 dark:stroke-emerald-400 fill-transparent lh-gauge-arc",
                    stroke-width = STROKE.to_string(),
                    r = normalized_radius.to_string(),
                    cx = RADIUS.to_string(),
                    cy = RADIUS.to_string(),
                    style = format!(
                        r#"stroke-dasharray: {c}px {c}px;
                       stroke-dashoffset: {o}px;"#,
                        c = circumference,
                        o = offset
                    )
                ) {}
                text(
                    x="50%",
                    y="50%",
                    text-anchor = "middle",
                    dy = ".37em", // Eyeballed
                    class = "fill-emerald-600 dark:fill-emerald-400 text-[2rem] font-medium font-mono-lighthouse"
                ) { (props.percent.to_string()) }
            }
            span(
                class = "text-xl text-emerald-700 dark:text-emerald-400"
            ) { (props.label) }
        }
    }
}

#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page<G: Html>(cx: Scope, examples: CodeExamples) -> View<G> {
    // // Fix these on mobile
    // let nav_buttons = match props.nav_buttons {
    //     NavButtons::Both(prev_id, next_id) => view! { cx,
    //         button(
    //             // This is absolutely positioned relative to the greater tile
    //             // It then has a fixed width, which we can use to center it
    //             class = "absolute top-24 rounded-full bg-white text-black w-8
    // left-1/2 -ml-4 inline-flex justify-center p-1 hover:bg-gray-200
    // transition-colors duration-200",             on:click = move |_| {
    //                 let document =
    // web_sys::window().unwrap().document().unwrap();                 let elem
    // = document.get_element_by_id(&format!("{}-tile", prev_id)).unwrap();
    //                 elem.scroll_into_view();
    //             }
    //         ) {
    //             span(
    //                 dangerously_set_inner_html = UP_ARROW
    //             ) {}
    //         }
    //         button(
    //             // This is absolutely positioned relative to the greater tile
    //             // It then has a fixed width, which we can use to center it
    //             class = "absolute bottom-14 rounded-full bg-white text-black w-8
    // left-1/2 -ml-4 inline-flex justify-center p-1 hover:bg-gray-200
    // transition-colors duration-200",             on:click = move |_| {
    //                 let document =
    // web_sys::window().unwrap().document().unwrap();                 let elem
    // = document.get_element_by_id(&format!("{}-tile", next_id)).unwrap();
    //                 elem.scroll_into_view();
    //             }
    //         ) {
    //             span(
    //                 dangerously_set_inner_html = DOWN_ARROW
    //             ) {}
    //         }
    //     },
    //     NavButtons::Top(prev_id) => view! { cx,
    //         button(
    //             // This is absolutely positioned relative to the greater tile
    //             // It then has a fixed width, which we can use to center it
    //             class = "absolute top-24 rounded-full bg-white text-black w-8
    // left-1/2 -ml-4 inline-flex justify-center p-1 hover:bg-gray-200
    // transition-colors duration-200",             on:click = move |_| {
    //                 let document =
    // web_sys::window().unwrap().document().unwrap();                 let elem
    // = document.get_element_by_id(&format!("{}-tile", prev_id)).unwrap();
    //                 elem.scroll_into_view();
    //             }
    //         ) {
    //             span(
    //                 dangerously_set_inner_html = UP_ARROW
    //             ) {}
    //         }
    //     },
    //     NavButtons::Bottom(next_id) => view! { cx,
    //         button(
    //             // This is absolutely positioned relative to the greater tile
    //             // It then has a fixed width, which we can use to center it
    //             class = "absolute bottom-14 rounded-full bg-white text-black w-8
    // left-1/2 -ml-4 inline-flex justify-center p-1 hover:bg-gray-200
    // transition-colors duration-200",             on:click = move |_| {
    //                 let document =
    // web_sys::window().unwrap().document().unwrap();                 let elem
    // = document.get_element_by_id(&format!("{}-tile", next_id)).unwrap();
    //                 elem.scroll_into_view();
    //             }
    //         ) {
    //             span(
    //                 dangerously_set_inner_html = DOWN_ARROW
    //             ) {}
    //         }
    //     }
    // };

    view! { cx,
        Container(
            header = HeaderProps {
                title: t!("perseus", cx),
                text_color: "text-white".to_string(),
                menu_color: "bg-white".to_string(),
                mobile_nav_extension: View::empty(),
                menu_open: None,
            },
            footer = true,
        ) {
                // Introduction screen with the app-in-a-file example
                IndexTile(
                    id = "intro".to_string(),
                    classes = "tile-start".to_string(),
                    order = TileOrder::TextLeft,
                    custom_supplement = None,
                    text_block = view! { cx,
                        // NOTE These styles are deliberately different from the rest to prevent text overlaps
                        p(class = "uppercase text-4xl font-semibold sm:font-normal xs:text-5xl 2xl:text-[4.75rem] p-2 title-font mb-4") { (t!("index-intro.heading", cx)) }
                        div(class = "uppercase w-full flex items-center flex-col sm:flex-row justify-center lg:justify-start") {
                            a(
                                class = "bg-white dark:bg-black text-black dark:text-white sm:text-lg p-4 px-6 sm:px-8 mx-2 rounded-lg font-semibold uppercase mb-3 sm:mb-0 hover:shadow-white/50 dark:hover:shadow-black/50 hover:shadow-lg transition duration-200 hover:-translate-y-1 hover:scale-110 ease-in-out",
                                href = link!("/docs", cx)
                            ) { (t!("index-intro.get-started-button", cx)) }
                            a(
                                class = "bg-[#8085ff] dark:bg-[#787CFC] text-white sm:text-lg p-4 px-6 sm:px-8 mx-2 rounded-lg font-semibold inline-flex items-center hover:shadow-[#8085ff]/50 dark:hover:shadow-[#787CFC]/50 hover:shadow-lg transition-shadow duration-200",
                                href = "https://github.com/framesurge/perseus",
                                target = "_blank"
                            ) {
                                span(
                                    class = "mr-1",
                                    dangerously_set_inner_html = GITHUB_SVG
                                )
                                    span { (format!(" {}", t!("index-intro.github-button", cx))) }
                            }
                        }
                    },
                    code = examples.app_in_a_file,
                    code_lang = "rust".to_string(),
                    extra = None,
                    nav_buttons = NavButtons::Bottom("state_gen".to_string())
                )
                // State generation tile
                IndexTile(
                    id = "state_gen".to_string(),
                    classes = "tile-state-generation".to_string(),
                    order = TileOrder::TextRight,
                    custom_supplement = None,
                    text_block = view! { cx,
                        p(class = "uppercase text-4xl font-semibold sm:font-normal xs:text-5xl sm:text-6xl 2xl:text-[5rem] p-2 title-font mb-4") {
                            (t!("index-state-gen.heading", cx))
                        }
                        p(class = "text-xl md:text-2xl 2xl:text-3xl p-2") {
                            span(
                                dangerously_set_inner_html = &t!("index-state-gen.desc", cx)
                            ) {}
                        }
                    },
                    code = examples.state_generation,
                    code_lang = "rust".to_string(),
                    extra = None,
                    nav_buttons = NavButtons::Both("intro".to_string(), "i18n".to_string())
                )
                // I18n tile
                IndexTile(
                    id = "i18n".to_string(),
                    classes = "tile-i18n".to_string(),
                    order = TileOrder::TextLeft,
                    custom_supplement = None,
                    text_block = view! { cx,
                        p(class = "uppercase text-4xl font-semibold sm:font-normal xs:text-5xl sm:text-6xl 2xl:text-[5rem] p-2 title-font mb-4") {
                            div(class = "tooltip") {
                                span(id = "i18n-dotted-border") { (t!("index-i18n.heading.start", cx)) }
                                // We have to undo most of the title font stuff from the parent
                                span(class = "tooltip-text font-sans text-base normal-case tracking-normal") { (t!("index-i18n.heading.tooltip", cx)) }
                            }
                            span { (t!("index-i18n.heading.rest", cx)) }
                        }
                        p(class = "text-xl md:text-2xl 2xl:text-3xl p-2") {
                            span(
                                dangerously_set_inner_html = &t!("index-i18n.desc", cx)
                            ) {}
                        }
                    },
                    code = examples.i18n,
                    code_lang = "rust".to_string(),
                    extra = None,
                    nav_buttons = NavButtons::Both("state_gen".to_string(), "opts".to_string())
                )
                // Options tile
                IndexTile(
                    id = "opts".to_string(),
                    classes = "tile-options".to_string(),
                    order = TileOrder::TextRight,
                    custom_supplement = None,
                    text_block = view! { cx,
                        p(class = "uppercase text-4xl font-semibold sm:font-normal xs:text-5xl sm:text-6xl 2xl:text-[5rem] p-2 title-font mb-4") {
                            (t!("index-opts.heading", cx)) // TODO Best heading?
                        }
                        p(class = "text-xl md:text-2xl 2xl:text-3xl p-2") {
                            (t!("index-opts.desc", cx))
                        }
                    },
                    code = examples.cli,
                    code_lang = "sh".to_string(),
                    extra = None,
                    nav_buttons = NavButtons::Both("i18n".to_string(), "speed".to_string())
                )
                // Speed tile (uses an image of the Lighthouse scores instead of a code example)
                IndexTile(
                    id = "speed".to_string(),
                    classes = "tile-speed".to_string(),
                    order = TileOrder::TextLeft,
                    text_block = view! { cx,
                        p(class = "uppercase text-4xl font-semibold sm:font-normal xs:text-5xl sm:text-6xl 2xl:text-[5rem] p-2 title-font mb-4") {
                            (t!("index-speed.heading", cx))
                        }
                        p(class = "text-xl md:text-2xl 2xl:text-3xl p-2") {
                            span(
                                dangerously_set_inner_html = &t!("index-speed.desc-line-1", cx)
                            ) {}
                            br()
                            span(
                                dangerously_set_inner_html = &t!("index-speed.desc-line-2", cx)
                            ) {}
                            br()
                            span(
                                dangerously_set_inner_html = &t!("index-speed.desc-line-3", cx) // TODO Add footnote caveat to this
                            ) {}
                        }
                    },
                    code = Example::Simple(String::new()),
                    code_lang = String::new(),
                    custom_supplement = Some(view! { cx,
                        div(class = "bg-white dark:bg-[#272822] rounded-2xl !p-8 w-full flex flex-col lg:flex-row justify-center lg:justify-evenly") {
                            AnimatedCircularProgressBar(
                                percent = 100,
                                label = t!("index-speed.desktop-perf-label", cx)
                            )
                            // TODO Footnote this
                            AnimatedCircularProgressBar(
                                percent = 97,
                                label = t!("index-speed.mobile-perf-label", cx)
                            )
                            AnimatedCircularProgressBar(
                                percent = 100,
                                label = t!("index-speed.best-practices-label", cx)
                            )
                        }
                    }),
                    extra = None,
                    nav_buttons = NavButtons::Both("opts".to_string(), "cta".to_string())
                )
                // Final tile (different)
                IndexTile(
                    id = "cta".to_string(),
                    classes = "tile-end".to_string(),
                    order = TileOrder::TextLeft, // TODO Change this?
                    text_block = view! { cx,
                        p(class = "uppercase text-4xl font-semibold sm:font-normal xs:text-5xl sm:text-6xl 2xl:text-[5rem] p-2 title-font mb-4") {
                            (t!("index-cta.heading", cx))
                        }
                    },
                    code = examples.get_started,
                    code_lang = "sh".to_string(),
                    custom_supplement = None,
                    extra = Some(view! { cx,
                        div(class = "flex justify-center") {
                            ul(
                                class = "text-center max-w-4xl"
                            ) {
                                a(
                                    class = "bg-white dark:bg-black text-black dark:text-white sm:text-lg p-4 px-6 sm:px-8 mx-2 rounded-lg font-semibold uppercase mb-3 min-w-[10em] text-center inline-block hover:bg-gray-200 dark:hover:bg-neutral-800 transition-colors duration-200",
                                    href = link!("/docs", cx)
                                ) { (t!("index-cta.docs-button", cx)) }
                                a(
                                    class = "bg-white dark:bg-black text-black dark:text-white sm:text-lg p-4 px-6 sm:px-8 mx-2 rounded-lg font-semibold uppercase mb-3 min-w-[10em] text-center inline-block hover:bg-gray-200 dark:hover:bg-neutral-800 transition-colors duration-200",
                                    href = "https://github.com/framesurge/perseus"
                                ) { (t!("index-cta.gh-button", cx)) }
                                a(
                                    class = "bg-white dark:bg-black text-black dark:text-white sm:text-lg p-4 px-6 sm:px-8 mx-2 rounded-lg font-semibold uppercase mb-3 min-w-[10em] text-center inline-block hover:bg-gray-200 dark:hover:bg-neutral-800 transition-colors duration-200",
                                    href = "https://docs.rs/perseus/latest/perseus"
                                ) { (t!("index-cta.api-docs-button", cx)) }
                                a(
                                    class = "bg-white dark:bg-black text-black dark:text-white sm:text-lg p-4 px-6 sm:px-8 mx-2 rounded-lg font-semibold uppercase mb-3 min-w-[10em] text-center inline-block hover:bg-gray-200 dark:hover:bg-neutral-800 transition-colors duration-200",
                                    href = "https://crates.io/crates/perseus"
                                ) { (t!("index-cta.crates-io-button", cx)) }
                                // TODO Update this with the Matrix link when it's set up
                                a(
                                    class = "cursor-not-allowed bg-gray-300 dark:bg-neutral-700 text-black dark:text-white sm:text-lg p-4 px-6 sm:px-8 mx-2 rounded-lg font-semibold uppercase mb-3 min-w-[10em] text-center inline-block",
                                ) { (t!("index-cta.matrix-button", cx)) }
                                a(
                                    class = "bg-white dark:bg-black text-black dark:text-white sm:text-lg p-4 px-6 sm:px-8 mx-2 rounded-lg font-semibold uppercase mb-3 min-w-[10em] text-center inline-block hover:bg-gray-200 dark:hover:bg-neutral-800 transition-colors duration-200",
                                    href = "https://discord.com/invite/GNqWYWNTdp"
                                ) { (t!("index-cta.discord-button", cx)) }
                                a(
                                    class = "bg-white dark:bg-black text-black dark:text-white sm:text-lg p-4 px-6 sm:px-8 mx-2 rounded-lg font-semibold uppercase mb-3 min-w-[10em] text-center inline-block hover:bg-gray-200 dark:hover:bg-neutral-800 transition-colors duration-200",
                                    href = link!("/comparisons", cx)
                                ) { (t!("index-cta.comparisons-button", cx)) }
                            }
                        }
                    }),
                    nav_buttons = NavButtons::Top("speed".to_string())
                )

                // Because of how Perseus currently shifts everything, we need to re-highlight
                // And if the user starts on a page with nothing, they'll see no highlighting on any other pages, so we rerun every time the URL changes
                // This will be relative to the base URI
                script(src = ".perseus/static/prism.js", defer = true)
                script {
                    "window.Prism.highlightAll();"
                }
            }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { (t!("perseus", cx)) }
        link(rel = "stylesheet", href = ".perseus/static/prism.css")
        // Prefetch the images and fonts so the browser gets on these as quickly as possible
        link(rel = "prefetch", href = ".perseus/static/mesh_open.jpg")
        link(rel = "prefetch", href = ".perseus/static/mesh_close.jpg")
        link(rel = "prefetch", href = ".perseus/static/cy-grotesk-grand-regular.woff2")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .template(index_page)
        .head(head)
        .build_state_fn(get_build_state)
}

#[derive(Serialize, Deserialize, Clone)]
struct CodeExamples {
    app_in_a_file: Example,
    state_generation: Example,
    i18n: Example,
    cli: Example,
    get_started: Example,
}

/// A representation of a code example, which might have a short version.
#[derive(Serialize, Deserialize, Clone)]
enum Example {
    /// A normal example.
    Simple(String),
    /// An example with a short version and a long one.
    WithExcerpts { excerpts: String, full: String },
}

#[perseus::build_state]
async fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<CodeExamples> {
    // We know exactly where the examples we want are
    // TODO Join these futures separately
    let props = CodeExamples {
        app_in_a_file: get_example("../examples/website/app_in_a_file/src/main.rs").await?,
        state_generation: get_example("../examples/website/state_generation/src/main.rs").await?,
        i18n: get_example("../examples/website/i18n/src/main.rs").await?,
        cli: get_example("../examples/website/cli.txt").await?,
        get_started: get_example("../examples/website/get_started.txt").await?,
    };

    Ok(props)
}

#[cfg(not(target_arch = "wasm32"))]
async fn get_example(path: &str) -> Result<Example, std::io::Error> {
    use tokio::fs;

    let raw = fs::read_to_string(path).await?;
    // Get rid of anything after a snip comment
    let snipped_parts = raw.split("// SNIP").collect::<Vec<_>>();
    let full = snipped_parts[0].to_string();
    // Check if there are excerpts
    if full.contains("// EXCERPT_START") {
        // Excerpts can't overlap, and they must be fully delimited, so just using
        // matched indices works for this case
        let mut excerpts = Vec::new();
        let excerpt_starts = full.match_indices("// EXCERPT_START");
        let excerpt_ends = full.match_indices("// EXCERPT_END").collect::<Vec<_>>();
        for (i, (start_idx, _)) in excerpt_starts.enumerate() {
            let (end_idx, _) = excerpt_ends[i];
            let excerpt = &full[(start_idx + 16)..end_idx];
            excerpts.push(excerpt.trim());
        }
        let excerpts_str = excerpts.join("\n\n// ...\n\n");

        // Remove the excerpt delimiters from the full version for cleanliness
        let full = full
            .replace("// EXCERPT_START\n", "")
            .replace("// EXCERPT_END\n", "");

        Ok(Example::WithExcerpts {
            excerpts: excerpts_str,
            full,
        })
    } else {
        Ok(Example::Simple(full))
    }
}
