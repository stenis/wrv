#![feature(try_blocks)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;
pub mod todo;
pub mod buttons;

pub mod core;
pub mod components;

//use buttons::Counter;
use components::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/wrv.css"/>
        <Title text="WRV USB RADIO" />
        <Html lang="en" />
        <main class="bg-slate-400 my-0 mx-auto text-center">
            <Router>     
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </Router>
        </main>
    }
}
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    //let (value, set_value) = create_signal(0);
    let handle = window_event_listener(ev::keydown, core::utility::keyboard_scroll);
    on_cleanup(move || handle.remove());

    view! {
        <div class="text-black font-sans flex flex-col min-h-screen">
            <Slide heading="Modern alternative web application " hide_previous=true>
                <h2 class="my-2 font-thin text-3xl text-sky-200/90">Presentation / Agenda</h2>
                <ul>
                    <li>Rust, Radio, web standards</li>
                    <li>DX using Leptos, dev loop (compile -> run)</li>
                </ul>
                <ConnectUSB text="connect usb" />
                <WCanvas />
                // <code
                //     class="text-sm sm:text-base inline-flex text-left items-center space-x-4 bg-gray-800 text-white rounded-lg p-4 pl-6">
                //     <span class="flex gap-4">
                //         <span class="shrink-0 text-gray-500">
                //             $
                //         </span>

                //         <span class="flex-1">
                //             <span>
                //                 composer require
                //             </span>

                //             <span class="text-yellow-500">
                //                 laravel/dusk
                //             </span>
                //         </span>
                //     </span>

                //     <svg class="shrink-0 h-5 w-5 transition text-gray-500 group-hover:text-white" xmlns="http://www.w3.org/2000/svg"
                //         viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                //         <path d="M8 2a1 1 0 000 2h2a1 1 0 100-2H8z"></path>
                //         <path
                //             d="M3 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v6h-4.586l1.293-1.293a1 1 0 00-1.414-1.414l-3 3a1 1 0 000 1.414l3 3a1 1 0 001.414-1.414L10.414 13H15v3a2 2 0 01-2 2H5a2 2 0 01-2-2V5zM15 11h2a1 1 0 110 2h-2v-2z">
                //         </path>
                //     </svg>
                // </code>
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
            </Slide>
            <Slide heading="">
                <p class="text-left m-3">
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent maximus felis eget quam fermentum euismod. Proin sagittis quam sodales tortor molestie imperdiet. Praesent ipsum erat, egestas pulvinar dui sed, ornare eleifend nisi. Curabitur sit amet accumsan mauris. Mauris finibus fermentum commodo. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Pellentesque cursus tortor ac sagittis accumsan. Fusce congue commodo commodo. Nam consectetur varius diam at vestibulum. Nam nec laoreet elit. Pellentesque quis dui quis quam maximus maximus. Duis ornare augue nunc, eget varius mauris pellentesque id. In ut tortor diam.
                </p>
                <p class="text-left m-3">
                    Cras ac quam ac elit maximus euismod. Nunc dignissim quam id lobortis rutrum. Etiam non laoreet enim. In maximus turpis non ante gravida, eget varius mauris ornare. Etiam vitae lorem id ligula mollis tempor vel a ex. Fusce mi mauris, vulputate nec nulla nec, ultricies fermentum neque. Pellentesque vulputate sed nunc et ullamcorper. Nullam vitae laoreet est, eget congue enim. Pellentesque velit mauris, tristique a rutrum vel, interdum et purus.
                </p>
            </Slide>
            <Slide heading="HEADING #2" hide_next=true>
                <p>bottom</p>
            </Slide>
        </div>
    }
}
