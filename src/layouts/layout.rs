use crate::components::element::footer::Footer;
use crate::components::element::header::Header;
use leptos::children::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Signal;
use leptos::*;

#[component]
pub fn Layout(#[prop(into)] status: Signal<i8>,children: Children) -> impl IntoView {
    view! {
        <div class="container mx-auto my-5 max-w-4xl bg-cyber-layout rounded-xl border-solid overflow-hidden shadow-sm">
            <Header status={status} />
            <main>
                {children()}
            </main>
            <Footer />
        </div>
    }
}
