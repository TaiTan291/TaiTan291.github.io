use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <nav class="p-4 flex justify-between items-center  top-0 bg-cyber-surface text-cyber-text backdrop-blur-md z-50">
                <nav class="space-x-4 text-cyber-text">
                    <a href="#about" class="hover:text-cyber-accent">About</a>
                    <a href="#skills" class="hover:text-cyber-accent">Skills</a>
                    <a href="#works" class="hover:text-cyber-accent">Works</a>
                </nav>
            </nav>
        </header>
    }
}
