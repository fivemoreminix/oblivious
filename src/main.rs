use std::io::{stdout, stdin, Write};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    event::Key,
    input::TermRead,
    *,
};

// To initialize any necessary data before starting the game
fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
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
        let copyright = "MIT licensed software by Luke I. Wilson";
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
                            1 => {}, // load game
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
