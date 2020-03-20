use cursive::{
    theme::{Color, PaletteColor, Theme},
    traits::Nameable,
    views::{Dialog, EditView, ListView, TextView},
    Cursive,
};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

struct AwnserPair {
    pub pair: (f32, f32),
}

impl Display for AwnserPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.pair.0, self.pair.1)
    }
}

impl AwnserPair {
    fn new(pair: (f32, f32)) -> Self {
        return Self { pair: pair };
    }
}

fn main() {
    let mut app = Cursive::default();
    let theme = default_theme(&app);
    app.set_theme(theme);
    app.add_layer(
        Dialog::new()
            .title("TUI Calculator")
            .content(
                ListView::new()
                    .child("A", EditView::new().on_submit(|_, _| {}).with_name("A"))
                    .delimiter()
                    .child("B", EditView::new().on_submit(|_, _| {}).with_name("B"))
                    .delimiter()
                    .child("C", EditView::new().on_submit(|_, _| {}).with_name("C")),
            )
            .button("Submit", |mut evt: &mut Cursive| {
                let a = get_val(evt, "A");
                let b = get_val(evt, "B");
                let c = get_val(evt, "C");
                display_calculate(&mut evt, &a, &b, &c);
            }),
    );
    app.run();
}

fn display_calculate(app: &mut Cursive, a: &str, b: &str, c: &str) {
    let content = format!("Awnser: {}", AwnserPair::new(calculate(a, b, c)));
    app.pop_layer();
    app.add_layer(Dialog::around(TextView::new(content)).button("Quit", |s| s.quit()));
}

fn calculate(a: &str, b: &str, c: &str) -> (f32, f32) {
    quadratic_equ(parse_num(a), parse_num(b), parse_num(c))
}

pub fn quadratic_equ(a: f32, b: f32, c: f32) -> (f32, f32) {
    (
        (-b + f32::sqrt(b.powf(2.0) - 4.0 * a * c)) / 2.0 * a,
        (-b - f32::sqrt(b.powf(2.0) - 4.0 * a * c)) / 2.0 * a,
    )
}

fn parse_num(n: &str) -> f32 {
    n.parse::<f32>().unwrap()
}

fn get_val(evt: &mut Cursive, label: &str) -> Rc<String> {
    evt.call_on_name(label, |view: &mut EditView| view.get_content())
        .unwrap()
}

fn default_theme(app: &Cursive) -> Theme {
    let mut theme = app.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme
}
