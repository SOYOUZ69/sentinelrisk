use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

#[derive(Deserialize, Debug, Clone)]
pub struct Risk {
    pub id: i32,
    pub title: String,
    pub severity: String,
    pub status: String,
}

#[function_component(RiskList)]
pub fn risk_list() -> Html {
    let risks = use_state(|| vec![]);
    let loading = use_state(|| true);

    {
        let risks = risks.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("http://localhost:8080/risks")
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
                        <li key={risk.id.to_string()}>
                            <strong>{ &risk.title }</strong>
                            { format!(" - Gravité: {} | Statut: {}", risk.severity, risk.status) }
                        </li>
                    }) }
                </ul>
            }
        </div>
    }
}
