const NUMBER_OF_OCTAVES: usize = 9;

#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Pitch {
    fn as_c_macro(self, octave: u8) -> String {
        use Pitch::*;

        let base = match self {
            Rest => "NOTE_REST",
            A => "NOTE_A",
            AS => "NOTE_AS",
            B => "NOTE_B",
            BS => "NOTE_BS",
            C => "NOTE_C",
            CS => "NOTE_CS",
            D => "NOTE_D",
            DS => "NOTE_DS",
            E => "NOTE_E",
            ES => "NOTE_ES",
            F => "NOTE_F",
            FS => "NOTE_FS",
            G => "NOTE_G",
            GS => "NOTE_GS",
        };
        let octave_str = if self == Rest {
            "".into()
        } else {
            format!("{}", octave)
        };
        format!("{}{}", base, octave_str)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum NoteDuration {
    Quarter,
    QuarterDot,
    Half,
    HalfDot,
    Whole,
    WholeDot,
    Breve,
    BreveDot,
    Other(u8),
}
impl NoteDuration {
    fn from_beats(beats: u8) -> Self {
        NoteDuration::Other(beats).normalize()
    }
    fn normalize(self) -> Self {
        use NoteDuration::*;
        if let Other(beats) = self {
            match beats {
                1 => Quarter,
                2 => Half,
                3 => HalfDot,
                4 => Whole,
                6 => WholeDot,
                8 => Breve,
                12 => BreveDot,
                _ => self,
            }
        } else {
            self
        }
    }
    fn as_c_macro(&self, pitch: Pitch, octave: u8) -> String {
        use NoteDuration::*;
        let base = match self {
            Quarter => "Q__NOTE",
            QuarterDot => "QD_NOTE",
            Half => "H__NOTE",
            HalfDot => "HD_NOTE",
            Whole => "W__NOTE",
            WholeDot => "WD_NOTE",
            Breve => "B__NOTE",
            BreveDot => "BD_NOTE",
            Other(_) => "M__NOTE",
        };
        let dur_arg = if let Other(beats) = self {
            format!(", {}", *beats as u64 * 16)
        } else {
            "".into()
        };
        format!("{}({}{})", base, pitch.as_c_macro(octave), dur_arg)
    }
}

struct Note {
    pitch: Pitch,
    octave: u8,
    duration: NoteDuration,
}
impl Note {
    fn as_c_macro(&self) -> String {
        self.duration.as_c_macro(self.pitch, self.octave)
    }
}

enum ParserState {
    BeginningOfLine,
    FirstBar,
    Body,
    EndOfLine,
}

pub fn make_qmk_note_sequence_from_letter_notes<I>(letter_notes: I) -> String
where
    I: IntoIterator<Item = char>,
{
    let letter_notes = letter_notes.into_iter();

    let mut pitch_seq_by_octave: [Vec<Pitch>; NUMBER_OF_OCTAVES] = Default::default();
    let mut is_octave_in_cur_section: [bool; NUMBER_OF_OCTAVES] = Default::default();

    let mut state = ParserState::BeginningOfLine;
    let mut cur_octave = 0u8;
    let mut cur_section_length = 0u8;

    for l in letter_notes {
        match state {
            ParserState::BeginningOfLine => match l {
                '\n' => {
                    for (octave, in_cur_section) in is_octave_in_cur_section.iter().enumerate() {
                        if !*in_cur_section {
                            pitch_seq_by_octave[octave].append(&mut vec![
                                Pitch::Rest;
                                cur_section_length
                                    as usize
                            ]);
                        }
                    }

                    for b in &mut is_octave_in_cur_section {
                        *b = false
                    }
                    cur_section_length = 0;
                }
                _ => {
                    cur_octave = l.to_digit(10).unwrap() as u8;
                    state = ParserState::FirstBar;
                }
            },
            ParserState::FirstBar => state = ParserState::Body,
            ParserState::Body => match l {
                '|' => state = ParserState::EndOfLine,
                _ => {
                    cur_section_length += 1;
                    pitch_seq_by_octave[cur_octave as usize].push(match l {
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
                    })
                }
            },
            ParserState::EndOfLine => state = ParserState::BeginningOfLine,
        }
    }

    let whole_sequence_length = pitch_seq_by_octave[0].len();

    let mut zipped_pitch_sequence: Vec<(u8, Pitch)> = vec![(0, Pitch::Rest); whole_sequence_length];
    for i in 0..whole_sequence_length {
        for octave in 0..NUMBER_OF_OCTAVES {
            let p = pitch_seq_by_octave[octave][i];
            if p != Pitch::Rest || octave == NUMBER_OF_OCTAVES - 1 {
                zipped_pitch_sequence[i] = (octave as u8, p);
                break;
            }
        }
    }

    let ((octave, pitch), rest) = zipped_pitch_sequence.split_first().unwrap();
    let mut cur_octave = *octave;
    let mut cur_pitch = *pitch;
    let mut beats_in_cur_note = 1u8;

    let mut collapsed_note_sequence: Vec<Note> = vec![];
    for (octave, pitch) in rest {
        if *pitch == Pitch::Rest {
            beats_in_cur_note += 1;
        } else {
            collapsed_note_sequence.push(Note {
                octave: cur_octave,
                pitch: cur_pitch,
                duration: NoteDuration::from_beats(beats_in_cur_note),
            });
            cur_octave = *octave;
            cur_pitch = *pitch;
            beats_in_cur_note = 1;
        }
    }
    collapsed_note_sequence.push(Note {
        octave: cur_octave,
        pitch: cur_pitch,
        duration: NoteDuration::from_beats(beats_in_cur_note),
    });

    collapsed_note_sequence
        .iter()
        .map(|n| n.as_c_macro())
        .collect::<Vec<_>>()
        .join(",")
}

mod test {
    const MEGALOVANIA_RAW: &'static str = "5|--d---------------d-------|
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

    #[test]
    fn test_megalovania() {
        println!(
            "{}",
            super::make_qmk_note_sequence_from_letter_notes(MEGALOVANIA_RAW.chars())
        );
    }
}
