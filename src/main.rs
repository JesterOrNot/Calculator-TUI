use cursive::traits::Nameable;
use cursive::{
    views::{Dialog, EditView, ListView, TextView},
    Cursive,
};

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
                    .child("C", EditView::new().on_submit(|_, _| {}).with_name("C"))
            )
            .button("Ok", |mut evt: &mut Cursive| {
                let a = evt
                    .call_on_name("A", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                let b = evt
                    .call_on_name("B", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                let c = evt
                    .call_on_name("C", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                display_calculate(&mut evt, &a, &b, &c);
            }),
    );
    app.run();
}

fn display_calculate(s: &mut Cursive, a: &str, b: &str, c: &str) {
    let content = format!(
        "Awnser: {}",
        calculate(a, b, c)
    );
    s.pop_layer();
    s.add_layer(Dialog::around(TextView::new(content)).button("Quit", |s| s.quit()));
}

fn calculate(a: &str, b: &str, c: &str) -> i32 {
    parse_num(a) + parse_num(b) + parse_num(c)
}

fn parse_num(n: &str) -> i32 {
    n.parse::<i32>().unwrap()
}
