extern crate stdweb;
#[macro_use] extern crate yew;
#[macro_use] extern crate yew_router;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterButton;
use yew_router::components::RouterLink;

use components::product_list::ProductList;

enum Msg {
    NoOp
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Props;

struct Model { }
struct Home { }

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NoOp => false
        }
    }
}

impl Component for Home {
    type Message = ();
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Routable for Home {
    fn resolve_props(route: &Route) -> Option<Self::Properties> {
        let first_segment = route.path_segments.get(0).unwrap();
        if "" == first_segment.as_str() {
            Some(Props)
        } else {
            None
        }
    }

    fn will_try_to_route(route: &Route) -> bool {
        route.path_segments.get(0).is_some()
    }
}

impl Renderable<Home> for Home {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "Home" }</h1>
            </div>
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let router_props: yew_router::Props = yew_router::Props {
            routes: routes![ProductList, Home],
            page_not_found: Some(DefaultPage(routing_failed_page))
        };
        html! {
            <div>
                <nav class="menu",>
                    <RouterButton: text=String::from("Go to Products"), route=Route::parse("/products"), />
                    <RouterLink: text=String::from("Go to Products"), route=Route::parse("/products"), />
                </nav>
                <div>
                    <YewRouter: with router_props, />
                </div>
            </div>
        }
    }
}

fn routing_failed_page(route: &Route) -> Html<YewRouter> {
    html! {
        <>
            {"This is the default 404 page"}
            <br/>
            {format!("Could not route: '{}'", route.to_route_string())}
        </>
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
