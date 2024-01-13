use crate::i18n::*;
use leptos::*;

leptos_i18n::load_locales!();

fn main() {
    provide_i18n_context();
    mount_to_body(App)
}

#[component]
pub fn App() -> impl IntoView {
    let i18n = use_i18n();

    view! { <p>{t!(i18n, init)}</p> }
}
