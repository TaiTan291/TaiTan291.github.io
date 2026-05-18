use crate::components::footer::Footer;
use crate::components::header::Header;
use leptos::children::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
pub fn MainLayouts(children: Children) -> impl IntoView {
    view! {
        <div class="container mx-auto my-5 max-w-4xl bg-cyber-layout rounded-xl border-solid overflow-hidden shadow-sm">
            <Header />
            <main>
                {children()}
            </main>
            <Footer />
        </div>
    }
}
