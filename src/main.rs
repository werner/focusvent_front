extern crate stdweb;
#[macro_use] extern crate yew;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod agents;
mod services;

use yew::prelude::*;
use agents::router;

pub enum EComponent {
    ProductList
}

enum Msg {
    NavigateTo(EComponent),
    HandleRoute(router::Route<()>)
}

struct Model {
    router: Box<Bridge<router::Router<()>>>
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: router::Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);
        Model { router }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleRoute(route) => {
                true
            },
            Msg::NavigateTo(component) => {
                let route = router::Route {
                    path_segments: vec!["products".to_string()],
                    query: None,
                    fragment: None,
                    state: ()
                };

                self.router.send(router::Request::ChangeRoute(route));
                false
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{{ "Hola Mundo" }}</h1>
                <button onclick=|_| Msg::NavigateTo(EComponent::ProductList),>{ "Lista de Productos" }</button>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
