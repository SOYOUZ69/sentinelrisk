use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

use crate::Route;

#[derive(Deserialize, Clone, Debug)]
struct Risk {
    id: String,
    title: String,
    description: Option<String>,
    impact: i32,
    probability: i32,
    status: String,
    external_id: Option<String>,
    category: Option<String>,
    location: Option<String>,
    regulation: Option<String>,
    control_measure_id: Option<String>,
    score: Option<i32>,
}

#[function_component(ViewRisk)]
pub fn view_risk() -> Html {
    let route = use_route::<Route>().unwrap();
    let id = if let Route::ViewRisk { id } = route {
        id
    } else {
        "".to_string()
    };

    let risk = use_state(|| None::<Risk>);
    {
        let risk = risk.clone();
        let id = id.clone();
        use_effect_with((), move |_| {
            let risk = risk.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(&format!("http://localhost:8081/risks/{}", id))
                    .send()
                    .await;

                if let Ok(resp) = response {
                    if let Ok(data) = resp.json::<Risk>().await {
                        risk.set(Some(data));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div>
            <h2>{ "Détails du Risque" }</h2>
            {
                if let Some(risk) = &*risk {
                    html! {
                        <div>
                            <p><strong>{ "Titre : " }</strong>{ &risk.title }</p>
                            <p><strong>{ "Description : " }</strong>{ risk.description.clone().unwrap_or_default() }</p>
                            <p><strong>{ "Réf. externe : " }</strong>{ risk.external_id.clone().unwrap_or_default() }</p>
                            <p><strong>{ "Catégorie : " }</strong>{ risk.category.clone().unwrap_or_default() }</p>
                            <p><strong>{ "Localisation : " }</strong>{ risk.location.clone().unwrap_or_default() }</p>
                            <p><strong>{ "Règlement : " }</strong>{ risk.regulation.clone().unwrap_or_default() }</p>
                            <p><strong>{ "Mesure de contrôle : " }</strong>{ risk.control_measure_id.clone().unwrap_or_default() }</p>
                            <p><strong>{ "Impact : " }</strong>{ risk.impact }</p>
                            <p><strong>{ "Probabilité : " }</strong>{ risk.probability }</p>
                            <p><strong>{ "Score : " }</strong>{ risk.score.unwrap_or(0) }</p>
                            <p><strong>{ "Statut : " }</strong>{ &risk.status }</p>
                        </div>
                    }
                } else {
                    html! { <p>{ "Chargement..." }</p> }
                }
            }
        </div>
    }
}
