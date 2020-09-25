//use plygui::common_api as common;
use plygui::*;

use std::borrow::Cow;
use std::fs::*;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

/*
fn create_tree() -> Box<dyn Control> {
    let level22 = types::RecursiveTupleVec::with_value("22", None);
    let level21 = types::RecursiveTupleVec::with_value("21", None);
    let level20 = types::RecursiveTupleVec::with_value("20", None);
    let level10 = types::RecursiveTupleVec::with_value("10", Some(vec![level20, level21, level22]));
    let mut level00 = types::RecursiveTupleVec::with_value("00", Some(vec![level10]));
    
    println!("{:#?}", level00);
    
    if let Ok(inn) = level00.put(&[1], Some(types::RecursiveTupleVec::with_value("11", None))) {
        if inn.is_some() { 
            panic!("Some! {:#?}", level00); 
        }
    }
    
    println!("{:#?}", level00);
    
    if let Ok(inn) = level00.put(&[0, 0], Some(types::RecursiveTupleVec::with_value("new 20", Some(vec![types::RecursiveTupleVec::with_value("30", None)])))) {
        if inn.is_none() { 
            panic!("None! {:#?}", level00); 
        }
    }
    
    println!("{:#?}", level00);
    
	let adapter = Box::new(common::SimpleTextAdapter::with_into_iterator(&["1", "11", "111", "1111"]));
    let mut tree = imp::Tree::with_adapter(adapter);
    tree.set_layout_height(layout::Size::MatchParent);
    tree.into_control()
}


*/

fn create_tree() -> Box<dyn Control> {
    let level22 = types::RecursiveTupleVec::with_value(String::from("22"), None);
    let level21 = types::RecursiveTupleVec::with_value(String::from("21"), None);
    let level20 = types::RecursiveTupleVec::with_value(String::from("20"), None);
    let level10 = types::RecursiveTupleVec::with_value(String::from("10"), Some(vec![level20, level21, level22]));
    let mut level00 = types::RecursiveTupleVec::with_value(String::from("00"), Some(vec![level10]));
    
    println!("{:#?}", level00);
    
    if let Ok(inn) = level00.put(&[1], Some(types::RecursiveTupleVec::with_value(String::from("11"), None))) {
        if inn.is_some() { 
            panic!("Some! {:#?}", level00); 
        }
    }
    
    println!("{:#?}", level00);
    
    if let Ok(inn) = level00.put(&[0, 0], Some(types::RecursiveTupleVec::with_value(String::from("new 20"), Some(vec![types::RecursiveTupleVec::with_value(String::from("30"), None)])))) {
        if inn.is_none() { 
            panic!("None! {:#?}", level00); 
        }
    }
    
    println!("{:#?}", level00);
    
    let adapter = common::SimpleTextTreeAdapter::from(level00);
    
    adapter.for_each(&mut (|i, s| {
        println!("{:?} {:?}", i, s);
    }));
    
    let adapter = Box::new(adapter);
    let mut list = imp::Tree::with_adapter(adapter);
    list.set_layout_height(layout::Size::MatchParent);
    list.on_item_click(Some(
        (|p: &mut dyn ItemClickable, i: &[usize], item_view: &mut dyn Control| {
            item_view.as_any_mut().downcast_mut::<imp::Text>().unwrap().set_label(format!("clicked {:?}", i).into());
            if (i[i.len()-1] % 2) > 0 {
                let _ = imp::Message::start_with_actions(
                    TextContent::Plain("Even :)".into()),
                    MessageSeverity::Info,
                    vec![],
                    Some(p.as_member())
                );
            } else {
                let _ = imp::Message::start_with_actions(
                    TextContent::Plain("Odd :(".into()),
                    MessageSeverity::Info,
                    vec![],
                    Some(p.as_member())
                );
            }
        })
        .into(),
    ));
    list.into_control()
}

fn create_list() -> Box<dyn Control> {
    let adapter = Box::new(common::SimpleTextAdapter::with_into_iterator(&["1"]));
    let mut list = imp::List::with_adapter(adapter);
    list.set_layout_height(layout::Size::MatchParent);
    list.on_item_click(Some(
        (|p: &mut dyn ItemClickable, i: &[usize], item_view: &mut dyn Control| {
            item_view.as_any_mut().downcast_mut::<imp::Text>().unwrap().set_label(format!("clicked {}", i[0]).into());
            let adapter = p.as_any_mut().downcast_mut::<imp::List>().unwrap().adapter_mut().as_any_mut().downcast_mut::<common::SimpleTextAdapter>().unwrap();
            if (i[0] % 2) > 0 {
                adapter.pop();
            } else {
                adapter.push(format!("More clicked {} / pressed {}", adapter.len_at(&[]).unwrap(), i[0]));
            }
        })
        .into(),
    ));
    list.into_control()
}

fn create_image(policy: ImageScalePolicy) -> Box<dyn Control> {
    let img = external::image::load(BufReader::new(File::open("resources/lulz.png").unwrap()), external::image::ImageFormat::Png).unwrap();
    let mut i = imp::Image::with_content(img);
    i.set_scale(policy);
    i.set_layout_height(layout::Size::MatchParent);
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
    let mut s = imp::Splitted::with_content(first, second, o).into_control();
    s.set_layout_height(layout::Size::MatchParent);
    s
}

fn create_button<'a, F, S>(name: &str, f: F, tag: Option<S>) -> Box<dyn Control>
where
    F: FnMut(&mut dyn Clickable) + 'static,
    S: Into<Cow<'a, str>>,
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
    button.set_tag(tag.map(|tag| tag.into()));
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
    vb.set_layout_height(layout::Size::MatchParent);
    for arg in args.drain(..) {
        vb.push_child(arg);
    }
    vb.into_control()
}

fn button_click(b: &mut dyn Clickable) {
    let b = b.as_any_mut().downcast_mut::<imp::Button>().unwrap();

    println!("button clicked: {} / {:?}", b.label(), b.id());
    //b.set_visibility(Visibility::Gone);
    //b.set_visibility(Visibility::Invisible);

    let parent = b.parent_mut().unwrap().is_container_mut().unwrap().is_multi_container_mut().unwrap();

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

            let parent = parent.is_multi_container_mut().unwrap();
            parent.child_at_mut(0).unwrap().set_visibility(Visibility::Visible);

            if let Some(member) = parent.find_control_mut(By::Tag("tagg".into())) {
                member.as_any_mut().downcast_mut::<imp::Button>().unwrap().click(false);
            }
        }
    };
    let mut s = create_splitted(
        layout::Orientation::Horizontal,
        create_frame(
            "Frame #1",
            create_vertical_layout(vec![
                create_button("Button #1", button_click, Some("tagg")),
                create_button("Button #2", click_2, Option::<String>::None),
                create_text("I am text"),
                //create_table(),
                create_tree(),
                //create_image(ImageScalePolicy::FitCenter),
            ]),
        ),
        create_frame(
            "Frame #2",
            create_vertical_layout(vec![
                create_button("Button #1", button_click, Option::<String>::None),
                create_button("Button #2", click_2, Option::<String>::None),
                create_text("I'm a text too"),
                create_list(),
                //create_image(),
            ]),
        ),
    );
    s.set_layout_width(layout::Size::MatchParent);
    s.set_layout_height(layout::Size::MatchParent);
    s
}

fn root2() -> Box<dyn Control> {
    let mut s = create_splitted(layout::Orientation::Horizontal, create_progress_bar(Progress::Value(35, 100)), create_image(ImageScalePolicy::CropCenter));
    s.set_layout_width(layout::Size::MatchParent);
    s.set_layout_height(layout::Size::MatchParent);
    s
}

pub fn exec(feeders: Arc<RwLock<Vec<callbacks::AsyncFeeder<callbacks::OnFrame>>>>) {
    let mut application = imp::Application::with_name("plygui");

    feeders.write().unwrap().push(application.on_frame_async_feeder());

    let window = application.new_window::<imp::Window>("plygui!!", WindowStartSize::Exact(800, 500), None);
    {
        let window = application.find_member_mut(FindBy::Id(window)).unwrap().as_any_mut().downcast_mut::<imp::Window>().unwrap();
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
    }

    let _tray = application.new_tray::<imp::Tray>(
        "Tray of Plygui",
        external::image::load_from_memory(include_bytes!("../resources/icon128x128.png")).unwrap(),
        Some(vec![
            MenuItem::Action(
                "Exit".into(),
                (|m: &mut dyn Member| {
                    let application = m.as_any_mut().downcast_mut::<imp::Tray>().unwrap().application_mut();
                    application.prepare_exit();
                    true
                })
                .into(),
                MenuItemRole::Help,
            ),
            MenuItem::Action(
                "No tray please".into(),
                (|m: &mut dyn Member| {
                    let id = m.id();
                    m.as_any_mut().downcast_mut::<imp::Tray>().unwrap().application_mut().close_root(FindBy::Id(id), true)
                })
                .into(),
                MenuItemRole::Help,
            ),
        ]),
    );
    let wi = application.new_window::<imp::Window>(
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

    {
        let wi = application.find_member_mut(FindBy::Id(wi)).unwrap().as_any_mut().downcast_mut::<imp::Window>().unwrap();
        wi.set_child(Some(root2()));
    }

    application.start();

    println!("Exiting");
}
