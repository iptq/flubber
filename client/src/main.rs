use relm::{Widget};
use relm_derive::Msg;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Message {

}

#[widget]
impl Widget for MainWin {
	fn model() {

	}

	fn update(&mut self, _: Message) {

	}

	view! {
		gtk::Window {

		}
	}
}

fn main() {
    MainWin::run(()).unwrap();
}
