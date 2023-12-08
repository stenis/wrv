use leptos::*;
use leptos_meta::*;
use leptos_router::*;
pub mod error_template;
pub mod todo;
pub mod buttons;

use buttons::Counter;

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
        // <script type="module" inner_html=code></script>
        <Script type_="module">
            "import init, { greet, start } from \"./gfx.js\";
            init().then(async () => { 
                console.log(\"WASM Loaded\");
                console.log(greet(\"test\"));
                await start();
            })"
        </Script>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        // sets the document title
        <Title text="Welcome to Leptos"/>

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
        <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
            <h1>serverfunc.</h1>
            <div id="draw-area" class="w-96 h-96 mx-auto"></div>
            <div class="flex flex-wrap m-auto">
                <button on:click=move |_| set_value.update(|value| *value -= 1) 
                    class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                    "-"
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
                // <CounterB />
                <Counter />
            </div>
        </div>
    }
}
