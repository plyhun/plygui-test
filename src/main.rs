extern crate plygui;
extern crate plygui_win32;

use plygui::*;
use plygui_win32::*;

fn main() {
	let mut application = Application::with_name("Plygui test");
	
	let mut window = application.new_window("plygui!!", 1280, 800, false);
	
	window.on_resize(Some(Box::new(|_, w, h| {
		println!("win resized to {}/{}", w, h);
	})));
	
	let mut button = Button::new("Butt1");
	let butt1_id = button.id();
	//button.set_layout_params(layout::Params::WrapContent, layout::Params::MatchParent);
	button.on_left_click(Some(Box::new(|b| {
		println!("button clicked: {}", b.label());
		b.set_visibility(Visibility::Invisible);
	})));
	button.on_resize(Some(Box::new(|_, w, h| {
		println!("button resized too to {}/{}", w, h);
	})));
	let mut vb = LinearLayout::new(layout::Orientation::Vertical);
	vb.on_resize(Some(Box::new(|_,w,h| {
		println!("wb resized to {}/{}", w, h);
	})));
	vb.push_child(button);
	
	let mut button = Button::new("Butt2");
	//button.set_layout_params(layout::Params::WrapContent, layout::Params::MatchParent);
	button.on_left_click(Some(Box::new(move |b| {
		println!("button clicked: {} / {:?}", b.label(), b.id());
		//b.set_visibility(Visibility::Gone);
		{
			let parent = b.parent().unwrap();
			println!("parent is {:?}", parent.role());
			println!("clicked is {:?}", parent.find_control_by_id(b.id()).unwrap().role());
		}
		let root = b.root_mut().unwrap();
		println!("root is {:?}", root.role());
		
		let butt1 = root.find_control_by_id_mut(butt1_id).unwrap();
		butt1.set_visibility(Visibility::Visible);
	})));
	button.on_resize(Some(Box::new(|_, w, h| {
		println!("button resized too to {}/{}", w, h);
	})));
	vb.push_child(button);
	
	window.set_child(Some(vb));
	
	//window.set_child(Some(button));
	
	application.start();
}