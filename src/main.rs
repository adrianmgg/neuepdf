use cursive::{
    view::{Nameable, Resizable},
    views::{Button, LinearLayout, Panel, ScrollView, TextView},
    Cursive,
};

fn switch_to_dashboard(s: &mut Cursive) {
    s.call_on_name("navbar", |bar: &mut LinearLayout| {
        bar.add_child(TextView::new("(dash btn clicked)"))
    });
}

fn main() {
    let mut siv = cursive::default();
    let navbar = LinearLayout::horizontal()
        .child(Button::new("dashboard", switch_to_dashboard))
        .full_width()
        .with_name("navbar");
    let mut dashboard_posts = LinearLayout::vertical();
    for i in 1..32 {
        let post = Panel::new(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(TextView::new("image-wizard"))
                        .child(TextView::new(" reblogged from "))
                        .child(TextView::new("im-mage")),
                )
                .child(TextView::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."))
                .child(TextView::new(format!("{}", i).repeat(128)))
        );
        dashboard_posts.add_child(post);
    }
    let main_content = ScrollView::new(dashboard_posts);
    let document = LinearLayout::vertical()
        .child(navbar)
        .child(main_content)
        .full_screen();
    siv.add_layer(document);
    siv.run();
}
