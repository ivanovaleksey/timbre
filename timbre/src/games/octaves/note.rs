use std::fmt;
use super::Sample;

lazy_static! {
    pub static ref GAMUTS: Vec<Gamut> = {
        let mut v = Vec::new();

        v.push(Gamut {
            key: Pitch::C,
            scale: [Pitch::C, Pitch::D, Pitch::E, Pitch::F, Pitch::G, Pitch::A, Pitch::B],
        });
        v.push(Gamut {
            key: Pitch::G,
            scale: [Pitch::G, Pitch::A, Pitch::B, Pitch::C, Pitch::D, Pitch::E, Pitch::Fsharp],
        });
        v.push(Gamut {
            key: Pitch::D,
            scale: [Pitch::D, Pitch::E, Pitch::Fsharp, Pitch::G, Pitch::A, Pitch::B, Pitch::Csharp],
        });
        v.push(Gamut {
            key: Pitch::A,
            scale: [Pitch::A, Pitch::B, Pitch::Csharp, Pitch::D, Pitch::E, Pitch::Fsharp, Pitch::Gsharp],
        });
        v.push(Gamut {
            key: Pitch::E,
            scale: [Pitch::E, Pitch::Fsharp, Pitch::Gsharp, Pitch::A, Pitch::B, Pitch::Csharp, Pitch::Dsharp],
        });
        v.push(Gamut {
            key: Pitch::B,
            scale: [Pitch::B, Pitch::Csharp, Pitch::Dsharp, Pitch::E, Pitch::Fsharp, Pitch::Gsharp, Pitch::Asharp],
        });

        v.push(Gamut {
            key: Pitch::F,
            scale: [Pitch::F, Pitch::G, Pitch::A, Pitch::Bflat, Pitch::C, Pitch::D, Pitch::E],
        });
        v.push(Gamut {
            key: Pitch::Bflat,
            scale: [Pitch::Bflat, Pitch::C, Pitch::D, Pitch::Eflat, Pitch::F, Pitch::G, Pitch::A],
        });
        v.push(Gamut {
            key: Pitch::Eflat,
            scale: [Pitch::Eflat, Pitch::F, Pitch::G, Pitch::Aflat, Pitch::Bflat, Pitch::C, Pitch::D],
        });
        v.push(Gamut {
            key: Pitch::Aflat,
            scale: [Pitch::Aflat, Pitch::Bflat, Pitch::C, Pitch::Dflat, Pitch::Eflat, Pitch::F, Pitch::G],
        });
        v.push(Gamut {
            key: Pitch::Dflat,
            scale: [Pitch::Dflat, Pitch::Eflat, Pitch::F, Pitch::Gflat, Pitch::Aflat, Pitch::Bflat, Pitch::C],
        });
        v.push(Gamut {
            key: Pitch::Fsharp,
            scale: [Pitch::Fsharp, Pitch::Gsharp, Pitch::Asharp, Pitch::B, Pitch::Csharp, Pitch::Dsharp, Pitch::F],
        });

        v
    };

    pub static ref TONALITIES: Vec<Tonality> = {
        let mut v = Vec::new();

        v.push(Tonality(Pitch::C));
        v.push(Tonality(Pitch::G));
        v.push(Tonality(Pitch::D));
        v.push(Tonality(Pitch::A));
        v.push(Tonality(Pitch::E));
        v.push(Tonality(Pitch::B));

        v.push(Tonality(Pitch::F));
        v.push(Tonality(Pitch::Bflat));
        v.push(Tonality(Pitch::Eflat));
        v.push(Tonality(Pitch::Aflat));
        v.push(Tonality(Pitch::Dflat));
        v.push(Tonality(Pitch::Fsharp));

        v
    };
}

// Represent a pitch from a particular octave.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Note {
    pub octave: Octave,
    pub pitch: Pitch,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Octave {
    Great = 2,
    Small = 3,
    First = 4,
    Second = 5,
    Third = 6,
    Fourth = 7,
}

impl Octave {
    pub fn next(&self) -> Option<Octave> {
        match *self {
            Octave::Great => Some(Octave::Small),
            Octave::Small => Some(Octave::First),
            Octave::First => Some(Octave::Second),
            Octave::Second => Some(Octave::Third),
            Octave::Third => Some(Octave::Fourth),
            Octave::Fourth => None,
        }
    }
}

type Scale = [Pitch; 7];

#[derive(Copy, Clone, Debug)]
pub struct Tonality(pub Pitch);

#[derive(Debug)]
pub struct Gamut {
    pub key: Pitch,
    pub scale: Scale,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pitch {
    Cflat,
    C,
    Csharp,
    Dflat,
    D,
    Dsharp,
    Eflat,
    E,
    Esharp,
    Fflat,
    F,
    Fsharp,
    Gflat,
    G,
    Gsharp,
    Aflat,
    A,
    Asharp,
    Bflat,
    B,
    Bsharp,
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self);
        let pitch = s.chars().nth(0).unwrap();
        let sign: &str = if s.ends_with("flat") {
            "b"
        } else if s.ends_with("sharp") {
            "#"
        } else {
            ""
        };

        write!(f, "{}{}", pitch, sign)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pitch_display() {
        assert_eq!(Pitch::C.to_string(), "C");
        assert_eq!(Pitch::Cflat.to_string(), "Cb");
        assert_eq!(Pitch::Csharp.to_string(), "C#");

        assert_eq!(Pitch::D.to_string(), "D");
        assert_eq!(Pitch::Dflat.to_string(), "Db");
        assert_eq!(Pitch::Dsharp.to_string(), "D#");
    }
}
