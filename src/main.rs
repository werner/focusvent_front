extern crate stdweb;
#[macro_use] extern crate yew;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod agents;
mod services;

use yew::prelude::*;
use agents::router;
use std::collections::HashMap;

#[derive(Clone)]
struct ProductComponent {}

impl ChildComponent for ProductComponent {
    fn route(&self) -> String {
        "products".to_string()
    }

    fn box_clone(&self) -> Box<ChildComponent> {
        Box::new((*self).clone())
    }
}

impl Component for ProductComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ProductComponent {  }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<ProductComponent> for ProductComponent {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h2> { "Productos" } </h2>
            </div>
        }
    }
}

pub trait ChildComponent {
    fn route(&self) -> String;
    fn box_clone(&self) -> Box<ChildComponent>;
}

impl Clone for Box<ChildComponent>
{
    fn clone(&self) -> Box<ChildComponent> {
        self.box_clone()
    }
}

impl PartialEq for Box<ChildComponent> {
    fn eq(&self, other: &Box<ChildComponent>) -> bool {
        self.route() == other.route()
    }
}

enum Msg {
    NavigateTo(Box<ChildComponent>),
    HandleRoute(router::Route<()>),
    AddRoute(String, Box<ChildComponent>)
}

struct RootModel {
    child_component: Option<Box<ChildComponent>>,
    router: Box<Bridge<router::Router<()>>>,
    routes: HashMap<String, Box<ChildComponent>>
}

impl Component for RootModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: router::Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);
        RootModel { child_component: None, router, routes: HashMap::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddRoute(path, component) => {
                self.routes.insert(path, component);
                false
            },
            Msg::HandleRoute(route) => {
                self.child_component = route.path_segments.get(0).and_then(|first_segment| {
                    self.routes.get(first_segment.as_str()).and_then(|component| Some((*component).clone()))
                }); 
                    
                true
            },
            Msg::NavigateTo(child_component) => {
                let mut path_routes = Vec::new();
                for (key_route, component) in &self.routes {
                    if component == &child_component {
                        path_routes.push(key_route.to_string());
                        break;
                    }
                }

                let route = router::Route {
                    path_segments: path_routes, //TODO: Seems to required much more
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
                <button onclick=|_| Msg::NavigateTo(Box::new(ProductComponent {  })),>
                    { "Lista de Productos" }
                </button>
                <div>
                  { 
                      if let Some(ref component) = self.child_component {
                          component.view()
                      } else {
                          "".to_string()
                      }
                  }
                </div>
            </div>
        }
    }
}

//impl Renderable<RootModel> for ChildComponent {
//    fn view(&self) -> Html<RootModel> {
//        match *self {
//
//            ChildComponent::ProductList => {
//                html! {
//                    <h2> { "Lista de Productos" } </h2>
//                }
//            },
//            ChildComponent::Parent => {
//                html! { <div></div> }
//            },
//            ChildComponent::PathNotFound(ref path) => html! {
//                <div>
//                    {format!("Invalid path: '{}'", path)}
//                </div>
//            }
//        }
//    }
//}

fn main() {
    yew::initialize();
    let mut app = App::<RootModel>::new().mount_to_body();
    app.send_message(Msg::AddRoute("products".to_string(), Box::new(ProductComponent {  })));
    yew::run_loop();
}
