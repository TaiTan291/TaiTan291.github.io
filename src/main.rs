pub mod app;
pub mod components;
pub mod layouts;
pub mod pages;

use leptos::*;
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes, A};
use leptos_router::*;

use crate::app::App;
use crate::pages::index::IndexPage;
//use crate::pages::favorite::favorite_page;

pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
