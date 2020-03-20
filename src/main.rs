use cursive::{
    traits::Nameable,
    views::{Dialog, EditView, ListView, TextView},
    Cursive,
};
use std::rc::Rc;

fn main() {
    let mut app = Cursive::default();
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
    let content = format!("Awnser: {}", calculate(a, b, c));
    app.pop_layer();
    app.add_layer(Dialog::around(TextView::new(content)).button("Quit", |s| s.quit()));
}

fn calculate(a: &str, b: &str, c: &str) -> i32 {
    parse_num(a) + parse_num(b) + parse_num(c)
}

fn parse_num(n: &str) -> i32 {
    n.parse::<i32>().unwrap()
}

fn get_val(evt: &mut Cursive, label: &str) -> Rc<String> {
    evt.call_on_name(label, |view: &mut EditView| view.get_content())
        .unwrap()
}
