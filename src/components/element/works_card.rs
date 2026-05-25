use leptos::control_flow::Show;
use leptos::prelude::create_signal;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::OnAttribute;
use leptos::prelude::Set;
use leptos::*;
use pulldown_cmark::{html, Options, Parser};

#[component]
pub fn WorksCard(
    #[prop(into)] title: String,
    #[prop(into)] image_url: String,
    #[prop(into)] stack: Vec<String>,
    #[prop(into)] md_path: String,
) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);

    // md -> html
    let markdown_input = include_str!(md_path);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    view! {
        <div
            class="min-w-50 h-40 m-3 flex flex-1 flex-col items-center justify-start bg-cyber-block rounded-md overflow-hidden shadow-sm cursor-pointer  transition-all"
            on:click=move |_| set_show_modal.set(true)
        >
            <img
                src={image_url.clone()}
                alt="head image"
                class="w-24 h-24 mt-2"
            />
            <h3 class="text-4xl">{title}</h3>
        </div>
        <Show when=move || show_modal.get()>
            <img
                src={image_url.clone()}
                alt="head image"
                class="w-24 h-24 mt-2"
            />
            <h1 class="text-4xl">{title}</h1>

        </Show>
    }
}
