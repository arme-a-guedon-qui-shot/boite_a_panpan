use leptos::prelude::ElementChild;
use leptos::*;

use leptos_meta::provide_meta_context;
use logging::log;
use prelude::{
    signal, ClassAttribute, CollectView, Get, GlobalAttributes, OnAttribute, StyleAttribute,
};

#[component]
pub fn Quotes() -> impl IntoView {
    let (get_quotes, _set_quotes) = signal(vec![
        (
            "Je m'appelle Lopez, Joe".to_string(),
            "01-jmappelle.wav".to_string(),
        ),
        (
            "Je suis obligé de faire ça...".to_string(),
            "02-suisoblige.wav".to_string(),
        ),
        (
            "Qui a pris une volée !".to_string(),
            "03-kapriunevolee.wav".to_string(),
        ),
        (
            "Et Hofmann de tes morts !".to_string(),
            "04-hehofman.wav".to_string(),
        ),
        (
            "Lopez de vos morts".to_string(),
            "05-lopezdevosmorts.wav".to_string(),
        ),
        (
            "Je vais vous enculer vos morts".to_string(),
            "06-jvaisvousenculer.wav".to_string(),
        ),
        (
            "Tu viens quand tu veux !".to_string(),
            "07-tuviensquand.wav".to_string(),
        ),
        (
            "il a fallu que je vienne...".to_string(),
            "08-50detesfreres.wav".to_string(),
        ),
        (
            "Tu vois ce que tu me forces à faire ?".to_string(),
            "09-tuvoiscequtumforce.wav".to_string(),
        ),
        (
            "Moi je viens tout seul !".to_string(),
            "10-moijevienstoutseul.wav".to_string(),
        ),
        (
            "Viens on va en finir".to_string(),
            "11-viensonvaenfinir.wav".to_string(),
        ),
        (
            "Moitié route, sur mon défunt père".to_string(),
            "12-moitieroute.wav".to_string(),
        ),
        (
            "Et celui qui vient pas, ...".to_string(),
            "13-etcuilaquivientpas.wav".to_string(),
        ),
        (
            "On se donne rendez-vous !".to_string(),
            "14-onsdonnerdv.wav".to_string(),
        ),
        (
            "Je veux en finir !".to_string(),
            "15-jeveuxenfinir.wav".to_string(),
        ),
        (
            "J'ai jamais pris une volée par un homme".to_string(),
            "16-jaijamaispris.wav".to_string(),
        ),
        ("J'en veux Joe !".to_string(), "17-jenveux.wav".to_string()),
        (
            "De toutes façons on est en 2012...".to_string(),
            "18-2012.wav".to_string(),
        ),
        (
            "Vous allez payer".to_string(),
            "19-vousallezpayer.wav".to_string(),
        ),
        (
            "Le sang de vos morts".to_string(),
            "20-lesang.wav".to_string(),
        ),
        (
            "La calotte de vos morts".to_string(),
            "21-lacalote.wav".to_string(),
        ),
        (
            "Viens là toi !".to_string(),
            "22-vienslatoi.wav".to_string(),
        ),
        (
            "La calotte de ses morts je lui suce".to_string(),
            "23-juisuce.wav".to_string(),
        ),
        (
            "Le sang de leurs morts il mangera".to_string(),
            "24-lesangdeleurmort.wav".to_string(),
        ),
        ("Ta femme...".to_string(), "25-tafemme.wav".to_string()),
        (
            "Ferme ta gueule ©".to_string(),
            "26-fermetagueule.wav".to_string(),
        ),
        ("J'ai pas peur moi".to_string(), "27-balles.wav".to_string()),
        (
            "C'est pas deux coups de poings, une tapette...".to_string(),
            "28-2coupsdepoing.wav".to_string(),
        ),
        (
            "Mon petit frère qui est ici là...".to_string(),
            "29-volees.wav".to_string(),
        ),
        (
            "C'est la castagne".to_string(),
            "30-castagne.wav".to_string(),
        ),
        ("?!?!".to_string(), "31-cri.wav".to_string()),
        (
            "Je fais 75 kilos je fais".to_string(),
            "32-75kilos.wav".to_string(),
        ),
    ]);
    view! {
        <div class="quoteboard">
            {move || {
                get_quotes
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, q)| {

                        view! {
                            <button  on:click=move |_| {
                                log!("{}",q.1.as_str());
                                let audio = web_sys::HtmlAudioElement::new_with_src(
                                        format!("public/audio/{}", q.1.as_str()).as_str(),
                                    )
                                    .expect("audio introuvable");
                                let _ = audio.play().expect("impossible de jouer l'audio");
                            }>
                                <div>
                                    <a id=i.to_string() href="#">{q.0}</a>
                                </div>

                            </button>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

#[component]
pub fn BoiteAPanpan() -> impl IntoView {
    provide_meta_context();
    view! {
        <main>
            <Quotes />
        </main>
    }
}
