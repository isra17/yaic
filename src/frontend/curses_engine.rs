use poll::{InputPoller};
use ncurses::*;

use ::backend::adapter::BackendAdapter;
use std::sync::mpsc::{channel,Receiver};

use ::frontend::rl_reader::RlReader;

pub struct CursesEngine {
    cmd_rx: Receiver<String>,
    reader : RlReader,
}

impl CursesEngine {
    pub fn new(backend_adapter : &BackendAdapter) -> CursesEngine {
        initscr();
        raw();
        noecho();

        let (cmd_tx, cmd_rx) = channel::<String>();
        CursesEngine {
            cmd_rx: cmd_rx,
            reader: RlReader::new("", cmd_tx)
        }
    }

    pub fn run(&self) {
        let stdin_poller = InputPoller::new(0);
        let cmd_rx = &self.cmd_rx;
        loop {
            self.update_prompt();
            let ref rcv = stdin_poller.rcv;
            select!(
                cmd = cmd_rx.recv() => self.handle_cmd(cmd.unwrap()),
                _ = rcv.recv() => self.handle_stdin()
            );
        }
    }

    fn handle_cmd(&self, cmd : String) {
        mvprintw(3, 0, format!("Last Command: {}", cmd).as_ref());
        clrtoeol();
        self.refresh();

        if cmd == "quit" {
            use std::process::exit;
            exit(0);
        }
    }

    fn handle_stdin(&self) {
        self.reader.process_input();
    }

    fn update_prompt(&self) {
        let content = self.reader.content();
        mvprintw(0, 0, format!("{} ", content).as_ref());
        mv(0, content.len() as i32);
        clrtoeol();
        self.refresh();
    }

    fn refresh(&self) {
        mv(0, self.reader.cursor_offset());
        refresh();
    }
}
