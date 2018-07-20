extern crate stdweb;
#[macro_use] extern crate yew;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod agents;
mod services;

use yew::prelude::*;
use agents::router;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChildComponent {
    ProductList,
    Parent,
    PathNotFound(String)
}

enum Msg {
    NavigateTo(ChildComponent),
    HandleRoute(router::Route<()>)
}

struct RootModel {
    child_component: ChildComponent,
    router: Box<Bridge<router::Router<()>>>
}

impl Component for RootModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: router::Route<()>| Msg::HandleRoute(route));
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
                self.child_component = if let Some(first_segment) = route.path_segments.get(0) {
                    match first_segment.as_str() {
                        "products" => ChildComponent::ProductList,
                        other => ChildComponent::PathNotFound(other.into())
                    }
                } else {
                    ChildComponent::PathNotFound("path not found".into())
                };
                true
            },
            Msg::NavigateTo(child_component) => {
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
            ChildComponent::PathNotFound(ref path) => html! {
                <div>
                    {format!("Invalid path: '{}'", path)}
                </div>
            }
        }
    }
}

fn main() {
    yew::initialize();
    App::<RootModel>::new().mount_to_body();
    yew::run_loop();
}
