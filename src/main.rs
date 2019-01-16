#![feature(get_type_id)]



use plygui::*;

fn create_frame(name: &str, child: Box<dyn Control>) -> Box<dyn Control> {
	let mut frame = imp::Frame::with_label(name);    
    frame.set_child(Some(child));
    frame.into_control()
}

fn create_text(text: &str) -> Box<dyn Control> {
    let text = imp::Text::with_text(text);
    text.into_control()
}

fn create_splitted(first: Box<dyn Control>, second: Box<dyn Control>) -> Box<dyn Control> {
	let mut splitted = imp::Splitted::with_content(first, second, layout::Orientation::Horizontal);
	splitted.set_layout_width(layout::Size::MatchParent);
    splitted.set_layout_height(layout::Size::WrapContent);
    splitted.into_control()
}

fn create_button<F>(name: &str, f: F) -> Box<dyn Control> where F: FnMut(&mut dyn Clickable) + 'static {
	let mut button = imp::Button::with_label(name);
    button.set_layout_width(layout::Size::MatchParent);
    button.set_layout_height(layout::Size::WrapContent);
    button.on_click(Some(f.into()));
    button.on_resize(Some(
        (|_: &mut dyn Member, w: u16, h: u16| {
             println!("button resized too to {}/{}", w, h);
         }).into(),
    ));
    button.into_control()
}

fn create_vertical_layout(mut args: Vec<Box<dyn Control>>) -> Box<dyn Control> {
	let mut vb = imp::LinearLayout::with_orientation(layout::Orientation::Vertical);
    vb.on_resize(Some(
        (|_: &mut dyn Member, w: u16, h: u16| {
             println!("vb resized to {}/{}", w, h);
         }).into(),
    ));
    for mut arg in args.drain(..) {
        match vb.layout_orientation() {
            layout::Orientation::Horizontal => {
                arg.set_layout_width(layout::Size::WrapContent);
                arg.set_layout_height(layout::Size::MatchParent);
            },
            layout::Orientation::Vertical => {
                arg.set_layout_width(layout::Size::MatchParent);
                arg.set_layout_height(layout::Size::WrapContent);
            },
        }
        vb.push_child(arg);
    }
    vb.into_control()
}

fn button_click(b: &mut dyn Clickable) {
	let b = b.as_any_mut().downcast_mut::<imp::Button>().unwrap();
	
    println!("button clicked: {}", b.label());
    //b.set_visibility(Visibility::Gone);
    //b.set_visibility(Visibility::Invisible);
    
    let parent = b.is_control_mut().unwrap().parent_mut().unwrap().is_container_mut().unwrap().is_multi_mut().unwrap();
    
    if parent.len() < 3 {
    	println!("add child");
    	parent.push_child(root());
    	imp::Alert::with_content(TextContent::Plain("So far so good".into()), AlertSeverity::Info, Some(parent.as_member()));
    } else {
        println!("remove child");
    	parent.pop_child();
    	imp::Alert::with_content(TextContent::LabelDescription("Crap happened".into(), "We did all we could".into()), AlertSeverity::Alert, Some(parent.as_member()));
    }
}

fn root() -> Box<dyn Control> {
    let _click_2 = |b: &mut dyn Clickable| {
    	let b = b.as_any_mut().downcast_mut::<imp::Button>().unwrap();
    	
        println!("button clicked: {} / {:?}", b.label(), b.as_control().id());
        {
            let id = b.id();
        	let parent = b.parent_mut().unwrap();
            let parent_member_id = parent.as_any().get_type_id();
            println!("parent is {:?}", parent_member_id);

            let parent = parent.is_container_mut().unwrap();
            println!(
                "clicked is {:?}",
                parent
                    .find_control_by_id(id)
                    .unwrap()
                    .as_any()
                    .get_type_id()
            );
            
            let parent = parent.is_multi_mut().unwrap();
            parent.child_at_mut(0).unwrap().set_visibility(Visibility::Visible);
        }
    };
    create_splitted(
        create_frame("Frame #1", create_vertical_layout(
            vec![
                create_button("Button #1", button_click), 
                //create_button("Button #2", click_2),
                create_text("I am text"),
            ]
        )),
        create_frame("Frame #2", create_vertical_layout(
            vec![
                create_button("Button #1", button_click), 
                //create_button("Button #2", click_2),
                create_text("I'm a text too"),
            ]
        )),
    )
}

fn main() {
    let mut application = imp::Application::with_name("Plygui test");
    let mut window = application.new_window("plygui!!", WindowStartSize::Exact(200, 200), WindowMenu::None);
    window.on_resize(Some(
        (|_: &mut dyn Member, w: u16, h: u16| {
             println!("win resized to {}/{}", w, h);
         }).into(),
    ));
    window.set_child(Some(root()));
    application.start();
    
    println!("Exiting");
}
