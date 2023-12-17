use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <CookBook/>
    }
}

#[component]
fn CookBook() -> impl IntoView {
    let (opened, set_opened) = create_signal(false);
    view! {
        <div class="CookBook"
        class:opened=move || opened()
        on:click=move |_| {
        set_opened.update(|o| *o = !*o);
        }> {move || if opened() {""} else {"COOKBOOK"}}</div>
    }
}
