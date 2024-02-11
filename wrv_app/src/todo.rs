use leptos::*;

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    println!("{title}");
    Ok(())
}

#[component]
pub fn BusyButton() -> impl IntoView {
    let (is_loading, set_is_loading) = create_signal(false);
    view! {
        <button class="btn-primary" on:click=move |_| {
            spawn_local(async move {
                let text = "So much to do!".to_string();
                _ = add_todo(text).await;

                set_is_loading(true);
            });
        }>
            {is_loading}
        </button>
    }
}