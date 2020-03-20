use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

use structopt::StructOpt;

use cursive::{
    theme::{Color, PaletteColor, Theme},
    traits::Nameable,
    views::{Dialog, EditView, ListView},
    Cursive,
};

#[derive(StructOpt)]
pub enum Command {
    Quadratic,
}

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command,
}

pub struct AwnserPair {
    pub pair: (f32, f32),
}

impl Display for AwnserPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.pair.0, self.pair.1)
    }
}

impl AwnserPair {
    pub fn new(pair: (f32, f32)) -> Self {
        Self { pair }
    }
}

pub fn display_calculate(app: &mut Cursive, a: &str, b: &str, c: &str) {
    if a.is_empty() || b.is_empty() || c.is_empty() {
        app.add_layer(Dialog::around(Dialog::info(
            "Error: must fill out all fields!",
        )));
        return;
    }
    let content = format!("Awnser: {}", AwnserPair::new(calculate(a, b, c)));
    clear_val(app, "A");
    clear_val(app, "B");
    clear_val(app, "C");
    app.add_layer(Dialog::around(Dialog::info(content)));
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

fn clear_val(evt: &mut Cursive, label: &str) {
    evt.call_on_name(label, |view: &mut EditView| view.set_content(""))
        .unwrap();
}

pub fn default_theme(app: &Cursive) -> Theme {
    let mut theme = app.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme
}

pub fn select_command(mut app: &mut Cursive, cmd: Command) {
    match cmd {
        Command::Quadratic => build_quadratic_tui(&mut app)
    }
}

fn build_quadratic_tui(app: &mut Cursive) {
    app.add_layer(
        Dialog::new()
            .title("TUI Calculator")
            .padding_lrtb(1, 1, 1, 0)
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
            })
            .button("Quit", |s| s.quit()),
    );
}
