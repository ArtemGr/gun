use std::sync::{Arc, Mutex};

use async_std::task;
use gun::GunBuilder;
use gun_websockets_wasm::WebsocketsWASM;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Cat {
    name: String,
    color: String,
}

use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App {
    clicks: u32,
    cat: Arc<Mutex<Option<Cat>>>,
}

#[derive(Clone)]
enum AppModel {
    Click,
}

#[derive(Clone)]
enum AppView {
    Clicked(u32),
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::Click => {
                self.clicks += 1;
                tx.send(&AppView::Clicked(self.clicks));
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        let cat_message = match &*self.cat.lock().unwrap() {
            Some(cat) => format!("{}\n{}", cat.name, cat.color),
            None => String::from("No cat"),
        };
        builder! {
            <div>
                <button on:click=tx.contra_map(|_| AppModel::Click)>
                    {(
                        "Hello from mogwai!",
                        rx.branch_map(|msg| {
                            match msg {
                                AppView::Clicked(1) => format!("Caught 1 click, click again 😀"),
                                AppView::Clicked(n) => format!("Caught {} clicks", n),
                            }
                        })
                    )}
                </button>
                <p>
                    {cat_message}
                </p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let mut gun = GunBuilder::new().peers(&["https://e2eec.herokuapp.com/gun"]);
    WebsocketsWASM::plug_into(&mut gun);
    let gun = gun.build();
    let gun = Arc::new(gun);

    let gun_clone = gun.clone();
    task::spawn_local(async move { gun_clone.start().await });

    let cat = Arc::new(Mutex::new(None));

    // let cat_clone = cat.clone();
    // task::spawn_local(async move {
    //     gun.get("cat").put(Cat { name: "henry".into(), color: "grey".into() }).await.unwrap();

    //     *cat_clone.lock().unwrap() = Some(gun.get("cat").once::<Cat>().await.unwrap());
    // });

    let gizmo = Gizmo::from(App { clicks: 0, cat });
    let view = View::from(gizmo.view_builder());
    view.run()
}
