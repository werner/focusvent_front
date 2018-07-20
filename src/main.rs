#[macro_use] extern crate stdweb;
#[macro_use] extern crate yew;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod agents;
mod services;

use yew::prelude::*;
use agents::router;
use stdweb::unstable::TryFrom;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChildComponent {
    Js(stdweb::Value),
    ProductList,
    Parent
}

impl TryFrom<stdweb::Value> for ChildComponent {
    type Error = ();
    fn try_from(value: stdweb::Value) -> Result<ChildComponent, ()> {
        Ok(ChildComponent::Js(value))
    }
}

impl Default for ChildComponent {
    fn default() -> ChildComponent { ChildComponent::Parent }
}

js_serializable!(ChildComponent);

enum Msg {
    NavigateTo(ChildComponent),
    HandleRoute(router::Route<ChildComponent>)
}

struct RootModel {
    child_component: ChildComponent,
    router: Box<Bridge<router::Router<ChildComponent>>>
}

impl Component for RootModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: router::Route<ChildComponent>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);
        RootModel {
            child_component: ChildComponent::Parent,
            router 
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleRoute(route) => {
                self.child_component = route.state;
                true
            },
            Msg::NavigateTo(child_component) => {
                let route = router::Route {
                    path_segments: vec!["products".to_string()],
                    query: None,
                    fragment: None,
                    state: child_component
                };

                self.router.send(router::Request::ChangeRoute(route));
                false
            }
        }
    }
}

impl Renderable<RootModel> for RootModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "Hola Mundo" }</h1>
                <button onclick=|_| Msg::NavigateTo(ChildComponent::ProductList),>{ "Lista de Productos" }</button>
                <div>
                  { self.child_component.view() }
                </div>
            </div>
        }
    }
}

impl Renderable<RootModel> for ChildComponent {
    fn view(&self) -> Html<RootModel> {
        match *self {
            ChildComponent::ProductList => {
                html! {
                    <h2> { "Lista de Productos" } </h2>
                }
            },
            ChildComponent::Parent => {
                html! { <div></div> }
            },
            ChildComponent::Js(_) => { html! { <div></div> } }
        }
    }
}

fn main() {
    yew::initialize();
    App::<RootModel>::new().mount_to_body();
    yew::run_loop();
}
