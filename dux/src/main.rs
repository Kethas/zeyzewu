fn main() {
    nannou_backend::start()
}

mod nannou_backend {
    use nannou::{rand::thread_rng, color::IntoLinSrgba};
    use wa::Random;

    struct Model {
        text: wa::Text,
        page: dux::Page,
        debug: bool,
    }

    pub fn start() {
        nannou::app(|app| Model {
            text: wa::text("ga^-xu,y-ze~ xu,y-ye` li~"),
            page: dux::Page::new(app.window_rect().w_h()),
            debug: false,
        })
        .simple_window(view)
        .event(event)
        .run()
    }

    fn event(app: &nannou::App, model: &mut Model, event: nannou::Event) {
        match event {
            nannou::Event::WindowEvent {
                id,
                simple: Some(event),
            } if id == app.window_id() => match event {
                nannou::event::WindowEvent::Resized(new_size) => {
                    model.page.update_dimensions((new_size.x, new_size.y))
                }
                nannou::event::WindowEvent::KeyPressed(key) => match key {
                    nannou::event::Key::Space
                        if app.keys.down.contains(&nannou::event::Key::LShift) =>
                    {
                        let new_text = wa::Text::random(&mut thread_rng());
                        println!("+{new_text}");
                        model.text.extend(new_text);
                    }
                    nannou::event::Key::Space => {
                        model.text = wa::Text::random(&mut thread_rng());
                        println!("{}", model.text);
                    }
                    nannou::event::Key::Tab => model.debug = !model.debug,
                    _ => {}
                },
                _ => {}
            },

            _ => {}
        }
    }

    fn view(app: &nannou::App, model: &Model, frame: nannou::Frame) {
        frame.clear(nannou::color::WHITE);

        let draw = app.draw();

        use nannou::color::named::*;
        let colors: &[nannou::color::Srgb<u8>] = if !model.debug {
            &[BLACK]
        } else {
            &[BLACK, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA, GRAY]
        };
        let mut colors_i = 0;

        let mut draw_rect = |rect: dux::Rect| {
            let x = rect.x + rect.w / 2.0;
            let y = rect.y - rect.h / 2.0;
            let w = rect.w;
            let h = rect.h;

            let mut color = colors[colors_i].into_lin_srgba();
            colors_i = (colors_i + 1) % colors.len();

            if model.debug {
                color.alpha = 0.90;
            }

            draw.rect().x(x).y(y).w(w).h(h).color(color);
        };

        if model.debug {
            let mut printer = dux::Printer::new(&model.page);

            let mut grapheme_debug = |printer: &mut dux::Printer, grapheme: &dux::Grapheme| {
                let pos = printer.get_position();
                let rect = dux::Rect {
                    x: pos.0 - printer.page.char_width(),
                    y: pos.1,
                    w: printer.page.char_width(),
                    h: if grapheme.height == dux::GraphemeHeight::Character {
                        printer.page.char_height
                    } else {
                        printer.page.punctuation_height()
                    },
                };
                let x = rect.x + rect.w / 2.0;
                let y = rect.y - rect.h / 2.0;
                let w = rect.w;
                let h = rect.h;

                draw.rect()
                    .x(x)
                    .y(y)
                    .w(w)
                    .h(h)
                    .stroke(RED)
                    .no_fill()
                    .stroke_weight(1.);
            };

            printer.grapheme_debug = Some(&mut grapheme_debug);

            let w = -printer.page.left_edge() + printer.page.right_edge();
            let h = -printer.page.top_edge() + printer.page.bottom_edge();

            draw.rect()
                .x(0.)
                .y(0.)
                .w(w)
                .h(h)
                .stroke(nannou::color::BLUE)
                .no_fill()
                .stroke_weight(1.);

            printer.print(&model.text, &mut draw_rect);
        } else {
            let mut printer = dux::Printer::new(&model.page);
            printer.print(&model.text, &mut draw_rect);
        }

        draw.to_frame(app, &frame).unwrap();
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



a 2 vertical short strokes - spaced 1 short stroke
e horizontal mid stroke with a vertical short stroke in the centre (undecided how)
i vertical mid stroke
o vertical short stroke
u vertical long stroke


all spaced 1 mid stroke
high tone  2 vertical short strokes on each side
low tone   (null)
peaking    2 verrtical mid strokes on each side
nasal tone 2 vertical long strokes on each side

*/
