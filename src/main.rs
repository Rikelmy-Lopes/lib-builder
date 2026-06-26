#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::components::app::App;

mod components;
mod config;
mod fs;
mod utils;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Lib Builder")
        .window(App::settings())
        .theme(App::theme)
        .run()
}
