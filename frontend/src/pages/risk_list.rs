use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use web_sys::window;
use yew_router::prelude::*;
use crate::Route;

#[derive(Deserialize, Debug, Clone)]
pub struct Risk {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub impact: i32,
    pub probability: i32,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub score: Option<i32>,
    pub external_id: Option<String>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub regulation: Option<String>,
    pub control_measure_id: Option<String>,
}

#[function_component(RiskList)]
pub fn risk_list() -> Html {
    let risks = use_state(|| vec![]);
    let loading = use_state(|| true);
    let navigator = use_navigator().unwrap();

    let delete_risk = {
        let risks = risks.clone();
        Callback::from(move |id: String| {
            let risks = risks.clone();
            spawn_local(async move {
                let res = Request::delete(&format!("http://localhost:8081/risks/{}", id))
                    .send()
                    .await;
                if let Ok(response) = res {
                    if response.ok() {
                        let updated: Vec<Risk> = risks.iter().cloned().filter(|r: &Risk| r.id != id).collect();
                        risks.set(updated);
                        if let Some(win) = window() {
                            let _ = win.alert_with_message("Risque supprimé avec succès !");
                        }
                    } else {
                        console::log_1(&"Échec de la suppression".into());
                    }
                }
            });
        })
    };

    {
        let risks = risks.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("http://localhost:8081/risks")
                    .send()
                    .await
                {
                    Ok(resp) => {
                        match resp.json::<Vec<Risk>>().await {
                            Ok(data) => risks.set(data),
                            Err(err) => console::log_1(&format!("Erreur JSON: {:?}", err).into()),
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&format!("Erreur: {:?}", err).into());
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div>
            <h2>{ "Liste des Risques" }</h2>
            if *loading {
                <p>{ "Chargement en cours..." }</p>
            } else {
                <ul>
                    { for risks.iter().cloned().map(|risk| html! {
                        <li key={risk.id.clone()}>
                            <strong>{ format!("{} [{}]", &risk.title, risk.status) }</strong>
                            <p>{ format!("Réf. externe : {}", risk.external_id.clone().unwrap_or_default()) }</p>
                            <p>{ format!("Catégorie : {}", risk.category.clone().unwrap_or_default()) }</p>
                            <p>{ format!("Localisation : {}", risk.location.clone().unwrap_or_default()) }</p>
                            <p>{ format!("Règlement : {}", risk.regulation.clone().unwrap_or_default()) }</p>
                            <p>{ format!("Mesure de contrôle : {}", risk.control_measure_id.clone().unwrap_or_default()) }</p>
                            <p>{ format!("Impact : {} | Probabilité : {} | Score : {}", risk.impact, risk.probability, risk.score.unwrap_or(0)) }</p>
                            <p>{ format!("Description : {}", risk.description.clone().unwrap_or_default()) }</p>
                            <button onclick={
                                let navigator = navigator.clone();
                                let id = risk.id.clone();
                                Callback::from(move |_| navigator.push(&Route::EditRisk { id: id.clone() }))
                            }>
                                { "✏ Modifier" }
                            </button>
                            <button onclick={
                                let navigator = navigator.clone();
                                let id = risk.id.clone();
                                Callback::from(move |_| navigator.push(&Route::ViewRisk { id: id.clone() }))
                            }>
                                { "🔍 Voir les détails" }
                            </button>
                            <button onclick={
                                let delete_risk = delete_risk.clone();
                                let id = risk.id.clone();
                                Callback::from(move |_| {
                                    if web_sys::window().unwrap().confirm_with_message("Voulez-vous vraiment supprimer ce risque ?").unwrap_or(false) {
                                        delete_risk.emit(id.clone());
                                    }
                                })
                            }>
                                { "🗑 Supprimer" }
                            </button>
                        </li>
                    }) }
                </ul>
            }
        </div>
    }
}
