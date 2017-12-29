extern crate plygui;

use plygui::*;

fn main() {
	let mut application = Application::with_name("Plygui test");
	
	let mut window = application.new_window("plygui!!", WindowStartSize::Exact(200, 200), false);
	
	window.on_resize(Some((|_: &mut UiMember, w: u16, h: u16| {
		println!("win resized to {}/{}", w, h);
	}).into()));
	
	let mut vb = LinearLayout::new(layout::Orientation::Vertical);
	vb.on_resize(Some((|_: &mut UiMember, w: u16, h: u16| {
		println!("wb resized to {}/{}", w, h);
	}).into()));
	
	/*let mut vbb = LinearLayout::new(layout::Orientation::Horizontal);
	vbb.set_layout_params(layout::Params::WrapContent, layout::Params::MatchParent);
	vbb.push_child(Button::new("Butt0"));
	vbb.push_child(Button::new("Butt00"));
	vb.push_child(vbb);*/
	
	let mut button = Button::new("Butt1");
	let butt1_id = button.as_base().id();
	//button.set_layout_params(layout::Params::WrapContent, layout::Params::MatchParent);
	button.on_left_click(Some((|b: &mut UiButton| {
		println!("button clicked: {}", b.label());
		b.set_visibility(Visibility::Gone);
		//b.set_visibility(Visibility::Invisible);
	}).into()));
	button.on_resize(Some((|_: &mut UiMember, w: u16, h: u16| {
		println!("button resized too to {}/{}", w, h);
	}).into()));
	vb.push_child(button);
	
	let mut button = Button::new("Butt2");
	//button.set_layout_params(layout::Params::WrapContent, layout::Params::MatchParent);
	button.on_left_click(Some((move |b: &mut UiButton| {
		println!("button clicked: {} / {:?}", b.label(), b.as_base().id());
		{
			let parent = b.parent().unwrap();
			let parent_member_id = parent.member_id();
			println!("parent is {:?}", parent_member_id);
			
			let parent: &UiContainer = match parent_member_id {
				members::MEMBER_ID_WINDOW => { 
					let parent: &Window = utils::common_to_impl(parent); 
					parent
				},
				members::MEMBER_ID_LAYOUT_LINEAR => { 
					let parent: &LinearLayout = utils::common_to_impl(parent); 
					parent
				},
				_ => unreachable!(),
			};
			
			println!("clicked is {:?}", parent.find_control_by_id(b.as_base().id()).unwrap().as_base().member_id());
		}
		let root = b.root_mut().unwrap();
		let root_member_id = root.member_id();
		println!("root is {:?}", root_member_id);
		
		let root: &mut UiContainer = match root_member_id {
			members::MEMBER_ID_WINDOW => { 
				let root: &mut Window = utils::common_to_impl_mut(root); 
				root
			},
			members::MEMBER_ID_LAYOUT_LINEAR => { 
				let root: &mut LinearLayout = utils::common_to_impl_mut(root); 
				root
			},
			_ => unreachable!(),
		};
		
		let butt1 = root.find_control_by_id_mut(butt1_id).unwrap();
		butt1.set_visibility(Visibility::Visible);
	}).into()));
	button.on_resize(Some((|_: &mut UiMember, w: u16, h: u16| {
		println!("button resized too to {}/{}", w, h);
	}).into()));
	vb.push_child(button);
	
	window.set_child(Some(vb));
	
	//window.set_child(Some(button));
	
	application.start();
}