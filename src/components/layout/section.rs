use leptos::children::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
pub fn Section(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <section class="m-7 p-5 w-auto h-auto bg-cyber-container text-cyber-text rounded-xl shadow-lg">
            <h1 class="text-2xl font-bold mb-4 underline decoration-2 underline-offset-4 text-cyber-accent">{title}</h1>
            <div>{children()}</div>
        </section>
    }
}
