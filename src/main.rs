use chrono::Utc;
use gloo::timers::callback::Interval;
use yew::prelude::*;

pub struct App {
	_interval: Interval,
	unix_epoch: i64,
}

pub enum Message {
	Tick,
}

impl App {
	fn get_unix_epoch() -> i64 {
		Utc::now().timestamp()
	}
}

impl Component for App {
	type Message = Message;
	type Properties = ();

	fn create(context: &Context<Self>) -> Self {
		let _interval = {
			let link = context.link().clone();
			Interval::new(1000, move || link.send_message(Message::Tick))
		};

		Self {
			_interval,
			unix_epoch: Self::get_unix_epoch(),
		}
	}

	fn update(&mut self, _context: &Context<Self>, message: Self::Message) -> bool {
		match message {
			Message::Tick => {
				self.unix_epoch = Self::get_unix_epoch();
				true
			}
		}
	}

	fn view(&self, _context: &Context<Self>) -> Html {
		html! {
			<div>
				<h1>{ "This is running in Rust with Yew, and the unix epoch is " } { self.unix_epoch } { "!" }</h1>
			</div>
		}
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}
