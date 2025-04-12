use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

mod pages;
use pages::risk_list::RiskList;
use pages::add_risk::AddRisk;
use pages::edit_risk::EditRisk;
use pages::view_risk::ViewRisk;
mod components;
use components::sidebar::Sidebar;

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/risks")]
    Risks,
    #[at("/risks/new")]
    AddRisk,
    #[at("/risks/edit/:id")]
    EditRisk { id: String },
    #[at("/risks/view/:id")]
    ViewRisk { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div style="display: flex;">
                <Sidebar />
                <div style="flex: 1; padding: 1rem;">
                    <Switch<Route> render={switch} />
                </div>
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home | Route::Risks => html! { <RiskList /> },
        Route::AddRisk => html! { <AddRisk /> },
        Route::EditRisk { id } => html! { <EditRisk /> },
        Route::ViewRisk { id } => html! { <ViewRisk /> },
        Route::NotFound => html! { <h1>{ "404 - Page non trouvée" }</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}