use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keyprocess() {
                die(error);
            }
        // for key in io::stdin().keys() {
        //     match key {
        //         Ok(key) => match key {
        //             Key::Char(c) => {
        //                 if c.is_control() {
        //                     println!("{:?}\r", c as u8);
        //                 } else {
        //                     println!("{:?} ({})\r", c as u8, c);
        //                 }
        //             }
        //             Key::Ctrl('d') => break,
        //             _ => println!("{:?}\r",key),
        //         },
        //         Err(err) => die(err),
        }
    }
    
    pub fn default() -> Self {
        Self{should_quit: false,}
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("\x1b[2J");
        io::stdout().flush()
    }

    fn process_keyprocess(&mut self) -> Result<(), std::io::Error> {
        let process_key = read_key()?;
        match process_key {
            Key::Ctrl('d') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}