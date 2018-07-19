extern crate stdweb;
#[macro_use] extern crate yew;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod agents;
mod services;

use yew::prelude::*;
use agents::router::Route;

enum Msg {
    HandleRoute(Route<()>)
}

struct Model {  }

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {  }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleRoute(route) => {

            }
        };
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
