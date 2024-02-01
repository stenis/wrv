use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::use_window;
use logging::log;
use wasm_bindgen::{prelude::*, JsCast};

pub mod error_template;
pub mod todo;
pub mod buttons;
pub mod wcanvas;
pub mod components;
pub mod utility;

use buttons::Counter;
use wcanvas::WCanvas;
use web_sys::{ScrollBehavior, ScrollToOptions};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        // <Stylesheet id="leptos" href="/pico.min.css"/>
        // <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>
        <Stylesheet id="leptos" href="/pkg/serverfunc.css"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        // sets the document title
        <Title text="USB RADIO" />

        // content for this welcome page
        <Router>
            <body>
                <main class="bg-slate-400 my-0 mx-auto text-center">
                    <Routes>
                        <Route path="" view=HomePage/>
                    </Routes>
                </main>
            </body>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    //let (value, set_value) = create_signal(0);
    let handle = window_event_listener(ev::keydown, |ev| {
        let code = ev.code();
        let h : f64 = window().inner_height().unwrap().as_f64().unwrap();
        let mut ops = ScrollToOptions::new();
        ops.behavior(ScrollBehavior::Smooth);

        match ev.key_code() {
            37 => {
                ops.top(-h);
                window().scroll_by_with_scroll_to_options(&ops);
            }, // left
            39 => {
                ops.top(h);
                window().scroll_by_with_scroll_to_options(&ops);
            }, // right
            _ => (),
        }
    });
    on_cleanup(move || handle.remove());

    view! {
        <div class="text-black font-sans flex flex-col min-h-screen">
            <components::Page heading="USB RADIO" hide_previous=true>
                <WCanvas />
            // <div class="flex flex-wrap m-auto">
            //     <button title="+" on:click=move |_| set_value.update(|value| *value -= 1) 
            //         class="rounded-lg px-1py-1 px-2 m-1 text-gray-100 bg-gray-400/70">
            //         <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" data-slot="icon" class="w-6 h-6">
            //             <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
            //         </svg>
            //     </button>
            //     <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
            //         {value}
            //     </button>
            //     <button on:click=move |_| set_value.update(|value| *value += 1) 
            //         class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
            //         "+"
            //     </button>
            // </div>
            // <div class="flex flex-wrap m-auto">
            //     <Counter />
            // </div>
        </components::Page>
        <components::Page heading="HEADING">
            <p class="text-left m-3">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent maximus felis eget quam fermentum euismod. Proin sagittis quam sodales tortor molestie imperdiet. Praesent ipsum erat, egestas pulvinar dui sed, ornare eleifend nisi. Curabitur sit amet accumsan mauris. Mauris finibus fermentum commodo. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Pellentesque cursus tortor ac sagittis accumsan. Fusce congue commodo commodo. Nam consectetur varius diam at vestibulum. Nam nec laoreet elit. Pellentesque quis dui quis quam maximus maximus. Duis ornare augue nunc, eget varius mauris pellentesque id. In ut tortor diam.
            </p>
            <p class="text-left m-3">
                Cras ac quam ac elit maximus euismod. Nunc dignissim quam id lobortis rutrum. Etiam non laoreet enim. In maximus turpis non ante gravida, eget varius mauris ornare. Etiam vitae lorem id ligula mollis tempor vel a ex. Fusce mi mauris, vulputate nec nulla nec, ultricies fermentum neque. Pellentesque vulputate sed nunc et ullamcorper. Nullam vitae laoreet est, eget congue enim. Pellentesque velit mauris, tristique a rutrum vel, interdum et purus.
            </p>
        </components::Page>
        <components::Page heading="HEADING #2" hide_next=true>
            <p>bottom</p>
        </components::Page>
    </div>
    }
}
