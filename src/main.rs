use std::io::{stdout, stdin, Write};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    event::Key,
    input::TermRead,
    *,
};

mod gamedata;

// To initialize any necessary data before starting the game
fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    // #[cfg(not(debug_assertions))]
    write!(stdout, "{}", cursor::Hide).unwrap(); // Hide the cursor before starting the game
    main_menu(&mut stdout);
    write!(stdout, "{}{}", clear::All, cursor::Show).unwrap(); // Clear game output and re-show cursor after game finishes
}

// menu options: their indexes are used to determine which action to perform later on
static MAIN_MENU_ITEMS: &[&'static str] = &[ "New Game", "Load Game", "Configure", "Quit" ];

// Handles all of the main menu's interaction
fn main_menu<T: Write>(t: &mut RawTerminal<T>) {
    let mut selection_index = 0usize;

    'mainloop: loop {
        let (w, h) = terminal_size().unwrap();

        // title of game
        // then just list different options out
        // selection is a box-drawing around the option, arrow keys change selection
        // bottom of screen has version and copyright info
        // updates: on input and terminal resize

        let cx = w / 2;
        // let cy = h / 2;

        write!(t, "{}", clear::All).unwrap();

        for (i, name) in MAIN_MENU_ITEMS.iter().enumerate() {
            if selection_index == i { // code for drawing the currently selected item
                let len: u16 = name.len() as u16 + 4; // horizontal length of entry
                write!(
                    t,
                    "{}{}{}{}{}  {}  {}{}{}{}",
                    cursor::Goto(cx - (len/2), 3 * i as u16 + 1), "┏", "━".repeat(len as usize - 2), "┓",
                    cursor::Goto(cx - (len/2), 3 * i as u16 + 2), name,
                    cursor::Goto(cx - (len/2), 3 * i as u16 + 3), "┗", "━".repeat(len as usize - 2), "┛",
                ).unwrap();
            } else { // code for drawing all the other de-selected items
                write!(
                    t,
                    "{}{}",
                    cursor::Goto(cx - (name.len() / 2) as u16, 3 * i as u16 + 2), name
                ).unwrap();
            }
            // Note: 3 * i so we have 3 lines per menu item to do our box drawing around it, + 2 for not counting from zero and the top line of space
        }

        // Version and copyright info at bottom left and bottom right
        let copyright = "MIT licensed software by Luke I. Wilson ";
        write!(t, "{} v{}{}{}", cursor::Goto(1, h), "0.3.0", " ".repeat(w as usize-(2+"0.3.0".len())-copyright.len()), copyright).unwrap();
        // write!(t, "{}v{}", cursor::Goto(2, h), "0.3.0").unwrap();
        // write!(t, "{}{}", cursor::Goto(w - copyright.len() as u16, h), copyright).unwrap();

        t.flush().unwrap(); // refresh terminal

        // handle input events
        for c in stdin().events() {
            let evt = c.unwrap();
            match evt {
                event::Event::Key(k) => match k {
                    Key::Down => if selection_index >= MAIN_MENU_ITEMS.len()-1 { // custom wrapping addition
                        selection_index = 0;
                    } else {
                        selection_index += 1;
                    },
                    Key::Up => if selection_index <= 0 { // custom wrapping subtraction
                        selection_index = MAIN_MENU_ITEMS.len()-1;
                    } else {
                        selection_index -= 1;
                    },
                    Key::Char('\n') => { // on return key pressed
                        match selection_index {
                            0 => {}, // new game
                            1 => SaveGameFinder { finding: true, string: String::new(), result_handler: &|_| {} }.run(t), // load game
                            2 => {}, // configure settings
                            3 => return, // quit game
                            _ => unimplemented!(),
                        }
                    }
                    _ => {},
                }
                _ => {}
            }
            continue 'mainloop; // Re-render on input received
        }
    }
}

struct SaveGameFinder<'a> {
    finding: bool, // if this instance is for finding an already existing save game, or making a new one
    // dir: Option<std::path::PathBuf>, // the final, verified directory path
    string: String, // the temporary, "String" -version of the directory, for typing purposes
    result_handler: &'a dyn Fn(std::path::PathBuf), // what do we do when we have the valid save path?
}

impl<'a> SaveGameFinder<'a> {
    const MIN_WIDTH: u16 = 45;
    const HEIGHT: u16 = 9;

    // The SaveGameFinder will take control of the current thread, until it is either cancelled,
    // or has successfully found a valid save data directory.
    pub fn run<T: Write>(&mut self, t: &mut RawTerminal<T>) {
        let title = if self.finding {
            "Input Existing Save Directory"
        } else {
            "Create New Save Directory"
        };
        let mut invalid = false;

        'mainloop: loop {
            let (w, h) = terminal_size().unwrap();

            // Lots of tedious code for drawing the dialog
            let tl = (w/2 - (Self::MIN_WIDTH/2), h/2 - (Self::HEIGHT/2)); // Character cell of top left of dialog

            write!(t, "{}╔{}╗", cursor::Goto(tl.0, tl.1), "═".repeat(Self::MIN_WIDTH as usize - 2)).unwrap(); // line 1
            write!(t, "{}║{}{}{}{}{}║", cursor::Goto(tl.0, tl.1 + 1),
                cursor::Goto(w/2 - (title.len()/2) as u16, tl.1 + 1), style::Bold, title, style::Reset,
                cursor::Goto(w/2 + (Self::MIN_WIDTH/2), tl.1 + 1),
            ).unwrap(); // line 2
            write!(t, "{}╠{}╣", cursor::Goto(tl.0, tl.1 + 2), "═".repeat(Self::MIN_WIDTH as usize - 2)).unwrap(); // line 3
            write!(t, "{}║{}║", cursor::Goto(tl.0, tl.1 + 3), " ".repeat(Self::MIN_WIDTH as usize - 2)).unwrap(); // line 4
            write!(t, "{}║ {}{}{} ║", cursor::Goto(tl.0, tl.1 + 4), color::Bg(color::White), " ".repeat(Self::MIN_WIDTH as usize - 4), color::Bg(color::Reset)).unwrap(); // line 5 (input)
            write!(t, "{}║{}║", cursor::Goto(tl.0, tl.1 + 5), " ".repeat(Self::MIN_WIDTH as usize - 2)).unwrap(); // line 6
            write!(t, "{}╟{}╢", cursor::Goto(tl.0, tl.1 + 6), "─".repeat(Self::MIN_WIDTH as usize - 2)).unwrap(); // line 7
            let accept_msg = "Confirm: RETURN";
            write!(t, "{}║ Cancel: ESC{}{} ║",
                cursor::Goto(tl.0, tl.1 + 7),
                cursor::Goto(w/2 + (Self::MIN_WIDTH/2) - (1 + accept_msg.len()) as u16, tl.1 + 7),
                accept_msg,
            ).unwrap(); // line 8
            write!(t, "{}╚{}╝", cursor::Goto(tl.0, tl.1 + 8), "═".repeat(Self::MIN_WIDTH as usize - 2)).unwrap(); // line 9

            // Update input box
            write!(t, "{}{}{}{}{}{}{}{}{}", cursor::Goto(tl.0 + 2, tl.1 + 4), cursor::Show,
                if invalid { color::Fg(color::Red).to_string() } else { color::Fg(color::Black).to_string() },
                color::Bg(color::White), self.string,
                " ".repeat(Self::MIN_WIDTH as usize - 4 - self.string.len()), color::Bg(color::Reset), color::Fg(color::Reset),
                cursor::Goto(tl.0 + 2 + self.string.len() as u16, tl.1 + 4),
            ).unwrap();

            t.flush().unwrap(); // refresh terminal

            // handle input events
            for c in stdin().keys() {
                match c.unwrap() {
                    Key::Esc => {
                        break 'mainloop; // Jump out of thread control and return to caller
                    }
                    Key::Char('\n') => {
                        if !invalid { // Creating a directory (it must not exist) (invalid is not set if self.finding)
                            let p = std::path::PathBuf::from(self.string.clone());
                            (self.result_handler)(p);
                            break 'mainloop;
                        }
                    }
                    Key::Char(c) => {
                        self.string.push(c);

                        if !self.finding {
                            let p = std::path::PathBuf::from(self.string.clone());
                            invalid = p.exists();
                        }
                    }
                    Key::Backspace => {
                        let _ = self.string.pop();
                    }
                    _ => {}
                }
                continue 'mainloop; // Re-render on input received
            }
        }
    }
}

// TODO: some architecture for Overlay that allows performing painter's method on multiple drawing objects easily
