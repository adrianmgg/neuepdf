use cursive::{
    event,
    view::{scroll::Scroller, Nameable, Resizable, Scrollable},
    views::{self, Button, LinearLayout, Panel, TextView},
    With,
};

fn main() {
    let mut siv = cursive::default();
    let navbar = LinearLayout::horizontal()
        .child(Button::new("dashboard", |_| {}))
        .full_width()
        .with_name("navbar");
    let mut dashboard_posts = LinearLayout::vertical();
    // placeholder posts
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
    // scrolling https://github.com/gyscos/cursive/blob/53846dde64754a94b2b9df8c0c32d612ffd57510/cursive/examples/lorem.rs#L23-L42
    let dash_posts_scroll = dashboard_posts
        .scrollable()
        .wrap_with(views::OnEventView::new)
        .on_pre_event_inner('k', |v, _| {
            let scroller = v.get_scroller_mut();
            if scroller.can_scroll_up() {
                scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
            }
            Some(event::EventResult::Consumed(None))
        })
        .on_pre_event_inner('j', |v, _| {
            let scroller = v.get_scroller_mut();
            if scroller.can_scroll_down() {
                scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
            }
            Some(event::EventResult::Consumed(None))
        });
    let main_content = dash_posts_scroll;
    let document = LinearLayout::vertical()
        .child(navbar)
        .child(main_content)
        .full_screen();
    siv.add_layer(document);
    siv.run();
}
