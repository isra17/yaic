// Not knowing much Rust + bending it to fit some C interface = this
// This is a GNU Readline Reader that keep feeding Readline from stdin
// and handle the readline callback to send it through a channel.
extern crate readline;

use self::readline::*;
use std::ffi::CStr;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use ncurses::*;

pub struct RlReader;

lazy_static! {
    static ref CMD_TX : Mutex<Option<Sender<String>>> = Mutex::new(None);
}

extern "C" fn handle_complete_line(text: *mut i8) {
    let ref rcv_tx = *CMD_TX.lock().unwrap();
    match rcv_tx {
        &Some(ref tx) => {
            let cmd = unsafe { String::from_utf8_lossy(CStr::from_ptr(text).to_bytes()).into_owned() };
            add_history(cmd.as_str());
            tx.send(cmd).unwrap();
        },
        &None => (),
    };
}

impl RlReader {
    pub fn new(prompt: &str, cmd_tx: Sender<String>) -> RlReader {
        using_history();
        mvprintw(4,0,"Create RlReader");
        *CMD_TX.lock().unwrap() = Some(cmd_tx);
        let reader = RlReader;
        rl_callback_handler_install(prompt, Some(handle_complete_line));
        reader
    }

    pub fn process_input(&self) {
        rl_callback_read_char()
    }

    pub fn content(&self) -> String {
        unsafe{ String::from_utf8_lossy(CStr::from_ptr(rl_line_buffer()).to_bytes()).into_owned() }
    }

    pub fn cursor_offset(&self) -> i32 {
        rl_point()
    }
}

impl Drop for RlReader {
    fn drop(&mut self) {
        rl_callback_handler_remove();
        *CMD_TX.lock().unwrap() = None;
    }
}
