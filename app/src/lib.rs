use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;
pub mod todo;

use todo::BusyButton;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        // <Stylesheet id="leptos" href="/pico.min.css"/>
        // <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <header class="container">
                <hgroup>
                    <h1>App</h1>
                </hgroup>
            </header>
            <main class="container">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <div class="container">
            <button role="button" class="outline" on:click=on_click>"Click Me: " {count}</button>
            <a href="#" role="button" class="outline" on:click=on_click>"Click Me: " {count}</a>
            <BusyButton />
        </div>
    }
}
