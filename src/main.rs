use std::io;
use std::io::prelude::*;

use letter_c::make_qmk_note_sequence_from_letter_notes;

fn main() -> io::Result<()> {
    let mut stdin_buf = String::new();
    io::stdin().read_to_string(&mut stdin_buf)?;
    let song_macro = make_qmk_note_sequence_from_letter_notes(stdin_buf.chars());

    println!("{}", song_macro);

    io::Result::Ok(())
}
