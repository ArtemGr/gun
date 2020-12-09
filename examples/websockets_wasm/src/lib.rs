use std::sync::Arc;

use async_std::task;
use gun::GunBuilder;
use gun_websockets_wasm::WebsocketsWASM;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Cat {
	name: String,
	color: String,
}

async fn gun() {
	env_logger::Builder::from_default_env()
	    .filter(None, log::LevelFilter::Info)
	    .init();

	let mut gun = GunBuilder::new().peers(&["https://e2eec.herokuapp.com/gun"]);
	WebsocketsWASM::plug_into(&mut gun);
	let gun = gun.build();
	let gun = Arc::new(gun);

	let gun_clone = gun.clone();
	task::spawn_local(async move { gun_clone.start().await });

	gun.get("cat").put(Cat { name: "henry".into(), color: "grey".into() }).await.unwrap();

	gun.get("cat").once(|cat: Cat| {
		log::info!("{:?}", cat);
	}).await.unwrap();
}

use mogwai::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Button {
    pub clicks: i32
}

#[derive(Clone)]
pub enum ButtonIn {
    Click
}

#[derive(Clone)]
pub enum ButtonOut {
    Clicks(String)
}

impl Component for Button {
    type ModelMsg = ButtonIn;
    type ViewMsg = ButtonOut;
    type DomNode = HtmlElement;

    fn update(
        &mut self,
        msg: &ButtonIn,
        tx_view: &Transmitter<ButtonOut>,
        _subscriber: &Subscriber<ButtonIn>
    ) {
        match msg {
            ButtonIn::Click => {
                self.clicks += 1;
                let text = if self.clicks == 1 {
                    "Clicked 1 time".to_string()
                } else {
                    format!("Clicked {} times", self.clicks)
                };
                tx_view.send(&ButtonOut::Clicks(text))
            }
        }
    }

    // Notice that the `Component::view` function returns a `ViewBuilder<T>` and not
    // a `View<T>`.
    fn view(
        &self,
        tx: &Transmitter<ButtonIn>,
        rx: &Receiver<ButtonOut>
    ) -> ViewBuilder<HtmlElement> {
        let tx_event = tx.contra_map(|_:&Event| ButtonIn::Click);
        let rx_text = rx.branch_map(|ButtonOut::Clicks(text)| text.clone());

        builder!(
            // Create a button that transmits a message into tx_event on click.
            <button on:click=tx_event>
                // Using braces we can embed rust values in our DOM.
                // Here we're creating a text node that starts with the
                // string "Clicked 0 times" and then updates every time a
                // message is received on rx_text.
                {("Clicked 0 times", rx_text)}
            </button>
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
	let gizmo = Gizmo::from(Button{ clicks: 0 });
	// ...and create a View from that gizmo's builder.
	let view = View::from(gizmo.view_builder());
	// Queue some messages for the component, as if the button had been clicked:
	gizmo.send(&ButtonIn::Click);
	gizmo.send(&ButtonIn::Click);

	assert_eq!(&view.html_string(), "<button>Clicked 2 times</button>");

	task::spawn_local(gun());

	if cfg!(target_arch = "wasm32") {
	    // running a view adds its DOM to the document.body and ownership is passed to the window,
	    // so this only works in the browser
	    view.run().unwrap_throw()
	}
}
