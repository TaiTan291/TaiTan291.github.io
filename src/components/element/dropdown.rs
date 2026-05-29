use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::*;
//use leptos::children::Children;

#[component]
pub fn Dropdown(#[prop(into)] title: String, children: ChildrenFn) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <div class="relative inline-block text-left">
            <button
                on:click=move |_| set_is_open.update(|open| *open = !*open)
                class="px-4 py-2 text-2x font-boid text-cyber-accent rounded-md transition"
            >
                {title}
            </button>

            <Show when=move || is_open.get()>
                <div class="px-4 indent-1">{children()}</div>
            </Show>
        </div>
    }
}
