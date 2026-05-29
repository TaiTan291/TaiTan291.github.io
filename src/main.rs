pub mod components;
pub mod layouts;
pub mod app;
pub mod pages;

use leptos::*;
use leptos_router::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A, Outlet},
};

use crate::app::App;
use crate::pages::index::IndexPage;
//use crate::pages::favorite::favorite_page;

pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App); 
}
