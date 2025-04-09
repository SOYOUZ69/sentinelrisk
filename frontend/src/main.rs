use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::risk_list::RiskList;
use pages::add_risk::AddRisk;

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/risks")]
    Risks,
    #[at("/risks/new")]
    AddRisk,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav>
                <ul>
                    <li><Link<Route> to={Route::Risks}>{ "Liste des Risques" }</Link<Route>></li>
                    <li><Link<Route> to={Route::AddRisk}>{ "Ajouter un Risque" }</Link<Route>></li>
                </ul>
            </nav>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home | Route::Risks => html! { <RiskList /> },
        Route::AddRisk => html! { <AddRisk /> },
        Route::NotFound => html! { <h1>{ "404 - Page non trouvée" }</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}