mod lib;

use cursive::{
    traits::Nameable,
    views::{Dialog, EditView, ListView},
    Cursive,
};
use lib::*;

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
