use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use std::rc::Rc;

use crate::Route;

#[derive(Serialize, Deserialize, Clone, Default)]
struct Risk {
    #[serde(skip_serializing)]
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
}

#[function_component(EditRisk)]
pub fn edit_risk() -> Html {
    let navigator = use_navigator().unwrap();
    let route = use_route::<Route>().unwrap();

    let rc_id = Rc::new(if let Route::EditRisk { id } = route {
        id.clone()
    } else {
        "".to_string()
    });

    let risk = use_state(Risk::default);
    {
        let risk = risk.clone();
        let rc_id = rc_id.clone();
        use_effect_with(rc_id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get(&format!("http://localhost:8081/risks/{}", rc_id.clone()))
                .send()
                .await;
            
            if let Ok(resp) = response {
                if let Ok(fetched) = resp.json::<Risk>().await {
                    risk.set(fetched);
                }
            }
            });
            || ()
        });
    }

    let form = risk.clone();
    let oninput = |field: &'static str| {
        let form = form.clone();
        Callback::from(move |e: InputEvent| {
            let mut updated = (*form).clone();
            let value: String = e.target_unchecked_into::<HtmlInputElement>().value();
            match field {
                "title" => updated.title = value,
                "description" => updated.description = Some(value),
                "impact" => updated.impact = value.parse().unwrap_or(0),
                "probability" => updated.probability = value.parse().unwrap_or(0),
                "status" => updated.status = value,
                "external_id" => updated.external_id = Some(value),
                "category" => updated.category = Some(value),
                "location" => updated.location = Some(value),
                "regulation" => updated.regulation = Some(value),
                "control_measure_id" => updated.control_measure_id = Some(value),
                _ => {}
            }
            form.set(updated);
        })
    };

    let onsubmit = {
        let risk = risk.clone();
        let navigator = navigator.clone();
        let rc_id_submit = rc_id.clone();
    
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let data = (*risk).clone();
            let navigator = navigator.clone(); // <-- cloner ici pour éviter FnOnce
            let rc_id_inner = (*rc_id_submit).clone();
    
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::put(&format!("http://localhost:8081/risks/{}", rc_id_inner))
                    .header("Content-Type", "application/json")
                    .json(&data)
                    .unwrap()
                    .send()
                    .await;
    
                if let Ok(response) = res {
                    if response.ok() {
                        navigator.push(&Route::Risks);
                    }
                }
            });
        })
    };

    html! {
        <form onsubmit={onsubmit}>
            <h2>{ "Modifier un Risque" }</h2>
            <input type="text" value={risk.title.clone()} oninput={oninput("title")} />
            <input type="text" value={risk.description.clone().unwrap_or_default()} oninput={oninput("description")} />
            <input type="number" value={risk.impact.to_string()} oninput={oninput("impact")} />
            <input type="number" value={risk.probability.to_string()} oninput={oninput("probability")} />
            <input type="text" value={risk.status.clone()} oninput={oninput("status")} />
            <input type="text" value={risk.external_id.clone().unwrap_or_default()} oninput={oninput("external_id")} />
            <input type="text" value={risk.category.clone().unwrap_or_default()} oninput={oninput("category")} />
            <input type="text" value={risk.location.clone().unwrap_or_default()} oninput={oninput("location")} />
            <input type="text" value={risk.regulation.clone().unwrap_or_default()} oninput={oninput("regulation")} />
            <input type="text" value={risk.control_measure_id.clone().unwrap_or_default()} oninput={oninput("control_measure_id")} />
            <button type="submit">{ "Enregistrer" }</button>
        </form>
    }
}
