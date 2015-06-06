
use poll::{InputPoller};
use ncurses::*;

use ::backend::adapter::BackendAdapter;
use std::io;


pub struct CursesEngine;

impl CursesEngine {
    pub fn new(backend_adapter : &BackendAdapter) -> CursesEngine {
        initscr();
        cbreak();
        noecho();
        CursesEngine
    }

    pub fn run(&self) {
        use std::sync::mpsc::channel;
        //let reader = SimpleCurserReader::new(); //TODO: Update reader with GNU readline reader

        let stdin_poller = InputPoller::new(0);
        let mut i = 0;
        loop {
            mvprintw(1, 0, format!("{}", i).as_ref());
            i+=1;
            let mut buf = String::new();
            let ref rcv = stdin_poller.rcv;
            select!(
                _ = rcv.recv() => {
                    mvprintw(0, 0, "Received input: ");
                    let c = getch();
                    addch(c as chtype);
                }
            );
        }
    }
}
