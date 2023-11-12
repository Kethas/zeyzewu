use std::{
    f32::consts::E,
    fmt::{Display, Write},
    str::FromStr,
};

use derive_more::{Deref, DerefMut};
use rand::Rng;
use serde_derive::*;
use unicode_normalization::char::{compose, decompose_canonical};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CStem {
    P,
    T,
    K,
    S,
    R,
}

impl Random for CStem {
    fn random(rng: &mut impl Rng) -> Self {
        use CStem::*;
        match rng.gen_range(0..5) {
            0 => P,
            1 => T,
            2 => K,
            3 => S,
            4 => R,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PureC {
    Strong(CStem),
    Blunt(CStem),
    Sharp(CStem),
}
impl PureC {
    pub fn stem(&self) -> CStem {
        match self {
            PureC::Strong(stem) | PureC::Blunt(stem) | PureC::Sharp(stem) => *stem,
        }
    }
}

impl Display for PureC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PureC::Strong(stem) => match stem {
                CStem::P => "p",
                CStem::T => "t",
                CStem::K => "k",
                CStem::S => "s",
                CStem::R => "r",
            },
            PureC::Blunt(stem) => match stem {
                CStem::P => "b",
                CStem::T => "d",
                CStem::K => "g",
                CStem::S => "z",
                CStem::R => "rw",
            },
            PureC::Sharp(stem) => match stem {
                CStem::P => "py",
                CStem::T => "ty",
                CStem::K => "ky",
                CStem::S => "sy",
                CStem::R => "l",
            },
        };

        f.write_str(str)
    }
}

impl Random for PureC {
    fn random(rng: &mut impl Rng) -> Self {
        let inner = CStem::random(rng);

        use PureC::*;
        match rng.gen_range(0..3) {
            0 => Strong(inner),
            1 => Blunt(inner),
            2 => Sharp(inner),
            _ => unreachable!(),
        }
    }
}

impl Ipa for PureC {
    fn ipa(&self) -> String {
        match self {
            PureC::Strong(CStem::P) => "pʰ",
            PureC::Blunt(CStem::P) => "b",
            PureC::Sharp(CStem::P) => "pʲ",

            PureC::Strong(CStem::T) => "t̺ʰ",
            PureC::Blunt(CStem::T) => "d̺",
            PureC::Sharp(CStem::T) => "t̺ʲ",

            PureC::Strong(CStem::K) => "kʰ",
            PureC::Blunt(CStem::K) => "g",
            PureC::Sharp(CStem::K) => "kʲ",

            PureC::Strong(CStem::S) => "s̺",
            PureC::Blunt(CStem::S) => "z̺",
            PureC::Sharp(CStem::S) => "sʲ",

            PureC::Strong(CStem::R) => "r",
            PureC::Blunt(CStem::R) => "ɹ",
            PureC::Sharp(CStem::R) => "l",
        }
        .to_owned()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum H {
    W,
    Y,
    X,
    H,
}

impl Display for H {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            H::W => "w",
            H::Y => "y",
            H::X => "x",
            H::H => "h",
        };

        f.write_str(str)
    }
}

impl Random for H {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..4) {
            0 => H::W,
            1 => H::Y,
            2 => H::X,
            3 => H::H,
            _ => unreachable!(),
        }
    }
}

impl Ipa for H {
    fn ipa(&self) -> String {
        match self {
            H::W => format!("{}\u{032F}\u{0357}", V::U),
            H::Y => format!("{}\u{032F}\u{0351}", V::I),
            H::X => "x".to_owned(),
            H::H => "h".to_owned(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum C {
    H(H),     // 4
    C(PureC), // 15
}

impl Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            C::H(h) => h.fmt(f),
            C::C(c) => c.fmt(f),
        }
    }
}

impl From<H> for C {
    fn from(value: H) -> Self {
        Self::H(value)
    }
}

impl From<PureC> for C {
    fn from(value: PureC) -> Self {
        Self::C(value)
    }
}

impl Random for C {
    fn random(rng: &mut impl Rng) -> Self {
        if rng.gen_bool(4. / 15.) {
            C::H(H::random(rng))
        } else {
            C::C(PureC::random(rng))
        }
    }
}

impl Ipa for C {
    fn ipa(&self) -> String {
        match self {
            C::H(h) => h.ipa(),
            C::C(c) => c.ipa(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum V {
    A,
    E,
    I,
    O,
    U,
}

impl Display for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            V::A => "a",
            V::E => "e",
            V::I => "i",
            V::O => "o",
            V::U => "u",
        };

        f.write_str(str)
    }
}

impl Random for V {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..5) {
            0 => V::A,
            1 => V::E,
            2 => V::I,
            3 => V::O,
            4 => V::U,
            _ => unreachable!(),
        }
    }
}

impl Ipa for V {
    fn ipa(&self) -> String {
        match self {
            V::A => "ä",
            V::E => "e̞",
            V::I => "i",
            V::O => "o̞",
            V::U => "u",
        }
        .to_owned()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum T {
    High,
    Low,
    Peaking,
    Nasal,
}

impl Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = if f.alternate() {
            match self {
                T::High => ",",
                T::Low => "`",
                T::Peaking => "^",
                T::Nasal => "~",
            }
        } else {
            match self {
                T::High => "\u{0301}",
                T::Low => "\u{0300}",
                T::Peaking => "\u{0302}",
                T::Nasal => "\u{0303}",
            }
        };

        f.write_str(str)
    }
}

impl Random for T {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..4) {
            0 => T::High,
            1 => T::Low,
            2 => T::Peaking,
            3 => T::Nasal,
            _ => unreachable!(),
        }
    }
}

impl Ipa for T {
    fn ipa(&self) -> String {
        match self {
            T::High => "˥",
            T::Low => "˩",
            T::Peaking => "˧˥˧",
            T::Nasal => "\u{0303}",
        }
        .to_owned()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Punctuation {
    WordBreak,
    PhraseBreak,
    SentenceEnd,
}

impl Random for Punctuation {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..10) {
            0..=3 => Punctuation::WordBreak,
            4..=6 => Punctuation::PhraseBreak,
            7..=9 => Punctuation::SentenceEnd,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Syllable {
    pub onset: C,
    pub vowel: V,
    pub tone: T,
    pub coda: Option<H>,
}

impl Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.onset.fmt(f)?;

        let v = self.vowel.to_string().chars().next().unwrap();
        let t = self.tone.to_string().chars().next().unwrap();

        if let Some(ch) = compose(v, t) {
            f.write_char(ch)?
        } else {
            f.write_char(v)?;
            f.write_char(t)?;
        }

        if let Some(coda) = &self.coda {
            coda.fmt(f)?;
        }

        Ok(())
    }
}

impl FromStr for Syllable {
    type Err = usize;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        let mut index = 0;

        // parse initial consonant

        let first = *chars.get(index).ok_or(index)?;
        let second = *chars.get(index + 1).ok_or(index + 1)?;

        let compound = if second == 'y' || second == 'w' {
            index += 1;
            true
        } else {
            false
        };

        let onset = if compound {
            let str = format!("{first}{second}");
            match str.as_str() {
                "py" => PureC::Sharp(CStem::P),
                "ty" => PureC::Sharp(CStem::T),
                "ky" => PureC::Sharp(CStem::K),
                "sy" => PureC::Sharp(CStem::S),
                "rw" => PureC::Blunt(CStem::R),

                _ => return Err(index - 1),
            }
            .into()
        } else {
            match first {
                'p' => PureC::Strong(CStem::P).into(),
                'b' => PureC::Blunt(CStem::P).into(),
                't' => PureC::Strong(CStem::T).into(),
                'd' => PureC::Blunt(CStem::T).into(),
                'k' => PureC::Strong(CStem::K).into(),
                'g' => PureC::Blunt(CStem::K).into(),
                's' => PureC::Strong(CStem::S).into(),
                'z' => PureC::Blunt(CStem::S).into(),
                'r' => PureC::Strong(CStem::R).into(),
                'l' => PureC::Sharp(CStem::R).into(),

                'w' => H::W.into(),
                'y' => H::Y.into(),
                'x' => H::X.into(),
                'h' => H::H.into(),

                _ => return Err(index),
            }
        };

        index += 1;

        // parse vowel
        let ch = *chars.get(index).ok_or(index)?;

        let mut vowel_ch = ch;
        let mut tone_ch = None;
        let mut counter = 0;

        decompose_canonical(ch, |ch| {
            if counter == 0 {
                vowel_ch = ch;
            } else if counter == 1 {
                tone_ch = Some(ch);
            } else {
                return;
            }

            counter += 1;
        });

        let vowel = match vowel_ch {
            'a' => V::A,
            'e' => V::E,
            'i' => V::I,
            'o' => V::O,
            'u' => V::U,

            _ => {
                eprintln!("invalid char: '{ch}'");
                return Err(index);
            }
        };

        index += 1;

        // parse tone
        let ch = if let Some(tone_ch) = tone_ch {
            tone_ch
        } else {
            chars.get(index).copied().ok_or(index)?
        };

        let tone = match ch {
            ',' | '\u{0301}' | '´' => T::High,
            '`' | '\u{0300}' => T::Low,
            '^' | '\u{0302}' => T::Peaking,
            '~' | '\u{0303}' => T::Nasal,

            _ => {
                eprintln!("invalid char: '{ch}'");
                return Err(index);
            }
        };

        index += 1;

        // parse coda
        let coda = chars.get(index).and_then(|ch| match ch {
            'w' => Some(H::W),
            'y' => Some(H::Y),
            'x' => Some(H::X),
            'h' => Some(H::H),

            _ => None,
        });

        Ok(Syllable {
            onset,
            vowel,
            tone,
            coda,
        })
    }
}

impl Random for Syllable {
    fn random(rng: &mut impl Rng) -> Self {
        Self {
            onset: C::random(rng),
            vowel: V::random(rng),
            tone: T::random(rng),
            coda: if rng.gen_bool(4. / 5.) {
                Some(H::random(rng))
            } else {
                None
            },
        }
    }
}

impl Ipa for Syllable {
    fn ipa(&self) -> String {
        let mut buffer = String::new();

        buffer.push_str(&self.onset.ipa());
        buffer.push_str(&self.vowel.ipa());

        if self.tone == T::Nasal {
            buffer.push_str(&self.tone.ipa());
        }

        if let Some(coda) = &self.coda {
            buffer.push_str(&coda.ipa());
        };

        if self.tone != T::Nasal {
            buffer.push_str(&self.tone.ipa());
        } else {
            buffer.push_str(&T::Low.ipa());
        }

        buffer
    }
}

pub fn syllable(str: &str) -> Syllable {
    str.parse().unwrap()
}

#[derive(Clone, Deref, DerefMut, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Word(#[deref] pub Vec<Syllable>);

impl IntoIterator for Word {
    type Item = Syllable;

    type IntoIter = <Vec<Syllable> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for syllable in &self.0 {
            if first {
                first = false;
            } else {
                f.write_char('-')?;
            }

            syllable.fmt(f)?;
        }

        Ok(())
    }
}

impl FromStr for Word {
    type Err = (usize, usize);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let syllables = s.split('-');

        let mut word = Vec::new();

        for (i, syllable) in syllables.enumerate() {
            word.push(syllable.parse().map_err(|e| (i, e))?);
        }

        Ok(Word(word))
    }
}

impl Random for Word {
    fn random(rng: &mut impl Rng) -> Self {
        let mut b = Vec::new();

        let mut n = 1.;

        loop {
            let chance = 1. / n;

            if rng.gen_bool(chance) {
                b.push(Syllable::random(rng))
            } else {
                break;
            }

            n += 1.;
        }

        Self(b)
    }
}

impl Ipa for Word {
    fn ipa(&self) -> String {
        let mut buffer = String::new();

        let mut first = true;
        for syllable in &self.0 {
            if first {
                first = false;
            } else {
                buffer.push('.')
            }

            buffer.push_str(&syllable.ipa())
        }

        buffer
    }
}

pub fn word(word: &str) -> Word {
    word.parse().unwrap()
}

#[derive(Clone, Deref, DerefMut, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Phrase(#[deref] pub Vec<Word>);

impl IntoIterator for Phrase {
    type Item = Word;

    type IntoIter = <Vec<Word> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Display for Phrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for word in &self.0 {
            if first {
                first = false;
            } else {
                f.write_char(' ')?;
            }
            word.fmt(f)?;
        }

        Ok(())
    }
}

impl FromStr for Phrase {
    type Err = (usize, usize, usize);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace();

        let mut phrase = Vec::new();

        for (i, word) in words.enumerate() {
            phrase.push(word.parse().map_err(|(e1, e2)| (i, e1, e2))?);
        }

        Ok(Phrase(phrase))
    }
}

impl Random for Phrase {
    fn random(rng: &mut impl Rng) -> Self {
        let mut b = Vec::new();

        let mut n = 1.;

        loop {
            let chance = 1. / n;

            if rng.gen_bool(chance) {
                b.push(Word::random(rng))
            } else {
                break;
            }

            n += 1.;
        }

        Self(b)
    }
}

impl Ipa for Phrase {
    fn ipa(&self) -> String {
        let mut buffer = String::new();

        let mut first = true;
        for word in &self.0 {
            if first {
                first = false;
            } else {
                buffer.push(' ')
            }

            buffer.push_str(&word.ipa())
        }

        buffer
    }
}

pub fn phrase(phrase: &str) -> Phrase {
    phrase.parse().unwrap()
}

#[derive(Clone, Deref, DerefMut, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sentence(#[deref] pub Vec<Phrase>);

impl IntoIterator for Sentence {
    type Item = Phrase;

    type IntoIter = <Vec<Phrase> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for phrase in &self.0 {
            if first {
                first = false;
            } else {
                f.write_str(" ")?;
            }
            phrase.fmt(f)?;
        }

        f.write_char('.')?;

        Ok(())
    }
}

impl FromStr for Sentence {
    type Err = (usize, usize, usize, usize);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let phrases = s.split(": ");

        let mut sentence = Vec::new();

        for (i, phrase) in phrases.enumerate() {
            sentence.push(phrase.parse().map_err(|(e1, e2, e3)| (i, e1, e2, e3))?);
        }

        Ok(Sentence(sentence))
    }
}

impl Random for Sentence {
    fn random(rng: &mut impl Rng) -> Self {
        let mut b = Vec::new();

        let mut n = 1.;

        loop {
            let chance = 1. / n;

            if rng.gen_bool(chance) {
                b.push(Phrase::random(rng))
            } else {
                break;
            }

            n += 1.;
        }

        Self(b)
    }
}

impl Ipa for Sentence {
    fn ipa(&self) -> String {
        let mut buffer = String::new();

        let mut first = true;
        for phrase in &self.0 {
            if first {
                first = false;
            } else {
                buffer.push_str(", ")
            }

            buffer.push_str(&phrase.ipa())
        }

        buffer
    }
}

pub fn sentence(sentence: &str) -> Sentence {
    sentence.parse().unwrap()
}

#[derive(Clone, Deref, DerefMut, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Paragraph(#[deref] pub Vec<Sentence>);

impl IntoIterator for Paragraph {
    type Item = Sentence;

    type IntoIter = <Vec<Sentence> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Display for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for sentence in &self.0 {
            sentence.fmt(f)?;
            f.write_char(' ')?;
        }

        Ok(())
    }
}

impl FromStr for Paragraph {
    type Err = (usize, usize, usize, usize, usize);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sentences = s.split(". ");

        let mut paragraph = Vec::new();

        for (i, sentence) in sentences.enumerate() {
            paragraph.push(
                sentence
                    .parse()
                    .map_err(|(e1, e2, e3, e4)| (i, e1, e2, e3, e4))?,
            );
        }

        Ok(Paragraph(paragraph))
    }
}

impl Random for Paragraph {
    fn random(rng: &mut impl Rng) -> Self {
        let mut b = Vec::new();

        let mut n = 1.;

        loop {
            let chance = 1. / n;

            if rng.gen_bool(chance) {
                b.push(Sentence::random(rng))
            } else {
                break;
            }

            n += 1.;
        }

        Self(b)
    }
}

impl Ipa for Paragraph {
    fn ipa(&self) -> String {
        let mut buffer = String::new();

        for sentence in &self.0 {
            buffer.push_str(&sentence.ipa());
            buffer.push_str(". ");
        }

        buffer
    }
}

pub fn paragraph(paragraph: &str) -> Paragraph {
    paragraph.parse().unwrap()
}

#[derive(Clone, Deref, DerefMut, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Text(#[deref] pub Vec<Paragraph>);

impl IntoIterator for Text {
    type Item = Paragraph;

    type IntoIter = <Vec<Paragraph> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for sentence in &self.0 {
            sentence.fmt(f)?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl FromStr for Text {
    type Err = (usize, usize, usize, usize, usize, usize);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paragraphs = s.split('\n');

        let mut text = Vec::new();

        for (i, paragraph) in paragraphs.enumerate() {
            text.push(
                paragraph
                    .parse()
                    .map_err(|(e1, e2, e3, e4, e5)| (i, e1, e2, e3, e4, e5))?,
            );
        }

        Ok(Text(text))
    }
}

impl Random for Text {
    fn random(rng: &mut impl Rng) -> Self {
        let mut b = Vec::new();

        let mut n = 1.;

        loop {
            let chance = 1. / n;

            if rng.gen_bool(chance) {
                b.push(Paragraph::random(rng))
            } else {
                break;
            }

            n += 1.;
        }

        Self(b)
    }
}

impl Ipa for Text {
    fn ipa(&self) -> String {
        let mut buffer = String::new();

        for paragraph in &self.0 {
            buffer.push_str(&paragraph.ipa());
            buffer.push('\n');
        }

        buffer
    }
}

pub fn text(text: &str) -> Text {
    text.parse().unwrap()
}

pub trait Random {
    fn random(rng: &mut impl Rng) -> Self;
}

pub trait Ipa {
    fn ipa(&self) -> String;
}
