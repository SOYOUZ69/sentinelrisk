use yew::prelude::*;
use gloo_net::http::Request;
use serde::Serialize;
use web_sys::HtmlInputElement;

#[derive(Serialize, Default, Clone)]
struct NewRisk {
    title: String,
    description: String,
    severity: String,
    status: String,
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
                let resp = Request::post("http://localhost:8080/risks")
                    .header("Content-Type", "application/json")
                    .json(&risk)
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(_) => message.set(Some("Risque ajouté avec succès.".to_string())),
                    Err(_) => message.set(Some("Échec de l'ajout du risque.".to_string())),
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
                "severity" => data.severity = input.value(),
                "status" => data.status = input.value(),
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
            <input type="text" placeholder="Gravité (Faible/Moyenne/Critique)" oninput={on_input("severity")} />
            <input type="text" placeholder="Statut (Nouveau/Accepté/...)" oninput={on_input("status")} />
            <button type="submit">{ "Ajouter" }</button>

            if let Some(msg) = &*message {
                <p>{ msg }</p>
            }
        </form>
    }
}
