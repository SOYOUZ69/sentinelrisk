use yew::prelude::*;
use gloo_net::http::Request;
use serde::Serialize;
use web_sys::HtmlInputElement;

#[derive(Serialize, Default, Clone)]
struct NewRisk {
    title: String,
    description: String,
    impact: i32,
    probability: i32,
    status: String,
    external_id: String,
    category: String,
    location: String,
    regulation: String,
    control_measure_id: String,
}

#[function_component(AddRisk)]
pub fn add_risk() -> Html {
    let form = use_state(NewRisk::default);
    let message = use_state(|| None as Option<String>);

    let on_submit = {
        let form = form.clone();
        let message = message.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let risk = (*form).clone();
            let message = message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("http://localhost:8081/risks")
                    .header("Content-Type", "application/json")
                    .json(&risk)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(response) => {
                        if response.ok() {
                            message.set(Some("Risque ajouté avec succès.".to_string()));
                        } else {
                            message.set(Some(format!("Erreur du serveur: {}", response.status())));
                        }
                    }
                    Err(_) => {
                        message.set(Some("Erreur réseau : impossible d'ajouter le risque.".to_string()));
                    }
                }
            });
        })
    };

    let on_input = |field: &'static str| {
        let form = form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut data = (*form).clone();
            match field {
                "title" => data.title = input.value(),
                "description" => data.description = input.value(),
                "impact" => data.impact = input.value().parse().unwrap_or(0),
                "probability" => data.probability = input.value().parse().unwrap_or(0),
                "status" => data.status = input.value(),
                "external_id" => data.external_id = input.value(),
                "category" => data.category = input.value(),
                "location" => data.location = input.value(),
                "regulation" => data.regulation = input.value(),
                "control_measure_id" => data.control_measure_id = input.value(),
                _ => (),
            }
            form.set(data);
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <h2>{ "Ajouter un Risque" }</h2>

            <input type="text" placeholder="Titre" oninput={on_input("title")} />
            <input type="text" placeholder="Description" oninput={on_input("description")} />
            <input type="number" placeholder="Impact (1-5)" oninput={on_input("impact")} />
            <input type="number" placeholder="Probabilité (1-5)" oninput={on_input("probability")} />
            <input type="text" placeholder="Statut (Nouveau/Accepté/...)" oninput={on_input("status")} />
            <input type="text" placeholder="Réf. Externe" oninput={on_input("external_id")} />
            <input type="text" placeholder="Catégorie" oninput={on_input("category")} />
            <input type="text" placeholder="Localisation" oninput={on_input("location")} />
            <input type="text" placeholder="Règlement" oninput={on_input("regulation")} />
            <input type="text" placeholder="Mesure de contrôle" oninput={on_input("control_measure_id")} />
            <button type="submit">{ "Ajouter" }</button>

            if let Some(msg) = &*message {
                <p>{ msg }</p>
            }
        </form>
    }
}
