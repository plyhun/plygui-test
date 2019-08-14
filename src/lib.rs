use plygui::*;

use std::fs::*;
use std::io::BufReader;
use std::sync::{Arc, RwLock};
use std::borrow::Cow;

fn create_image() -> Box<dyn Control> {
    let img = external::image::load(BufReader::new(File::open("resources/lulz.png").unwrap()), external::image::PNG).unwrap();

    let mut i = imp::Image::with_content(img);
    i.set_scale(ImageScalePolicy::CropCenter);
    
    i.into_control()
}
fn create_progress_bar(progress: Progress) -> Box<dyn Control> {
	imp::ProgressBar::with_progress(progress).into_control()
}
fn create_frame(name: &str, child: Box<dyn Control>) -> Box<dyn Control> {
    let mut frame = imp::Frame::with_label(name);
    frame.set_child(Some(child));
    frame.into_control()
}

fn create_text(text: &str) -> Box<dyn Control> {
    imp::Text::with_text(text).into_control()
}

fn create_splitted(o: layout::Orientation, first: Box<dyn Control>, second: Box<dyn Control>) -> Box<dyn Control> {
    imp::Splitted::with_content(first, second, o).into_control()
}

fn create_button<'a, F, S>(name: &str, f: F, tag: Option<S>) -> Box<dyn Control>
where
    F: FnMut(&mut dyn Clickable) + 'static,
    S: Into<Cow<'a, str>>
{
    let mut button = imp::Button::with_label(name);
    button.on_click(Some(f.into()));
    button.on_size(Some(
        (|_: &mut dyn HasSize, w: u16, h: u16| {
            println!("button resized too to {}/{}", w, h);
            true
        })
        .into(),
    ));
    button.set_tag(tag.map(|tag|tag.into()));
    button.into_control()
}

fn create_vertical_layout(mut args: Vec<Box<dyn Control>>) -> Box<dyn Control> {
    let mut vb = imp::LinearLayout::with_orientation(layout::Orientation::Vertical);
    vb.on_size(Some(
        (|_: &mut dyn HasSize, w: u16, h: u16| {
            println!("vb resized to {}/{}", w, h);
            true
        })
        .into(),
    ));
    for mut arg in args.drain(..) {
        match vb.layout_orientation() {
            layout::Orientation::Horizontal => {
                arg.set_layout_width(layout::Size::WrapContent);
                arg.set_layout_height(layout::Size::MatchParent);
            }
            layout::Orientation::Vertical => {
                arg.set_layout_width(layout::Size::MatchParent);
                arg.set_layout_height(layout::Size::WrapContent);
            }
        }
        vb.push_child(arg);
    }
    vb.into_control()
}

fn button_click(b: &mut dyn Clickable) {
    let b = b.as_any_mut().downcast_mut::<imp::Button>().unwrap();

    println!("button clicked: {} / {:?}", b.label(), b.id());
    //b.set_visibility(Visibility::Gone);
    //b.set_visibility(Visibility::Invisible);

    let parent = b.is_control_mut().unwrap().parent_mut().unwrap().is_container_mut().unwrap().is_multi_mut().unwrap();

    if parent.len() < 4 {
        println!("add child");
        parent.push_child(root());
        let _ = imp::Message::start_with_actions(
            TextContent::Plain("So far so good".into()),
            MessageSeverity::Info,
            vec![
                (
                    "Ok".into(),
                    (|m: &mut dyn Member| {
                        let _ = imp::Message::start_with_actions(TextContent::LabelDescription("Even better".into(), "Keep doing it".into()), MessageSeverity::Warning, vec![], Some(m));
                        true
                    })
                    .into(),
                ),
                (
                    "Close".into(),
                    (|m: &mut dyn Member| {
                        println!("{:?} closed", m.id());
                        false
                    })
                    .into(),
                ),
            ],
            Some(parent.as_member()),
        );
    } else {
        println!("remove child");
        parent.pop_child();
        let _ = imp::Message::with_content(TextContent::LabelDescription("Crap happened".into(), "We did all we could".into()), MessageSeverity::Alert, Some(parent.as_member())).start();
    }
}

fn root() -> Box<dyn Control> {
    let click_2 = |b: &mut dyn Clickable| {
        let b = b.as_any_mut().downcast_mut::<imp::Button>().unwrap();

        println!("button clicked: {} / {:?}", b.label(), b.as_control().id());
        {
            use plygui::FindBy as By;

            let id = b.id();
            let parent = b.parent_mut().unwrap();
            let parent_member_id = parent.as_any().type_id();
            println!("parent is {:?}", parent_member_id);

            let parent = parent.is_container_mut().unwrap();
            println!("clicked is {:?}", parent.find_control(By::Id(id)).unwrap().as_any().type_id());

            let parent = parent.is_multi_mut().unwrap();
            parent.child_at_mut(0).unwrap().set_visibility(Visibility::Visible);
            
            if let Some(member) = parent.find_control_mut(By::Tag("tagg".into())) {
                member.as_any_mut().downcast_mut::<imp::Button>().unwrap().click(false);
            }
        }
    };
    create_splitted(
    	layout::Orientation::Horizontal,
        create_frame(
            "Frame #1",
            create_vertical_layout(vec![
                create_button("Button #1", button_click, Some("tagg")),
                create_button("Button #2", click_2, Option::<String>::None),
                create_text("I am text"),
                //create_image(),
            ]),
        ),
        create_frame(
            "Frame #2",
            create_vertical_layout(vec![
                create_button("Button #1", button_click, Option::<String>::None),
                create_button("Button #2", click_2, Option::<String>::None),
                create_text("I'm a text too"),
                //create_image(),
            ]),
        ),
    )
}

fn root2() -> Box<dyn Control> {
	create_splitted(
		layout::Orientation::Vertical, 
		create_progress_bar(Progress::Value(35, 100)),
		create_image(), 
	)
}

pub fn exec(feeders: Arc<RwLock<Vec<callbacks::AsyncFeeder<callbacks::OnFrame>>>>) {
    let mut application = imp::Application::get().unwrap();

    feeders.write().unwrap().push(application.on_frame_async_feeder());

    let mut window = application.new_window("plygui!!", WindowStartSize::Exact(800, 500), None);

    window.on_size(Some(
        (|_: &mut dyn HasSize, w: u16, h: u16| {
            println!("win resized to {}/{}", w, h);
            true
        })
        .into(),
    ));
    window.on_close(Some(
        (|w: &mut dyn Closeable| {
            let actions = vec![
                (
                    "Okay".into(),
                    (|m: &mut dyn Member| {
                        let _ = imp::Message::start_with_actions(TextContent::LabelDescription("Good boi".into(), "Keep working".into()), MessageSeverity::Info, vec![], Some(m));
                        true
                    })
                    .into(),
                ),
                (
                    "Close I said!".into(),
                    (|m: &mut dyn Member| {
                        println!("{:?} closed", m.id());

                        false
                    })
                    .into(),
                ),
            ];
            if let Ok(answer) = imp::Message::start_with_actions(
                TextContent::LabelDescription("No close man".into(), "Srsly".into()),
                MessageSeverity::Warning,
                actions,
                Some(w.as_any_mut().downcast_mut::<imp::Window>().unwrap()),
            ) {
                if answer == "Close I said!" {
                    return true;
                }
            }
            false
        })
        .into(),
    ));
    window.set_child(Some(root()));

    let mut tray = application.new_tray(
        "Tray of Plygui",
        Some(vec![
            MenuItem::Action(
                "Exit".into(),
                (|_m: &mut dyn Member| {
                    let application = imp::Application::get().unwrap();
                    application.exit(false)
                })
                .into(),
                MenuItemRole::Help,
            ),
            MenuItem::Action("No tray please".into(), (|m: &mut dyn Member| m.as_any_mut().downcast_mut::<imp::Tray>().unwrap().close(true)).into(), MenuItemRole::Help),
        ]),
    );
    tray.set_image(Cow::Owned(external::image::load_from_memory(include_bytes!("../resources/icon512x512.png")).unwrap()));
    
    
    let mut wi = application.new_window(
        "guiply %)",
        WindowStartSize::Exact(400, 400),
        Some(vec![
            MenuItem::Sub(
                "Help".into(),
                vec![MenuItem::Action(
                    "About".into(),
                    (|_m: &mut dyn Member| {
                        println!("Plygui: Test");
                        true
                    })
                    .into(),
                    MenuItemRole::None,
                )],
                MenuItemRole::Help,
            ),
            MenuItem::Sub(
                "Old".into(),
                vec![
                    MenuItem::Action(
                        "Older".into(),
                        (|_m: &mut dyn Member| {
                            println!("Something old!");
                            true
                        })
                        .into(),
                        MenuItemRole::Options,
                    ),
                    MenuItem::Delimiter,
                    MenuItem::Action(
                        "Oldest".into(),
                        (|_m: &mut dyn Member| {
                            println!("Yikes!");
                            true
                        })
                        .into(),
                        MenuItemRole::None,
                    ),
                ],
                MenuItemRole::None,
            ),
        ]),
    );
    
    wi.set_child(Some(root2()));

    application.start();

    println!("Exiting");
}
