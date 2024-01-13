use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;
pub mod todo;
pub mod buttons;
pub mod wcanvas;

use buttons::Counter;
use wcanvas::WCanvas;

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
        <Title text="HEY!" />

        // content for this welcome page
        <Router>
            <body>
                <main class="my-0 mx-auto max-w-3xl text-center">
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
    let (value, set_value) = create_signal(0);

    view! {
        <div class="text-black font-sans flex flex-col min-h-screen">
            <h1 class="my-2 font-thin text-3xl text-sky-200/90">SERVERFUNC</h1>
            <div id="draw-area" class="w-96 h-96 mx-auto bg-gray-500/30">
                <WCanvas />
            </div>
            <div class="flex flex-wrap m-auto">
                <button title="+" on:click=move |_| set_value.update(|value| *value -= 1) 
                    class="rounded-lg px-1py-1 px-2 m-1 text-gray-100 bg-gray-400/70">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" data-slot="icon" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
                    </svg>
                </button>
                <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                    {value}
                </button>
                <button on:click=move |_| set_value.update(|value| *value += 1) 
                    class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                    "+"
                </button>
            </div>
            <div class="flex flex-wrap m-auto">
                <Counter />
            </div>
        </div>
    }
}
