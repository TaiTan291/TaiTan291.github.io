use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::*;
use leptos_router::components::A;

#[component]
pub fn Toc(#[prop(into)] title: String, #[prop(into)] tag: String) -> impl IntoView {
    view! {
        <div class="border-3 rounded-3xl p-4 m-4 max-w-xs w-auto h-auto bg-white text-center">
            <A href=tag>
                <h2 class="font-bold p-18">
                    {title}
                </h2>
            </A>
        </div>
    }
}

#[component]
pub fn LinkToc(#[prop(into)] title: String, #[prop(into)] url: String, #[prop(into)] image_url: String) -> impl IntoView {
    view! {
        <a 
            href=url
            class="border-3 rounded-3xl p-4 m-4 max-w-xs w-auto h-auto bg-white flex text-center items-center"
        >
                <img
                    src={image_url.clone()}
                    alt="Logo"
                    class="w-20 h-20 m-1 object-contain"
                />

            <h2 class="px-4 font-bold p-18">
                {title}
            </h2>
        </a>
    }
}
#[component]
pub fn GorioshiLinkToc(#[prop(into)] title: String, #[prop(into)] url: String, #[prop(into)] image_url: String) -> impl IntoView {
    view! {
        <a 
            href=url
            class="border-3 rounded-3xl p-4 m-4 max-w-xs w-auto h-auto bg-black flex text-center items-center"
        >
                <img
                    src={image_url.clone()}
                    alt="Logo"
                    class="w-20 h-20 m-1 object-contain"
                />

            <h2 class="px-4 font-bold p-18 text-white">
                {title}
            </h2>
        </a>
    }
}
