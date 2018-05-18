#![feature(get_type_id)]

extern crate plygui;

use plygui::*;

fn main() {
    let mut application = Application::with_name("Plygui test");

    let mut window = application.new_window("plygui!!", WindowStartSize::Exact(200, 200), WindowMenu::None);

    window.on_resize(Some(
        (|_: &mut UiMember, w: u16, h: u16| {
             println!("win resized to {}/{}", w, h);
         }).into(),
    ));

    let mut vb = LinearLayout::with_orientation(layout::Orientation::Vertical);
    vb.on_resize(Some(
        (|_: &mut UiMember, w: u16, h: u16| {
             println!("wb resized to {}/{}", w, h);
         }).into(),
    ));

    let mut vbb = LinearLayout::with_orientation(layout::Orientation::Horizontal);
    vbb.set_layout_padding(layout::BoundarySize::AllTheSame(5).into());
    let mut button = Button::with_label("Butt0");
    button.set_layout_width(layout::Size::WrapContent);
    button.set_layout_height(layout::Size::WrapContent);
    button.set_layout_padding(layout::BoundarySize::AllTheSame(5).into());
    vbb.push_child(button.into_control());
    
    let mut button = Button::with_label("Butt00");
    button.set_layout_width(layout::Size::WrapContent);
    button.set_layout_height(layout::Size::WrapContent);
    button.set_layout_padding(layout::BoundarySize::AllTheSame(5).into());
    vbb.push_child(button.into_control());
    
    let mut frame = Frame::with_label("Horizontal Frame");
    frame.set_child(Some(vbb.into_control()));

    vb.push_child(frame.into_control());
    //vb.push_child(vbb.into_control());

    let mut button = Button::with_label("Butt1");
    let butt1_id = button.id();
    //button.set_layout_params(layout::Params::WrapContent, layout::Params::MatchParent);
    button.on_click(Some(
        (|b: &mut UiButton| {
             println!("button clicked: {}", b.label());
             b.set_visibility(Visibility::Gone);
             //b.set_visibility(Visibility::Invisible);
         }).into(),
    ));
    button.on_resize(Some(
        (|_: &mut UiMember, w: u16, h: u16| {
             println!("button resized too to {}/{}", w, h);
         }).into(),
    ));
    vb.push_child(button.into_control());

    let mut button = Button::with_label("Butt2");
    //button.set_layout_params(layout::Params::WrapContent, layout::Params::MatchParent);
    button.on_click(Some(
        (move |b: &mut UiButton| {
            println!("button clicked: {} / {:?}", b.label(), b.as_control().id());
            {
            	let parent = b.parent().unwrap();
                let parent_member_id = parent.as_any().get_type_id();
                println!("parent is {:?}", parent_member_id);

                let parent: &UiContainer = parent.is_container().unwrap();

                println!(
                    "clicked is {:?}",
                    parent
                        .find_control_by_id(b.id())
                        .unwrap()
                        .as_any()
                        .get_type_id()
                );
            }
            let root = b.root_mut().unwrap();
            let root_member_id = root.as_any().get_type_id();
            println!("root is {:?}", root_member_id);

            let root: &mut UiContainer = root.is_container_mut().unwrap();

            let butt1 = root.find_control_by_id_mut(butt1_id).unwrap();
            butt1.set_visibility(Visibility::Visible);
        }).into(),
    ));
    button.on_resize(Some(
        (|_: &mut UiMember, w: u16, h: u16| {
             println!("button resized too to {}/{}", w, h);
         }).into(),
    ));
    vb.push_child(button.into_control());

    window.set_child(Some(vb.into_control()));

    //window.set_child(Some(button.into_control()));

    application.start();
}
