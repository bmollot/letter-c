const megalovania_raw: &'static str = "5|--d---------------d-------|
4|dd--a--G-g-f-dfgcc--a--G-g|

5|--------d---------------d-|
4|-f-dfg----a--G-g-f-dfg----|
3|------bb--------------AA--|

5|--------------d-----------|
4|a--G-g-f-dfgdd--a--G-g-f-d|

5|----d---------------d-----|
4|fgcc--a--G-g-f-dfg----a--G|
3|------------------bb------|

5|----------d---------------|
4|-g-f-dfg----a--G-g-f-dfgdd|
3|--------AA----------------|

5|d---------------d---------|
4|--a--G-g-f-dfgcc--a--G-g-f|

5|------d---------------d---|
4|-dfg----a--G-g-f-dfg----a-|
3|----bb--------------AA----|

5|------------d-------------|
4|-G-g-f-dfgdd--a--G-g-f-dfg|

5|--d---------------d-------|
4|cc--a--G-g-f-dfg----a--G-g|
3|----------------bb--------|

5|--------d-----------------|
4|-f-dfg----a--G-g-f-dfgf-ff|
3|------AA------------------|

4|-f-f-d-d--d-ffff-g-G-gfdfg|

5|-------------c----d-d-d-dc|
4|--f-ff-g-G-a---a-------a--|

4|--------a-aa-a-a-g-g----a-|

5|---------d----d-------c---|
4|aa-a-g-a---ag---a-g-f---a-|

5|-----------c--------------|
4|g-f-d-ef-a----------------|

4|--fdfgGgfdGgfdfg--------G-|

5|-c-------------c--C-------|
4|a--aGgfdef-g-G------G-Ggfg|

4|--------------f-e---d---e-|
3|--------f-g-a-------------|

4|--f---g---e---a-------aGgF|

4|feDdC-------D-------------|

4|--fdfgGgfdGgfdeg--------G-|

5|--c-------------c-C-------|
4|a---aGgfdef-g-a-----G-Ggfg|

4|------------f-e---d---e---|
3|------f-g-a---------------|

4|f---g---e---a-------aGgFfe|

4|DdC-------D---------------|
3|--------------------b-----|

4|------f---e-------d-------|

4|f-------------------------|

4|------------------f---e---|
3|------b-------------------|

4|----d-----------d---------|

3|----------------------b---|

4|--------f---e-------d-----|

4|--f-----------------------|

4|--------------------f---e-|
3|--------b-----------------|

4|------d-------d-----------|

5|----d---------------d-----|
4|--dd--a--G-g-f-dfgdd--a--G|

5|----------d---------------|
4|-g-f-dfgCC--a--G-g-f-dfgcc|

5|d---------------d---------|
4|--a--G-g-f-dfgdd--a--G-g-f|

5|------d---------------d---|
4|-dfgdd--a--G-g-f-dfgCC--a-|

5|------------d-------------|
4|-G-g-f-dfgcc--a--G-g-f-dfg|";

enum Pitch {
    Rest,
    A,
    AS,
    B,
    BS,
    C,
    CS,
    D,
    DS,
    E,
    ES,
    F,
    FS,
    G,
    GS,
}

enum NoteDuration {
    Quarter,
    QuarterDot,
    Half,
    HalfDot,
    Whole,
    WholeDot,
    Breve,
    BreveDot,
}

struct Note {
    pitch: Pitch,
    octave: u8,
    duration: NoteDuration,
}

enum ParserState {
    BeginningOfLine,
    FirstBar,
    Body,
    SecondBar,
    EndOfLine,
}

pub fn make_qmk_note_sequence_from_letter_notes<I>(letter_notes: I) -> String
where
    I: IntoIterator<Item = char>,
{
    let letter_notes = letter_notes.into_iter();

    let mut pitch_seq_by_octave: [Vec<Pitch>; 9] = Default::default();

    let mut state = ParserState::BeginningOfLine;
    let mut cur_octave = 0u8;

    for l in letter_notes {
        match state {
            ParserState::BeginningOfLine => {
                cur_octave = l.to_digit(10).unwrap() as u8;
                state = ParserState::FirstBar;
            }
            ParserState::FirstBar => state = ParserState::Body,
            ParserState::Body => match l {
                '|' => state = ParserState::EndOfLine,
                l => pitch_seq_by_octave[cur_octave as usize].push(match l {
                    'a' => Pitch::A,
                    'A' => Pitch::AS,
                    'b' => Pitch::B,
                    'B' => Pitch::BS,
                    'c' => Pitch::C,
                    'C' => Pitch::CS,
                    'd' => Pitch::D,
                    'D' => Pitch::DS,
                    'e' => Pitch::E,
                    'E' => Pitch::ES,
                    'f' => Pitch::F,
                    'F' => Pitch::FS,
                    'g' => Pitch::G,
                    'G' => Pitch::GS,
                    _ => Pitch::Rest,
                }),
            },
            ParserState::SecondBar => state = ParserState::EndOfLine,
            ParserState::EndOfLine => state = ParserState::BeginningOfLine,
        }
    }

    "".into()
}
