use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::hooks::use_location;
use crate::Route;
use std::rc::Rc;
use std::cell::RefCell;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let show_risks = use_state(|| true);
    let location = use_location().unwrap();
    let current_path = location.path();

    let toggle_risks = {
        let show_risks = show_risks.clone();
        Callback::from(move |_| show_risks.set(!*show_risks))
    };

    html! {
        <div class="d-flex flex-column bg-light" style="width: 220px; height: 100vh; border-right: 1px solid #ccc;">
            <div class="navbar navbar-expand-lg navbar-light bg-light">
                <a class="navbar-brand" href="#">{ "SentinelRisk" }</a>
            </div>
            <div>
                <button 
                    class={classes!(
                        "btn",
                        "btn-link",
                        "w-100",
                        "text-left",
                        if *show_risks { "bg-primary" } else { "" }
                    )}
                    onclick={toggle_risks.clone()}>
                    { format!("{} Gestion des risques", if *show_risks { "🔽" } else { "▶️" }) }
                </button>
                if *show_risks {
                    <ul class="list-group list-group-flush mt-2">
                        <li class="list-group-item p-2">
                            <Link<Route>
                                to={Route::Risks}
                                classes={classes!(
                                    "nav-link",
                                    if current_path == "/risks" { "active" } else { "text-muted" }
                                )}
                            >
                                { "📋 Liste des Risques" }
                            </Link<Route>>
                        </li>
                        <li class="list-group-item p-2">
                            <Link<Route>
                                to={Route::AddRisk}
                                classes={classes!(
                                    "nav-link",
                                    if current_path == "/risks/add" { "active" } else { "text-muted" }
                                )}
                            >
                                { "➕ Ajouter un Risque" }
                            </Link<Route>>
                        </li>
                    </ul>
                }
            </div>
        </div>
    }
}