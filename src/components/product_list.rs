use yew::prelude::*;
use yew_router::prelude::*;

pub struct ProductList {  }

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Props;

pub enum Msg {  }

impl Component for ProductList {
    type Message = Msg;
    type Properties = Props;

   fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ProductList { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

}

impl Renderable<ProductList> for ProductList {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h2> { "Lista de Productos" } </h2>
            </div>
        }
    }
}

impl Routable for ProductList {
    fn resolve_props(route: &Route) -> Option<Self::Properties> {
        let first_segment = route.path_segments.get(0).unwrap();
        if "products" == first_segment.as_str() {
            Some(Props)
        } else {
            None // This will only render if the first path segment is "a"
        }
    }

    fn will_try_to_route(route: &Route) -> bool {
        route.path_segments.get(0).is_some()
    }
}
