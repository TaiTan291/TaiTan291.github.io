use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-auto h-6 bg-cyber-surface text-center text-sm text-cyber-text">
            <p>{ "© 2026 TaiTan291 All Rights Reserved." }</p>
        </footer>
    }
}
