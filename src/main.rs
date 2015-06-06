#![feature(std_misc)]

extern crate poll;
extern crate ncurses;

mod frontend;
mod backend;

use ncurses::*;
use backend::adapter::BackendAdapter;
use frontend::curses_engine::CursesEngine;
use std::thread;

fn main() {
    let backend_adapter = BackendAdapter::new();
    let ncurses_frontend = CursesEngine::new(&backend_adapter);
    let ft = thread::spawn(move || { ncurses_frontend.run(); });
    backend_adapter.run();
    ft.join();
}
