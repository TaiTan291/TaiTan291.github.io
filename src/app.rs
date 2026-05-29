use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use crate::pages::index::IndexPage;
use crate::pages::favorite::Favorite;
use crate::pages::not_found::NotFoundPage;
use crate::pages::favorite::Gateway;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| view! { <NotFoundPage/> }>
                    <Route path=path!("/") view=IndexPage/>
                    <Route path=path!("/hobby") view=Favorite/>
                    <Route path=path!("/404") view=NotFoundPage/>
                    <Route path=path!("/114514") view=Gateway/>
                </Routes>
            </main>
        </Router>
    }
}
