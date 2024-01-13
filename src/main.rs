use leptos::*;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! { <p>"AsqArs’ Website"</p> }
}
