use crate::pages::favorite::Favorite;
use crate::pages::favorite::Gateway;
use crate::pages::index::IndexPage;
use crate::pages::not_found::NotFoundPage;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

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
