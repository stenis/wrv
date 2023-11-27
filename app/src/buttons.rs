
use gloo_timers::future::TimeoutFuture;
use leptos::{*, html::*};

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10c
async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    println!("got here.");
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
pub fn CounterB() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button class="btn-primary"
            on:click=move |_| { set_count.update(|n| *n += 1); }
        >
            {count}
        </button>
    }
}

#[component]
pub fn Counter() -> impl IntoView {
   // this count is our synchronous, local state
    let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_local_resource(
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_data(value).await },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_local_resource(|| (), |_| async move { load_data(1).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Fetching...".into())
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading() { " (...)" } else { " (Idle)" };

    view! {
        <button class="btn-primary"
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            disabled=loading
        >
            "Click me"
        </button>
        <p class="p-2 m-1">
            <code>"stable"</code>": " {stable}
        </p>
        <p class="p-2 m-1">
            <code>"count"</code>": " {count}
        </p>
        <p class="m-3">
            <code>"async_value"</code>": "
            {async_result}
            {is_loading}
        </p>
    }

    //   div()
    //     .child((
    //         button()
    //          .attr("disabled", loading )
    //          .attr("class", "btn-primary")
    //             // typed events found in leptos::ev
    //             // 1) prevent typos in event names
    //             // 2) allow for correct type inference in callbacks
    //             .on(ev::click, move |_| set_count.update(|n| *n += 1))
    //             .child("click"),
    //         p()
    //           .attr("class", "p-2 m-1").child(("Value: ", count)),
    //     ))
}