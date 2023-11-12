#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Stroke {
    Nil,

    P,
    B,
    Py,
    T,
    D,
    Ty,
    K,
    G,
    Ky,
    S,
    Z,
    Sy,
    R,
    Rw,
    L,

    WTop,
    WBottom,
    YTop,
    YBottom,
    XTop,
    XBottom,
    HTop,
    HBottom,

    NullCoda,

    A,
    E,
    I,
    O,
    U,

    HighTone,
    PeakingTone,
    NasalTone,

    WordBreak,
    PhraseBreak,
    SentenceBreak,
}

pub trait ToStroke {
    fn to_stroke(&self) -> Stroke;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub trait ToRects {
    fn to_rects(&self, printer: &Printer) -> Vec<Rect>;
}

impl ToRects for Stroke {
    // This is a monstrosity of a function.
    // I will have to refactor this one day
    fn to_rects(&self, printer: &Printer) -> Vec<Rect> {
        let top_right = printer.get_position();
        let middle_x = top_right.0 - printer.page.char_width() * 0.5;
        let middle_y = top_right.1 - printer.page.char_height * 0.5;

        let short_stroke_minus_width =
            printer.page.short_stroke_length() - printer.page.stroke_width();

        let top_long_stroke_rect = Rect {
            x: top_right.0 - printer.page.long_stroke_length(),
            y: top_right.1 - short_stroke_minus_width,
            w: printer.page.long_stroke_length(),
            h: printer.page.stroke_width(),
        };

        let top_mid_stroke_rect = Rect {
            x: top_right.0
                - 0.5 * printer.page.long_stroke_length()
                - 0.5 * printer.page.mid_stroke_length(),
            y: top_right.1 - short_stroke_minus_width,
            w: printer.page.mid_stroke_length(),
            h: printer.page.stroke_width(),
        };

        let bottom_mid_stroke_rect = Rect {
            x: top_mid_stroke_rect.x,
            y: top_right.1 - printer.page.char_height + printer.page.short_stroke_length(),
            w: top_mid_stroke_rect.w,
            h: top_mid_stroke_rect.h,
        };

        let blunt_stroke_rect = Rect {
            x: top_right.0
                - printer.page.long_stroke_length() / 2.0
                - printer.page.stroke_width() / 2.0,
            y: top_right.1,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let sharp_stroke_rect_long_stem_l = Rect {
            x: middle_x - printer.page.mid_stroke_length() * 0.5,
            y: top_right.1,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let sharp_stroke_rect_long_stem_r = Rect {
            x: middle_x + printer.page.mid_stroke_length() * 0.5 - printer.page.stroke_width(),
            y: top_right.1,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let sharp_stroke_rect_mid_stem_l = Rect {
            x: top_right.0 - printer.page.long_stroke_length()
                + 0.5 * (printer.page.long_stroke_length() - printer.page.mid_stroke_length())
                - 0.5 * printer.page.stroke_width()
                + printer.page.mid_stroke_length() / 3.0,
            y: top_right.1,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let sharp_stroke_rect_mid_stem_r = Rect {
            x: sharp_stroke_rect_mid_stem_l.x + printer.page.mid_stroke_length() / 3.0,
            y: top_right.1,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let p_wing_l_rect = Rect {
            x: top_right.0 - printer.page.long_stroke_length(),
            y: top_long_stroke_rect.y
                + 0.5 * (printer.page.short_stroke_length() - printer.page.stroke_width()),
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let p_wing_r_rect = Rect {
            x: top_right.0 - printer.page.stroke_width(),
            y: top_long_stroke_rect.y
                + 0.5 * (printer.page.short_stroke_length() - printer.page.stroke_width()),
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let t_wing_l_rect = Rect {
            x: top_right.0 - printer.page.long_stroke_length(),
            y: top_right.1,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let t_wing_r_rect = Rect {
            x: top_right.0 - printer.page.stroke_width(),
            y: top_right.1,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let k_wing_l_rect = Rect {
            x: top_right.0 - printer.page.long_stroke_length(),
            y: top_long_stroke_rect.y,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let k_wing_r_rect = Rect {
            x: top_right.0 - printer.page.stroke_width(),
            y: top_long_stroke_rect.y,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let s_wing_l_rect = Rect {
            x: top_mid_stroke_rect.x,
            y: top_mid_stroke_rect.y
                + 0.5 * (printer.page.short_stroke_length() - printer.page.stroke_width()),
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let s_wing_r_rect = Rect {
            x: top_mid_stroke_rect.x + top_mid_stroke_rect.w - printer.page.stroke_width(),
            y: top_mid_stroke_rect.y
                + 0.5 * (printer.page.short_stroke_length() - printer.page.stroke_width()),
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let x_top_wing_l_rect = Rect {
            x: top_mid_stroke_rect.x,
            y: top_mid_stroke_rect.y,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let x_top_wing_r_rect = Rect {
            x: top_mid_stroke_rect.x + top_mid_stroke_rect.w - printer.page.stroke_width(),
            y: top_mid_stroke_rect.y,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let x_bottom_wing_l_rect = Rect {
            x: x_top_wing_l_rect.x,
            y: bottom_mid_stroke_rect.y,
            w: x_top_wing_l_rect.w,
            h: x_top_wing_l_rect.h,
        };

        let x_bottom_wing_r_rect = Rect {
            x: x_top_wing_r_rect.x,
            y: bottom_mid_stroke_rect.y,
            w: x_top_wing_r_rect.w,
            h: x_top_wing_r_rect.h,
        };

        let h_top_wing_l_rect = Rect {
            x: top_mid_stroke_rect.x + top_mid_stroke_rect.w - printer.page.stroke_width(),
            y: top_right.1,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let h_top_wing_r_rect = Rect {
            x: top_mid_stroke_rect.x,
            y: top_right.1,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let h_bottom_wing_l_rect = Rect {
            x: h_top_wing_l_rect.x,
            y: bottom_mid_stroke_rect.y,
            w: h_top_wing_l_rect.w,
            h: h_top_wing_l_rect.h,
        };

        let h_bottom_wing_r_rect = Rect {
            x: h_top_wing_r_rect.x,
            y: bottom_mid_stroke_rect.y,
            w: h_top_wing_r_rect.w,
            h: h_top_wing_r_rect.h,
        };

        let top_w_top = Rect {
            x: top_long_stroke_rect.x,
            y: top_right.1,
            w: printer.page.long_stroke_length(),
            h: printer.page.stroke_width(),
        };

        let top_w_bottom = Rect {
            x: top_mid_stroke_rect.x,
            y: top_w_top.y - printer.page.stroke_width() * 2.0,
            w: printer.page.mid_stroke_length(),
            h: printer.page.stroke_width(),
        };

        let bottom_w_bottom = Rect {
            x: top_w_bottom.x,
            y: top_right.1 - printer.page.char_height + printer.page.stroke_width(),
            w: top_w_bottom.w,
            h: top_w_bottom.h,
        };

        let bottom_w_top = Rect {
            x: top_w_top.x,
            y: bottom_w_bottom.y + printer.page.stroke_width() * 2.0,
            w: top_w_top.w,
            h: top_w_top.h,
        };

        let top_y_top = Rect {
            x: top_mid_stroke_rect.x,
            y: top_right.1,
            w: printer.page.mid_stroke_length(),
            h: printer.page.stroke_width(),
        };

        let bottom_y_top = Rect {
            x: top_y_top.x,
            y: bottom_w_top.y,
            w: top_y_top.w,
            h: top_y_top.h,
        };

        let top_y_bottom = top_w_bottom;
        let bottom_y_bottom = bottom_w_bottom;

        let null_coda_wing_rect = Rect {
            x: blunt_stroke_rect.x,
            y: top_right.1 - printer.page.char_height + printer.page.short_stroke_length(),
            w: blunt_stroke_rect.w,
            h: blunt_stroke_rect.h,
        };

        let a_l_rect = Rect {
            x: middle_x - printer.page.short_stroke_length() * 0.5,
            y: middle_y + printer.page.short_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let a_r_rect = Rect {
            x: middle_x + printer.page.short_stroke_length() * 0.5 - printer.page.stroke_width(),
            y: middle_y + printer.page.short_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let e_mid_stroke_rect = Rect {
            x: middle_x - printer.page.mid_stroke_length() * 0.5,
            y: middle_y + printer.page.stroke_width() * 0.5,
            w: printer.page.mid_stroke_length(),
            h: printer.page.stroke_width(),
        };

        let e_short_stroke_rect = Rect {
            x: middle_x - printer.page.stroke_width() * 0.5,
            y: middle_y + printer.page.stroke_width() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let i_stroke_rect = Rect {
            x: middle_x - printer.page.stroke_width() * 0.5,
            y: middle_y + printer.page.mid_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.mid_stroke_length(),
        };

        let o_stroke_rect = Rect {
            x: middle_x - printer.page.stroke_width() * 0.5,
            y: middle_y + printer.page.short_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let u_stroke_rect = Rect {
            x: middle_x - printer.page.stroke_width() * 0.5,
            y: middle_y + printer.page.long_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.long_stroke_length(),
        };

        let high_l_rect = Rect {
            x: middle_x - printer.page.mid_stroke_length() * 0.5,
            y: middle_y + printer.page.short_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let high_r_rect = Rect {
            x: middle_x + printer.page.mid_stroke_length() * 0.5 - printer.page.stroke_width(),
            y: middle_y + printer.page.short_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let peaking_l_rect = Rect {
            x: middle_x - printer.page.mid_stroke_length() * 0.5,
            y: middle_y + printer.page.mid_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.mid_stroke_length(),
        };

        let peaking_r_rect = Rect {
            x: middle_x + printer.page.mid_stroke_length() * 0.5 - printer.page.stroke_width(),
            y: middle_y + printer.page.mid_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.mid_stroke_length(),
        };

        let nasal_l_rect = Rect {
            x: middle_x - printer.page.mid_stroke_length() * 0.5,
            y: middle_y + printer.page.long_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.long_stroke_length(),
        };

        let nasal_r_rect = Rect {
            x: middle_x + printer.page.mid_stroke_length() * 0.5 - printer.page.stroke_width(),
            y: middle_y + printer.page.long_stroke_length() * 0.5,
            w: printer.page.stroke_width(),
            h: printer.page.long_stroke_length(),
        };

        let word_break_rect = Rect {
            x: top_right.0
                - printer.page.char_width() / 2.0
                - printer.page.short_stroke_length() / 2.0,
            y: top_right.1 - printer.page.punctuation_height() / 2.0
                + printer.page.stroke_width() / 2.0,
            w: printer.page.short_stroke_length(),
            h: printer.page.stroke_width(),
        };

        let phrase_break_top = Rect {
            x: word_break_rect.x,
            y: top_right.1,
            w: printer.page.short_stroke_length(),
            h: printer.page.stroke_width(),
        };

        let phrase_break_bottom = Rect {
            x: word_break_rect.x,
            y: top_right.1 - printer.page.punctuation_height() + printer.page.stroke_width(),
            w: printer.page.short_stroke_length(),
            h: printer.page.stroke_width(),
        };

        let sentence_break_l = Rect {
            x: word_break_rect.x,
            y: word_break_rect.y
                + (printer.page.short_stroke_length() / 2.0 - printer.page.stroke_width() / 2.0),
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        let sentence_break_r = Rect {
            x: word_break_rect.x + printer.page.short_stroke_length() - printer.page.stroke_width(),
            y: sentence_break_l.y,
            w: printer.page.stroke_width(),
            h: printer.page.short_stroke_length(),
        };

        match self {
            Stroke::Nil => vec![],
            Stroke::P => vec![top_long_stroke_rect, p_wing_l_rect, p_wing_r_rect],
            Stroke::B => vec![
                top_long_stroke_rect,
                p_wing_l_rect,
                p_wing_r_rect,
                blunt_stroke_rect,
            ],
            Stroke::Py => vec![
                top_long_stroke_rect,
                p_wing_l_rect,
                p_wing_r_rect,
                sharp_stroke_rect_long_stem_l,
                sharp_stroke_rect_long_stem_r,
            ],
            Stroke::T => vec![top_long_stroke_rect, t_wing_l_rect, t_wing_r_rect],
            Stroke::D => vec![
                top_long_stroke_rect,
                t_wing_l_rect,
                t_wing_r_rect,
                blunt_stroke_rect,
            ],
            Stroke::Ty => vec![
                top_long_stroke_rect,
                t_wing_l_rect,
                t_wing_r_rect,
                sharp_stroke_rect_long_stem_l,
                sharp_stroke_rect_long_stem_r,
            ],
            Stroke::K => vec![top_long_stroke_rect, k_wing_l_rect, k_wing_r_rect],
            Stroke::G => vec![
                top_long_stroke_rect,
                k_wing_l_rect,
                k_wing_r_rect,
                blunt_stroke_rect,
            ],
            Stroke::Ky => vec![
                top_long_stroke_rect,
                k_wing_l_rect,
                k_wing_r_rect,
                sharp_stroke_rect_long_stem_l,
                sharp_stroke_rect_long_stem_r,
            ],
            Stroke::S => vec![top_mid_stroke_rect, s_wing_l_rect, s_wing_r_rect],
            Stroke::Z => vec![
                top_mid_stroke_rect,
                s_wing_l_rect,
                s_wing_r_rect,
                blunt_stroke_rect,
            ],
            Stroke::Sy => vec![
                top_mid_stroke_rect,
                s_wing_l_rect,
                s_wing_r_rect,
                sharp_stroke_rect_mid_stem_l,
                sharp_stroke_rect_mid_stem_r,
            ],
            Stroke::R => vec![top_mid_stroke_rect],
            Stroke::Rw => vec![top_mid_stroke_rect, blunt_stroke_rect],
            Stroke::L => vec![
                top_mid_stroke_rect,
                sharp_stroke_rect_mid_stem_l,
                sharp_stroke_rect_mid_stem_r,
            ],
            Stroke::WTop => vec![top_w_top, top_w_bottom],
            Stroke::WBottom => vec![bottom_w_top, bottom_w_bottom],
            Stroke::YTop => vec![top_y_top, top_y_bottom],
            Stroke::YBottom => vec![bottom_y_top, bottom_y_bottom],
            Stroke::XTop => vec![top_mid_stroke_rect, x_top_wing_l_rect, x_top_wing_r_rect],
            Stroke::XBottom => vec![
                bottom_mid_stroke_rect,
                x_bottom_wing_l_rect,
                x_bottom_wing_r_rect,
            ],
            Stroke::HTop => vec![top_mid_stroke_rect, h_top_wing_l_rect, h_top_wing_r_rect],
            Stroke::HBottom => vec![
                bottom_mid_stroke_rect,
                h_bottom_wing_l_rect,
                h_bottom_wing_r_rect,
            ],

            Stroke::NullCoda => vec![bottom_w_top, null_coda_wing_rect],

            Stroke::A => vec![a_l_rect, a_r_rect],
            Stroke::E => vec![e_mid_stroke_rect, e_short_stroke_rect],
            Stroke::I => vec![i_stroke_rect],
            Stroke::O => vec![o_stroke_rect],
            Stroke::U => vec![u_stroke_rect],

            Stroke::HighTone => vec![high_l_rect, high_r_rect],
            Stroke::PeakingTone => vec![peaking_l_rect, peaking_r_rect],
            Stroke::NasalTone => vec![nasal_l_rect, nasal_r_rect],

            Stroke::WordBreak => vec![word_break_rect],
            Stroke::PhraseBreak => vec![phrase_break_top, phrase_break_bottom],
            Stroke::SentenceBreak => vec![word_break_rect, sentence_break_l, sentence_break_r],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GraphemeHeight {
    Character,
    Punctuation,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grapheme {
    pub strokes: Vec<Stroke>,
    pub height: GraphemeHeight,
}

pub trait ToGrapheme {
    fn to_grapheme(&self) -> Grapheme;
}

impl ToGrapheme for Grapheme {
    fn to_grapheme(&self) -> Grapheme {
        self.clone()
    }
}

pub trait ToGraphemes {
    fn to_graphemes(&self) -> Vec<Grapheme>;
}

impl ToGraphemes for Vec<Grapheme> {
    fn to_graphemes(&self) -> Vec<Grapheme> {
        self.clone()
    }
}

pub struct Page {
    // the dimensions of the page
    pub dimensions: (f32, f32),

    // the margins on the top and bottom edges of the page, in multiples of page height
    pub margins_top_bottom: f32,
    // the margins on the left and right edges of the page, in multiples of page width
    pub margins_left_right: f32,
    // the spacing between characters (multiples of char_height)
    pub char_spacing_ratio: f32,
    // the spacing between lines (multiples of char_width)
    pub line_spacing: f32,
    // the total height of a character 2 * (short stroke - stroke width) + long stroke
    pub char_height: f32,
    // the ratio between long_stroke_width and mid_stroke_length
    pub mid_stroke_ratio: f32,
    // the ratio between long_stroke_width and short_stroke_length
    pub short_stroke_ratio: f32,
    // the ratio between long_stroke_width and stroke_width
    pub stroke_width_ratio: f32,
}

impl Page {
    pub fn new(dimensions: (f32, f32)) -> Self {
        Self {
            dimensions,
            margins_top_bottom: 0.02,
            margins_left_right: 0.02,
            char_spacing_ratio: 0.065,
            line_spacing: 0.1,
            char_height: 75.,
            mid_stroke_ratio: 0.5,
            short_stroke_ratio: 0.2,
            stroke_width_ratio: 0.02 / 0.3, // this one is 0.6666... so maybe it's better to keep it like this
        }
    }

    pub fn update_dimensions(&mut self, dimensions: (f32, f32)) {
        self.dimensions = dimensions
    }

    pub fn change_attribute(&mut self, attribute: Attribute, modifier: f32) {
        let amount = modifier * attribute.change_amount();

        match attribute {
            Attribute::MarginsTopBottom => self.margins_top_bottom += amount,
            Attribute::MarginsLeftRight => self.margins_left_right += amount,
            Attribute::CharSpacingRatio => self.char_spacing_ratio += amount,
            Attribute::LineSpacingRatio => self.line_spacing += amount,
            Attribute::CharHeight => self.char_height += amount,
            Attribute::MidStrokeRatio => self.mid_stroke_ratio += amount,
            Attribute::ShortStrokeRatio => self.short_stroke_ratio += amount,
            Attribute::StrokeWidthRatio => self.stroke_width_ratio += amount,
        }
    }

    pub fn get_attribute(&self, attribute: Attribute) -> f32 {
        match attribute {
            Attribute::MarginsTopBottom => self.margins_top_bottom,
            Attribute::MarginsLeftRight => self.margins_left_right,
            Attribute::CharSpacingRatio => self.char_spacing_ratio,
            Attribute::LineSpacingRatio => self.line_spacing,
            Attribute::CharHeight => self.char_height,
            Attribute::MidStrokeRatio => self.mid_stroke_ratio,
            Attribute::ShortStrokeRatio => self.short_stroke_ratio,
            Attribute::StrokeWidthRatio => self.stroke_width_ratio,
        }
    }

    pub fn char_spacing(&self) -> f32 {
        self.char_spacing_ratio * self.char_height
    }

    pub fn stroke_width(&self) -> f32 {
        self.long_stroke_length() * self.stroke_width_ratio
    }

    fn long_stroke_length(&self) -> f32 {
        // char height = 2 * (short_stroke - stroke_width) * char_height + long_stroke * char_height
        // char_height - long_stroke * char_height = 2 * (short_stroke - stroke_width) * char_height
        // - long_stroke * char_height = 2 * (short_stroke - stroke_width) * char_height - char_height
        // long_stroke * char_height = -2 * (short_stroke - stroke_width) * char_height + char_height
        // long_stroke = 1 - 2 * (short_stroke - stroke_width)
        let long_stroke_ratio = 1.0 - 2.0 * (self.short_stroke_ratio - self.stroke_width_ratio);

        self.char_height * long_stroke_ratio
    }

    pub fn mid_stroke_length(&self) -> f32 {
        self.long_stroke_length() * self.mid_stroke_ratio
    }

    pub fn short_stroke_length(&self) -> f32 {
        self.long_stroke_length() * self.short_stroke_ratio
    }

    // with the margin applied
    pub fn right_edge(&self) -> f32 {
        let margin = self.dimensions.0 * self.margins_left_right;
        (self.dimensions.0) / 2. - margin
    }
    pub fn top_edge(&self) -> f32 {
        let margin = self.dimensions.1 * self.margins_top_bottom;
        (self.dimensions.1) / 2. - margin
    }
    pub fn bottom_edge(&self) -> f32 {
        let margin = self.dimensions.1 * self.margins_top_bottom;
        -(self.dimensions.1) / 2. + margin
    }
    pub fn left_edge(&self) -> f32 {
        let margin = self.dimensions.0 * self.margins_left_right;
        -(self.dimensions.0) / 2. + margin
    }

    pub fn char_width(&self) -> f32 {
        self.long_stroke_length()
    }

    pub fn punctuation_height(&self) -> f32 {
        self.short_stroke_length()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attribute {
    MarginsTopBottom,
    MarginsLeftRight,
    CharSpacingRatio,
    LineSpacingRatio,
    CharHeight,
    MidStrokeRatio,
    ShortStrokeRatio,
    StrokeWidthRatio,
}

pub const ATTRIBUTES: &[Attribute] = &[
    Attribute::MarginsTopBottom,
    Attribute::MarginsLeftRight,
    Attribute::CharSpacingRatio,
    Attribute::LineSpacingRatio,
    Attribute::CharHeight,
    Attribute::MidStrokeRatio,
    Attribute::ShortStrokeRatio,
    Attribute::StrokeWidthRatio,
];

impl Attribute {
    pub fn change_amount(&self) -> f32 {
        match self {
            Attribute::MarginsTopBottom => 0.005,
            Attribute::MarginsLeftRight => 0.005,
            Attribute::CharSpacingRatio => 0.005,
            Attribute::LineSpacingRatio => 0.005,
            Attribute::CharHeight => 2.,
            Attribute::MidStrokeRatio => 0.0025,
            Attribute::ShortStrokeRatio => 0.0025,
            Attribute::StrokeWidthRatio => 0.001,
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

pub struct Printer<'a> {
    pub page: &'a Page,
    line: usize,
    chars: usize,
    puncts: usize,

    pub grapheme_debug: Option<&'a mut dyn FnMut(&mut Printer, &Grapheme)>,
}

impl<'a> Printer<'a> {
    pub fn new(page: &'a Page) -> Self {
        Self {
            page,
            line: 0,
            chars: 0,
            puncts: 0,

            grapheme_debug: None,
        }
    }

    pub fn print_grapheme(&mut self, grapheme: &Grapheme, draw_rect: &mut impl FnMut(Rect)) {
        let grapheme_height = match grapheme.height {
            GraphemeHeight::Character => self.page.char_height,
            GraphemeHeight::Punctuation => self.page.punctuation_height(),
        };

        if self.get_position().1 - grapheme_height < self.page.bottom_edge() {
            self.newline();
        }

        if let Some(grapheme_debug) = self.grapheme_debug.take() {
            grapheme_debug(self, grapheme);

            self.grapheme_debug = Some(grapheme_debug);
        }

        let rects = grapheme.strokes.iter().flat_map(|s| s.to_rects(self));

        // draw rects
        rects.for_each(draw_rect);

        // advance printer
        match grapheme.height {
            GraphemeHeight::Character => self.increment_char(),
            GraphemeHeight::Punctuation => self.increment_punctuation(),
        }
    }

    pub fn print(&mut self, p: &impl Print, draw_rect: &mut impl FnMut(Rect)) {
        p.print(self, draw_rect);
    }

    pub fn increment_char(&mut self) {
        if self.get_position().1 - self.page.char_height < self.page.bottom_edge() {
            self.newline();
        } else {
            self.chars += 1;
        }
    }

    pub fn increment_punctuation(&mut self) {
        if self.get_position().1 - self.page.punctuation_height() < self.page.bottom_edge() {
            self.newline();
        } else {
            self.puncts += 1;
        }
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.chars = 0;
        self.puncts = 0;
    }

    // returns the top right corner of the current character
    pub fn get_position(&self) -> (f32, f32) {
        (
            self.page.right_edge()
                - (self.page.char_width() * (1.0 + self.page.line_spacing)) * (self.line as f32),
            self.page.top_edge()
                - (self.page.char_height * self.chars as f32
                    + self.page.punctuation_height() * self.puncts as f32
                    + self.page.char_spacing() * (self.puncts + self.chars) as f32),
        )
    }
}

pub trait Print {
    fn print(&self, printer: &mut Printer, draw_rect: &mut impl FnMut(Rect));
}

impl<T> Print for T
where
    T: ToGraphemes,
{
    fn print(&self, printer: &mut Printer, draw_rect: &mut impl FnMut(Rect)) {
        self.to_graphemes()
            .iter()
            .for_each(|g| printer.print_grapheme(g, draw_rect));
    }
}

mod strokes {
    use itertools::Itertools;
    use wa::Punctuation;

    use super::*;

    impl ToStroke for wa::PureC {
        fn to_stroke(&self) -> Stroke {
            match self {
                wa::PureC::Strong(c) => match c {
                    wa::CStem::P => Stroke::P,
                    wa::CStem::T => Stroke::T,
                    wa::CStem::K => Stroke::K,
                    wa::CStem::S => Stroke::S,
                    wa::CStem::R => Stroke::R,
                },
                wa::PureC::Blunt(c) => match c {
                    wa::CStem::P => Stroke::B,
                    wa::CStem::T => Stroke::D,
                    wa::CStem::K => Stroke::G,
                    wa::CStem::S => Stroke::Z,
                    wa::CStem::R => Stroke::Rw,
                },
                wa::PureC::Sharp(c) => match c {
                    wa::CStem::P => Stroke::Py,
                    wa::CStem::T => Stroke::Ty,
                    wa::CStem::K => Stroke::Ky,
                    wa::CStem::S => Stroke::Sy,
                    wa::CStem::R => Stroke::L,
                },
            }
        }
    }

    impl ToStroke for wa::H {
        fn to_stroke(&self) -> Stroke {
            match self {
                wa::H::W => Stroke::WTop,
                wa::H::Y => Stroke::YTop,
                wa::H::X => Stroke::XTop,
                wa::H::H => Stroke::HTop,
            }
        }
    }

    struct Coda(Option<wa::H>);

    impl ToStroke for Coda {
        fn to_stroke(&self) -> Stroke {
            match self.0 {
                Some(h) => match h {
                    wa::H::W => Stroke::WBottom,
                    wa::H::Y => Stroke::YBottom,
                    wa::H::X => Stroke::XBottom,
                    wa::H::H => Stroke::HBottom,
                },
                None => Stroke::NullCoda,
            }
        }
    }

    impl ToStroke for wa::C {
        fn to_stroke(&self) -> Stroke {
            match self {
                wa::C::H(h) => h.to_stroke(),
                wa::C::C(c) => c.to_stroke(),
            }
        }
    }

    impl ToStroke for wa::V {
        fn to_stroke(&self) -> Stroke {
            match self {
                wa::V::A => Stroke::A,
                wa::V::E => Stroke::E,
                wa::V::I => Stroke::I,
                wa::V::O => Stroke::O,
                wa::V::U => Stroke::U,
            }
        }
    }

    impl ToStroke for wa::T {
        fn to_stroke(&self) -> Stroke {
            match self {
                wa::T::High => Stroke::HighTone,
                wa::T::Low => Stroke::Nil,
                wa::T::Peaking => Stroke::PeakingTone,
                wa::T::Nasal => Stroke::NasalTone,
            }
        }
    }

    impl ToGrapheme for wa::Syllable {
        fn to_grapheme(&self) -> Grapheme {
            let onset = self.onset.to_stroke();
            let vowel = self.vowel.to_stroke();
            let tone = self.tone.to_stroke();
            let coda = Coda(self.coda).to_stroke();

            let strokes = if tone == Stroke::Nil {
                vec![onset, vowel, coda]
            } else {
                vec![onset, vowel, tone, coda]
            };

            Grapheme {
                strokes,
                height: GraphemeHeight::Character,
            }
        }
    }

    impl ToStroke for wa::Punctuation {
        fn to_stroke(&self) -> Stroke {
            match self {
                wa::Punctuation::WordBreak => Stroke::WordBreak,
                wa::Punctuation::PhraseBreak => Stroke::PhraseBreak,
                wa::Punctuation::SentenceBreak => Stroke::SentenceBreak,
            }
        }
    }

    impl ToGrapheme for wa::Punctuation {
        fn to_grapheme(&self) -> Grapheme {
            let stroke = self.to_stroke();

            Grapheme {
                strokes: vec![stroke],
                height: GraphemeHeight::Punctuation,
            }
        }
    }

    impl<T> ToGraphemes for T
    where
        T: ToGrapheme,
    {
        fn to_graphemes(&self) -> Vec<Grapheme> {
            vec![self.to_grapheme()]
        }
    }

    impl ToGraphemes for wa::Word {
        fn to_graphemes(&self) -> Vec<Grapheme> {
            self.0.iter().map(ToGrapheme::to_grapheme).collect()
        }
    }

    impl ToGraphemes for wa::Phrase {
        fn to_graphemes(&self) -> Vec<Grapheme> {
            self.0
                .iter()
                .map(ToGraphemes::to_graphemes)
                .intersperse_with(|| vec![Punctuation::WordBreak.to_grapheme()])
                .flatten()
                .collect()
        }
    }

    impl ToGraphemes for wa::Sentence {
        fn to_graphemes(&self) -> Vec<Grapheme> {
            self.0
                .iter()
                .map(ToGraphemes::to_graphemes)
                .intersperse_with(|| vec![Punctuation::PhraseBreak.to_grapheme()])
                .flatten()
                .collect()
        }
    }

    impl ToGraphemes for wa::Paragraph {
        fn to_graphemes(&self) -> Vec<Grapheme> {
            self.0
                .iter()
                .map(ToGraphemes::to_graphemes)
                .intersperse_with(|| vec![Punctuation::SentenceBreak.to_grapheme()])
                .flatten()
                .collect()
        }
    }

    impl Print for wa::Text {
        fn print(&self, printer: &mut Printer, draw_rect: &mut impl FnMut(Rect)) {
            self.0.iter().map(ToGraphemes::to_graphemes).for_each(|gs| {
                gs.iter().for_each(|g| printer.print(g, draw_rect));
                printer.newline();
            })
        }
    }
}
