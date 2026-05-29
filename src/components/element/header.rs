use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Signal;
use leptos::prelude::Get;
use leptos::*;
use leptos_router::components::A;

#[component]
pub fn Header(#[prop(into)] status: Signal<i8>) -> impl IntoView {
    let current_view = move || {
        match status.get() {
            1 => vec![
                view! { <A href="#about">"About"</A> }.into_view(),
                view! { <A href="#skills">"Skills"</A> }.into_view(),
                view! { <A href="#works">"Works"</A> }.into_view(),
            ],

            2 => vec![
                view! { <A href="#about">"About"</A> }.into_view(),
                view! { <A href="#hobby">"My hobby"</A> }.into_view(),
                view! { <A href="#vtubr">"Vtuber"</A> }.into_view(),
                view! { <A href="#link">"Link"</A> }.into_view(),
            ],

            _ => vec![],
        }
    };

    view! {
        <header>
            <nav class="p-4 flex justify-between items-center top-0 bg-cyber-surface text-cyber-layouttext backdrop-blur-md z-50">
                <A href="/">"TaiTan's Portfolio"</A>

                <nav class="space-x-4 text-cyber-layouttext">
                    {current_view()}
                </nav>
            </nav>
        </header>
    }
}
