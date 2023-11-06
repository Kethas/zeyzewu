use std::{
    fmt::{Display, Write},
    str::FromStr,
    sync::{Arc, Mutex},
};

use nannou::{
    draw::properties::spatial::position,
    prelude::*,
    rand::{thread_rng, Rng},
};

impl Drawable for wa::CStem {
    fn draw(&self, page: &Page, char_centre: (f32, f32), draw: &Draw) {
        let y = char_centre.1 + 0.5 * page.char_height() - 0.75 * page.short_stroke_length();

        match self {
            wa::CStem::P => {
                // draw long stroke
                let x = char_centre.0;
                let (w, h) = (page.long_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

                // draw left short stroke
                // primary stroke x - half a long stroke + half a stroke width
                let x = x - page.long_stroke_length() / 2.0 + page.stroke_width() / 2.0;
                let (w, h) = (page.stroke_width(), page.short_stroke_length());
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
                // draw right short stroke
                // left short stroke x + long stroke - stroke width
                let x = x + page.long_stroke_length() - page.stroke_width();
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
            wa::CStem::T => {
                // draw long stroke
                let x = char_centre.0;
                let (w, h) = (page.long_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

                // draw left short stroke
                let y = y + page.short_stroke_length() / 2.0;
                let x = x - page.long_stroke_length() / 2.0 + page.stroke_width() / 2.0;
                let (w, h) = (page.stroke_width(), page.short_stroke_length());
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
                // draw right short stroke
                let x = x + page.long_stroke_length() - page.stroke_width();
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
            wa::CStem::K => {
                // draw long stroke
                let x = char_centre.0;
                let (w, h) = (page.long_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

                // draw left short stroke
                let y = y - page.short_stroke_length() / 2.0;
                let x = x - page.long_stroke_length() / 2.0 + page.stroke_width() / 2.0;
                let (w, h) = (page.stroke_width(), page.short_stroke_length());
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
                // draw right short stroke
                let x = x + page.long_stroke_length() - page.stroke_width();
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
            wa::CStem::S => {
                // draw mid stroke
                let x = char_centre.0;
                let (w, h) = (page.mid_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

                // draw left short stroke
                // primary stroke x - half a mid stroke + half a stroke width
                let x = x - page.mid_stroke_length() / 2.0 + page.stroke_width() / 2.0;
                let (w, h) = (page.stroke_width(), page.short_stroke_length());
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
                // draw right short stroke
                // left short stroke x + mid stroke - stroke widht
                let x = x + page.mid_stroke_length() - page.stroke_width();
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
            wa::CStem::R => {
                // draw mid stroke
                let x = char_centre.0;
                let (w, h) = (page.mid_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
        }
    }
}

impl Drawable for wa::PureC {
    fn draw(&self, page: &Page, char_centre: (f32, f32), draw: &Draw) {
        let stem = self.stem();

        stem.draw(page, char_centre, draw);

        let y = char_centre.1 + 0.5 * page.char_height() - 0.75 * page.short_stroke_length()
            + page.short_stroke_length() / 2.0;

        if let wa::PureC::Blunt(_) = self {
            // vertical short stroke in the middle
            draw.rect()
                .color(page.fg)
                .x_y(char_centre.0, y)
                .w_h(page.stroke_width(), page.short_stroke_length());
        }

        if let wa::PureC::Sharp(_) = self {
            let spacing = match stem {
                wa::CStem::R | wa::CStem::S => page.mid_stroke_length(),
                _ => page.long_stroke_length(),
            } / 6.0;

            // vertical short stroke on the left
            draw.rect()
                .color(page.fg)
                .x_y(char_centre.0 - spacing, y)
                .w_h(page.stroke_width(), page.short_stroke_length());

            // vertical short stroke on the right
            draw.rect()
                .color(page.fg)
                .x_y(char_centre.0 + spacing, y)
                .w_h(page.stroke_width(), page.short_stroke_length());
        }
    }
}

impl DrawableTopOrBottom for wa::H {
    fn draw(&self, page: &Page, char_centre: (f32, f32), top: bool, draw: &Draw) {
        let modifier = if top { 1.0 } else { -1.0 };
        let y = char_centre.1
            + modifier * (0.5 * page.char_height() - 0.75 * page.short_stroke_length());

        match self {
            wa::H::W => {
                // draw long stroke
                let y = y + page.short_stroke_length() / 2.;
                let x = char_centre.0;
                let (w, h) = (page.long_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

                // draw mid stroke
                let y = y - page.short_stroke_length();
                let (w, h) = (page.mid_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
            wa::H::Y => {
                // draw mid stroke
                let y = y + page.short_stroke_length() / 2.;
                let x = char_centre.0;
                let (w, h) = (page.mid_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

                // draw mid stroke
                let y = y - page.short_stroke_length();
                let (w, h) = (page.mid_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
            wa::H::X => {
                // draw mid stroke
                let x = char_centre.0;
                let (w, h) = (page.mid_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

                // draw left short stroke
                // primary stroke x - half a mid stroke + half a stroke width
                let x = x - page.mid_stroke_length() / 2.0 + page.stroke_width() / 2.0;
                let y = y - page.short_stroke_length() / 2.0;
                let (w, h) = (page.stroke_width(), page.short_stroke_length());
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
                // draw right short stroke
                // left short stroke x + mid stroke - stroke widht
                let x = x + page.mid_stroke_length() - page.stroke_width();
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
            wa::H::H => {
                // draw mid stroke
                let x = char_centre.0;
                let (w, h) = (page.mid_stroke_length(), page.stroke_width());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

                // draw left short stroke
                // primary stroke x - half a mid stroke + half a stroke width
                let x = x - page.mid_stroke_length() / 2.0 + page.stroke_width() / 2.0;
                let y = y + page.short_stroke_length() / 2.0;
                let (w, h) = (page.stroke_width(), page.short_stroke_length());
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
                // draw right short stroke
                // left short stroke x + mid stroke - stroke widht
                let x = x + page.mid_stroke_length() - page.stroke_width();
                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
        }
    }
}

impl Drawable for wa::C {
    fn draw(&self, page: &Page, char_centre: (f32, f32), draw: &Draw) {
        match self {
            wa::C::H(h) => h.draw(page, char_centre, true, draw),
            wa::C::C(c) => c.draw(page, char_centre, draw),
        }
    }
}

impl Drawable for wa::V {
    fn draw(&self, page: &Page, char_centre: (f32, f32), draw: &Draw) {
        let (x, y) = char_centre;
        match self {
            wa::V::A => {
                // draw mid stroke
                let x = x + page.short_stroke_length() / 2.;
                let (w, h) = (page.stroke_width(), page.short_stroke_length());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

                // draw mid stroke
                let x = x - page.short_stroke_length();
                let (w, h) = (page.stroke_width(), page.short_stroke_length());

                draw.rect().color(page.fg).x_y(x, y).w_h(w, h);
            }
            wa::V::E => {
                draw.rect()
                    .color(page.fg)
                    .x_y(x, y)
                    .w_h(page.mid_stroke_length(), page.stroke_width());
                draw.rect()
                    .color(page.fg)
                    .x_y(x, y - page.short_stroke_length() / 2.0)
                    .w_h(page.stroke_width(), page.short_stroke_length());
            }
            wa::V::I => {
                draw.rect()
                    .color(page.fg)
                    .x_y(x, y)
                    .w_h(page.stroke_width(), page.mid_stroke_length());
            }
            wa::V::O => {
                draw.rect()
                    .color(page.fg)
                    .x_y(x, y)
                    .w_h(page.stroke_width(), page.short_stroke_length());
            }
            wa::V::U => {
                draw.rect()
                    .color(page.fg)
                    .x_y(x, y)
                    .w_h(page.stroke_width(), page.long_stroke_length());
            }
        }
    }
}

impl Drawable for wa::T {
    fn draw(&self, page: &Page, char_centre: (f32, f32), draw: &Draw) {
        let (x, y) = char_centre;

        let spacing = (page.mid_stroke_length()) / 2.0 + page.stroke_width();

        match self {
            wa::T::High => {
                draw.rect()
                    .color(page.fg)
                    .x_y(x - spacing, y)
                    .w_h(page.stroke_width(), page.short_stroke_length());
                draw.rect()
                    .color(page.fg)
                    .x_y(x + spacing, y)
                    .w_h(page.stroke_width(), page.short_stroke_length());
            }
            wa::T::Low => { /* NOP */ }
            wa::T::Peaking => {
                draw.rect()
                    .color(page.fg)
                    .x_y(x - spacing, y)
                    .w_h(page.stroke_width(), page.mid_stroke_length());
                draw.rect()
                    .color(page.fg)
                    .x_y(x + spacing, y)
                    .w_h(page.stroke_width(), page.mid_stroke_length());
            }
            wa::T::Nasal => {
                draw.rect()
                    .color(page.fg)
                    .x_y(x - spacing, y)
                    .w_h(page.stroke_width(), page.long_stroke_length());
                draw.rect()
                    .color(page.fg)
                    .x_y(x + spacing, y)
                    .w_h(page.stroke_width(), page.long_stroke_length());
            }
        }
    }
}
impl Drawable for wa::Punctuation {
    fn draw(&self, page: &Page, punct_centre: (f32, f32), draw: &Draw) {
        let (x, y) = punct_centre;

        match self {
            wa::Punctuation::WordBreak => {
                draw.rect()
                    .color(page.fg)
                    .x_y(x, y)
                    .w_h(page.short_stroke_length(), page.stroke_width());
            }
            wa::Punctuation::PhraseBreak => {
                let spacing = page.short_stroke_length() / 2.;

                draw.rect()
                    .color(page.fg)
                    .x_y(x, y + spacing)
                    .w_h(page.short_stroke_length(), page.stroke_width());

                draw.rect()
                    .color(page.fg)
                    .x_y(x, y - spacing)
                    .w_h(page.short_stroke_length(), page.stroke_width());
            }
            wa::Punctuation::SentenceEnd => {
                draw.rect()
                    .color(page.fg)
                    .x_y(x, y)
                    .w_h(page.short_stroke_length(), page.stroke_width());

                let spacing = page.short_stroke_length() / 2.;

                draw.rect()
                    .color(page.fg)
                    .x_y(x + spacing, y)
                    .w_h(page.stroke_width(), page.short_stroke_length());

                draw.rect()
                    .color(page.fg)
                    .x_y(x - spacing, y)
                    .w_h(page.stroke_width(), page.short_stroke_length());
            }
        }
    }
}

impl Print for wa::Punctuation {
    fn print(&self, printer: &mut Printer, draw: &Draw) {
        printer.print_punctuation(self, draw);
    }
}

impl Print for wa::Syllable {
    fn print(&self, printer: &mut Printer, draw: &Draw) {
        printer.print_syllable(self, draw)
    }
}

impl Print for wa::Word {
    fn print(&self, printer: &mut Printer, draw: &Draw) {
        for syllable in &self.0 {
            printer.print(syllable, draw);
        }
    }
}

impl Print for wa::Phrase {
    fn print(&self, printer: &mut Printer, draw: &Draw) {
        let mut first = true;
        for word in &self.0 {
            if first {
                first = false;
            } else {
                printer.print(Punctuation::WordBreak, draw);
            }
            printer.print(word, draw);
        }
    }
}

impl Print for wa::Sentence {
    fn print(&self, printer: &mut Printer, draw: &Draw) {
        let mut first = true;
        for phrase in &self.0 {
            if first {
                first = false;
            } else {
                printer.print(Punctuation::PhraseBreak, draw);
            }
            printer.print(phrase, draw);
        }
    }
}

impl Print for wa::Paragraph {
    fn print(&self, printer: &mut Printer, draw: &Draw) {
        for sentence in &self.0 {
            printer.print(sentence, draw);
            printer.print(Punctuation::SentenceEnd, draw);
        }
    }
}

impl Print for wa::Text {
    fn print(&self, printer: &mut Printer, draw: &Draw) {
        for paragraph in &self.0 {
            printer.print(paragraph, draw);
            printer.newline();
        }
    }
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

pub struct Page {
    // the dimensions of the page
    pub dimensions: (f32, f32),

    // the margins on the edges of the page
    pub margin: f32,
    // the padding on the edges of each character
    pub padding: f32,
    // the spacing between characters (multiples of char-width)
    pub char_spacing_ratio: f32,
    // the spacing between lines (multiples of char_width)
    pub line_spacing: f32,
    // the spacing between segments within a char
    pub segment_spacing_ratio: f32,
    // the total width of a character (excluding the padding)
    pub char_width: f32,
    // the ratio between char_width - 2padding and long_stroke_length
    pub long_stroke_ratio: f32,
    // the ratio between char_width - 2padding and mid_stroke_length
    pub mid_stroke_ratio: f32,
    // the ratio between char_width - 2padding and short_stroke_length
    pub short_stroke_ratio: f32,
    // the ratio between char_width - 2padding and stroke_width
    pub stroke_width_ratio: f32,

    pub bg: Srgb<u8>,
    pub fg: Srgb<u8>,
}

impl Page {
    pub fn new(dimensions: (f32, f32)) -> Self {
        Self {
            dimensions,
            margin: 30.,
            padding: 0.,
            char_spacing_ratio: 0.065,
            line_spacing: 1.,
            segment_spacing_ratio: -0.05,
            char_width: 75.,
            long_stroke_ratio: 0.3,
            mid_stroke_ratio: 0.15,
            short_stroke_ratio: 0.06,
            stroke_width_ratio: 0.02,

            fg: WHITE,
            bg: BLACK,
        }
    }

    pub fn update_dimensions(&mut self, dimensions: (f32, f32)) {
        self.dimensions = dimensions
    }

    pub fn change_attribute(&mut self, attribute: Attribute, modifier: f32) {
        let amount = modifier * attribute.change_amount();

        match attribute {
            Attribute::Margin => self.margin += amount,
            Attribute::Padding => self.padding += amount,
            Attribute::CharSpacingRatio => self.char_spacing_ratio += amount,
            Attribute::LineSpacingRatio => self.line_spacing += amount,
            Attribute::SegmentSpacingRatio => self.segment_spacing_ratio += amount,
            Attribute::CharWidth => self.char_width += amount,
            Attribute::LongStrokeRatio => self.long_stroke_ratio += amount,
            Attribute::MidStrokeRatio => self.mid_stroke_ratio += amount,
            Attribute::ShortStrokeRatio => self.short_stroke_ratio += amount,
            Attribute::StrokeWidthRatio => self.stroke_width_ratio += amount,
        }
    }

    pub fn get_attribute(&self, attribute: Attribute) -> f32 {
        match attribute {
            Attribute::Margin => self.margin,
            Attribute::Padding => self.padding,
            Attribute::CharSpacingRatio => self.char_spacing_ratio,
            Attribute::LineSpacingRatio => self.line_spacing,
            Attribute::SegmentSpacingRatio => self.segment_spacing_ratio,
            Attribute::CharWidth => self.char_width,
            Attribute::LongStrokeRatio => self.long_stroke_ratio,
            Attribute::MidStrokeRatio => self.mid_stroke_ratio,
            Attribute::ShortStrokeRatio => self.short_stroke_ratio,
            Attribute::StrokeWidthRatio => self.stroke_width_ratio,
        }
    }

    pub fn segment_spacing(&self) -> f32 {
        self.segment_spacing_ratio * self.char_width
    }

    pub fn char_spacing(&self) -> f32 {
        self.char_spacing_ratio * self.char_width
    }

    pub fn stroke_width(&self) -> f32 {
        self.base_stroke_length() * self.stroke_width_ratio
    }

    fn base_stroke_length(&self) -> f32 {
        self.char_width * 2. - self.padding * 2.
    }

    pub fn long_stroke_length(&self) -> f32 {
        self.base_stroke_length() * self.long_stroke_ratio
    }

    pub fn mid_stroke_length(&self) -> f32 {
        self.base_stroke_length() * self.mid_stroke_ratio
    }

    pub fn short_stroke_length(&self) -> f32 {
        self.base_stroke_length() * self.short_stroke_ratio
    }

    // with the margin applied
    pub fn right_edge(&self) -> f32 {
        self.dimensions.0 / 2. - self.margin
    }
    pub fn top_edge(&self) -> f32 {
        self.dimensions.1 / 2. - self.margin
    }
    pub fn bottom_edge(&self) -> f32 {
        -self.dimensions.1 / 2. + self.margin
    }

    pub fn char_height(&self) -> f32 {
        self.long_stroke_length()
            + 2. * self.segment_spacing()
            + 2. * 1.5 * self.short_stroke_length()
    }

    fn punctuation_height(&self) -> f32 {
        1.5 * self.short_stroke_length()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attribute {
    Margin,              // M
    Padding,             // P
    CharSpacingRatio,    // C
    SegmentSpacingRatio, // S
    LineSpacingRatio,    // L
    CharWidth,           // W
    LongStrokeRatio,     // R
    MidStrokeRatio,      // N
    ShortStrokeRatio,    // T
    StrokeWidthRatio,    // Q
}

pub const ATTRIBUTES: [Attribute; 10] = [
    Attribute::Margin,              // M
    Attribute::Padding,             // P
    Attribute::CharSpacingRatio,    // C
    Attribute::SegmentSpacingRatio, // S
    Attribute::LineSpacingRatio,    // L
    Attribute::CharWidth,           // W
    Attribute::LongStrokeRatio,     // R
    Attribute::MidStrokeRatio,      // N
    Attribute::ShortStrokeRatio,    // T
    Attribute::StrokeWidthRatio,    // Q
];

impl Attribute {
    pub fn change_amount(&self) -> f32 {
        match self {
            Attribute::Margin => 1.,
            Attribute::Padding => 0.25,
            Attribute::CharSpacingRatio => 0.005,
            Attribute::SegmentSpacingRatio => 0.002,
            Attribute::LineSpacingRatio => 0.005,
            Attribute::CharWidth => 2.,
            Attribute::LongStrokeRatio => 0.005,
            Attribute::MidStrokeRatio => 0.005,
            Attribute::ShortStrokeRatio => 0.005,
            Attribute::StrokeWidthRatio => 0.002,
        }
    }

    pub fn next(&self) -> Self {
        unsafe {
            let u = std::mem::transmute::<Attribute, u8>(*self);
            std::mem::transmute::<u8, Attribute>((u + 1) % ATTRIBUTES.len() as u8)
        }
    }

    pub fn prev(&self) -> Self {
        unsafe {
            let u = std::mem::transmute::<Attribute, u8>(*self);
            std::mem::transmute::<u8, Attribute>(
                (u + ATTRIBUTES.len() as u8 - 1) % ATTRIBUTES.len() as u8,
            )
        }
    }
}

struct Model {
    text: wa::Text,
    page: Page,
    selected_attribute: Attribute,
}

fn model(app: &App) -> Model {
    Model {
        text: Text::random(&mut thread_rng()),
        page: Page::new(app.window_rect().w_h()),
        selected_attribute: Attribute::CharWidth,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    let shift = app.keys.down.contains(&Key::LShift);
    let control = app.keys.down.contains(&Key::LControl);

    let modifier = if shift { 2.0 } else { 1.0 } * if control { 0.5 } else { 1.0 };

    match event {
        Event::WindowEvent {
            id,
            simple: Some(we),
        } if id == app.window_id() => match we {
            Resized(new_size) => model.page.update_dimensions((new_size.x, new_size.y)),

            KeyReleased(Key::M) => model.selected_attribute = Attribute::Margin,
            KeyReleased(Key::P) => model.selected_attribute = Attribute::Padding,
            KeyReleased(Key::C) => model.selected_attribute = Attribute::CharSpacingRatio,
            KeyReleased(Key::L) => model.selected_attribute = Attribute::LineSpacingRatio,
            KeyReleased(Key::W) => model.selected_attribute = Attribute::CharWidth,
            KeyReleased(Key::R) => model.selected_attribute = Attribute::LongStrokeRatio,
            KeyReleased(Key::N) => model.selected_attribute = Attribute::MidStrokeRatio,
            KeyReleased(Key::T) => model.selected_attribute = Attribute::ShortStrokeRatio,
            KeyReleased(Key::Q) => model.selected_attribute = Attribute::StrokeWidthRatio,

            KeyPressed(Key::Up) if *DEBUG.lock().unwrap() => model
                .page
                .change_attribute(model.selected_attribute, modifier),

            KeyPressed(Key::Down) if *DEBUG.lock().unwrap() => model
                .page
                .change_attribute(model.selected_attribute, -modifier),

            KeyReleased(Key::Right) => model.selected_attribute = model.selected_attribute.next(),
            KeyReleased(Key::Left) => model.selected_attribute = model.selected_attribute.prev(),

            KeyReleased(Key::Space) => {
                if shift {
                    let text2 = Text::random(&mut thread_rng());

                    model.text.0.extend(text2.0);
                } else {
                    model.text = Text::random(&mut thread_rng());
                }
            }

            KeyReleased(Key::Tab) => {
                let mut debug = DEBUG.lock().unwrap();
                *debug = !*debug;
            }

            KeyReleased(Key::LAlt) => std::mem::swap(&mut model.page.fg, &mut model.page.bg),

            KeyReleased(Key::F) => {
                let fullscreen = app.main_window().is_fullscreen();
                app.main_window().set_fullscreen(!fullscreen);
            }
            _ => {}
        },
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

lazy_static::lazy_static! {
    static ref DEBUG: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref PARAGRAPH: Box<Paragraph> = Box::new(paragraph("ga^-xu,y-we`-pe~-ye, li~-ye`: se^-xu,y-pe~. ga^-xu,y li~-ye`."));
    static ref TEXT: Box<Text> = Box::new(text(include_str!("text.txt")));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut printer = Printer::new(&model.page);

    let draw = app.draw();

    draw.background().color(model.page.bg);

    let text = TEXT.as_ref();

    printer.print(text, &draw);

    if *DEBUG.lock().unwrap() {
        let txt = {
            let mut buffer = String::new();

            for attribute in ATTRIBUTES {
                let selected = attribute == model.selected_attribute;

                if selected {
                    buffer.push('[');
                }

                buffer.push_str(&format!(
                    "{:?}: {:.4}",
                    attribute,
                    model.page.get_attribute(attribute)
                ));

                if selected {
                    buffer.push(']');
                }

                buffer.push('\n')
            }

            buffer
        };
        let (w, h) = app.window_rect().w_h();
        draw.text(&txt)
            .color(BLUE)
            .font_size(14)
            .left_justify()
            .x_y((-w / 2.) * 0.7, (h / 2.) * 0.7)
            .w_h(200., 200.);

        draw.text(&text.to_string())
            .color(BLUE)
            .font_size(14)
            .left_justify()
            .x_y((-w / 2.) * 0.7, (-h / 2.) * 0.6)
            .w_h(200., 1000.);
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Printer<'a> {
    page: &'a Page,
    line: usize,
    position: (f32, f32),
}

impl<'a> Printer<'a> {
    pub fn new(page: &'a Page) -> Self {
        let position = (page.right_edge(), page.top_edge());
        Self {
            page,
            line: 0,
            position,
        }
    }

    pub fn print(&mut self, p: impl Print, draw: &Draw) {
        p.print(self, draw);
    }

    pub fn print_syllable(&mut self, syllable: &wa::Syllable, draw: &Draw) {
        if self.position.1 - self.page.char_height() < self.page.bottom_edge() {
            self.newline()
        }

        let mut char_centre = self.position;
        char_centre.0 -= self.page.char_width / 2.;
        char_centre.1 -= self.page.char_height() / 2.;

        if *DEBUG.lock().unwrap() {
            // debug frame

            draw.rect()
                .no_fill()
                .stroke(RED)
                .stroke_weight(self.page.stroke_width() * 0.5)
                .x_y(char_centre.0, char_centre.1)
                .w_h(self.page.char_width, self.page.char_height());
        }

        // draw onset
        let onset = &syllable.onset;
        onset.draw(self.page, char_centre, draw);

        // draw vowel
        let vowel = &syllable.vowel;
        vowel.draw(self.page, char_centre, draw);

        // draw tone
        let tone = &syllable.tone;
        tone.draw(self.page, char_centre, draw);

        // draw coda
        if let Some(coda) = &syllable.coda {
            coda.draw(self.page, char_centre, false, draw);
        } else {
            let page = &self.page;
            let y = char_centre.1
                + -1. * (0.5 * page.char_height() - 0.75 * page.short_stroke_length());

            // draw long stroke
            let x = char_centre.0;
            let (w, h) = (page.long_stroke_length(), page.stroke_width());

            draw.rect().color(page.fg).x_y(x, y).w_h(w, h);

            draw.rect()
                .color(page.fg)
                .x_y(x, y - page.short_stroke_length() / 2.0)
                .w_h(page.stroke_width(), page.short_stroke_length());
        }

        self.position.1 -= self.page.char_height() + self.page.char_spacing();
    }

    pub fn print_punctuation(&mut self, punctuation: &wa::Punctuation, draw: &Draw) {
        if self.position.1 - self.page.punctuation_height() < self.page.bottom_edge() {
            self.newline()
        }

        let mut punct_centre = self.position;
        punct_centre.0 -= self.page.char_width / 2.;
        punct_centre.1 -= self.page.punctuation_height() / 2.;

        if *DEBUG.lock().unwrap() {
            // debug frame
            draw.rect()
                .no_fill()
                .stroke(RED)
                .stroke_weight(self.page.stroke_width() * 0.5)
                .x_y(punct_centre.0, punct_centre.1)
                .w_h(self.page.char_width, self.page.punctuation_height());
        }

        punctuation.draw(self.page, punct_centre, draw);

        self.position.1 -= self.page.punctuation_height() + self.page.char_spacing();
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.position = (
            self.page.right_edge()
                - (self.page.char_width * self.page.line_spacing) * (self.line as f32),
            self.page.top_edge(),
        );
    }
}

trait Print {
    fn print(&self, printer: &mut Printer, draw: &Draw);
}

impl<T: Print> Print for &T {
    fn print(&self, printer: &mut Printer, draw: &Draw) {
        (*self).print(printer, draw);
    }
}

trait Drawable {
    fn draw(&self, page: &Page, centre: (f32, f32), draw: &Draw);
}

trait DrawableTopOrBottom {
    fn draw(&self, page: &Page, centre: (f32, f32), top: bool, draw: &Draw);
}

impl<T> Drawable for T
where
    T: DrawableTopOrBottom,
{
    fn draw(&self, page: &Page, centre: (f32, f32), draw: &Draw) {
        self.draw(page, centre, true, draw)
    }
}

/*
p horizontal long stroke with short strokes on each end (centred)
b
py

t horizontal long stroke with short strokes on each end (0.5 short stroke up)
d
ty

k horizontal long stroke with short strokes on each end (0.5 short stroke down)
g
ky

s horizontal mid stroke with short strokes on each end (centred)
z
sy

r horizontal mid stroke
rw
l

w horizontal long stroke with horizontal mid stroke under
y hotizontal mid stroke with horizontal mid stroke under

x horizontal mid stroke with short strokes on each end (0.5 short stroke down)
h horizontal mid stroke with short strokes on each end (0.5 short stroke up)

a (null)
e horizontal mid stroke with a vertical short stroke in the centre (undecided how)
i vertical mid stroke
o vertical short stroke
u vertical long stroke

high tone  2 vertical short strokes on each side
low tone   (null)
peaking    2 verrtical mid strokes on each side
nasal tone 2 vertical long strokes on each side

*/
