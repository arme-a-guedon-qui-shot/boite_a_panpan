use leptos::prelude::{Effect, ElementChild, Update};
use leptos::*;

use leptos_meta::provide_meta_context;
use logging::log;
use prelude::{signal, ClassAttribute, CollectView, Get, OnAttribute};
use rand::seq::SliceRandom;

#[component]
pub fn Quotes() -> impl IntoView {
    let (get_quotes, set_quotes) = signal(vec![
        (
            "Je m'appelle Lopez, Joe".to_string(),
            "01-jmappelle.mp3".to_string(),
        ),
        (
            "Je suis obligé de faire ça...".to_string(),
            "02-suisoblige.mp3".to_string(),
        ),
        (
            "Qui a pris une volée !".to_string(),
            "03-kapriunevolee.mp3".to_string(),
        ),
        (
            "Et Hofmann de tes morts !".to_string(),
            "04-hehofman.mp3".to_string(),
        ),
        (
            "Lopez de vos morts".to_string(),
            "05-lopezdevosmorts.mp3".to_string(),
        ),
        (
            "Je vais vous enculer vos morts".to_string(),
            "06-jvaisvousenculer.mp3".to_string(),
        ),
        (
            "Tu viens quand tu veux !".to_string(),
            "07-tuviensquand.mp3".to_string(),
        ),
        (
            "il a fallu que je vienne...".to_string(),
            "08-50detesfreres.mp3".to_string(),
        ),
        (
            "Tu vois ce que tu me forces à faire ?".to_string(),
            "09-tuvoiscequtumforce.mp3".to_string(),
        ),
        (
            "Moi je viens tout seul !".to_string(),
            "10-moijevienstoutseul.mp3".to_string(),
        ),
        (
            "Viens on va en finir".to_string(),
            "11-viensonvaenfinir.mp3".to_string(),
        ),
        (
            "Moitié route, sur mon défunt père".to_string(),
            "12-moitieroute.mp3".to_string(),
        ),
        (
            "Et celui qui vient pas, ...".to_string(),
            "13-etcuilaquivientpas.mp3".to_string(),
        ),
        (
            "On se donne rendez-vous !".to_string(),
            "14-onsdonnerdv.mp3".to_string(),
        ),
        (
            "Je veux en finir !".to_string(),
            "15-jeveuxenfinir.mp3".to_string(),
        ),
        (
            "J'ai jamais pris une volée par un homme".to_string(),
            "16-jaijamaispris.mp3".to_string(),
        ),
        ("J'en veux Joe !".to_string(), "17-jenveux.mp3".to_string()),
        (
            "De toutes façons on est en 2012...".to_string(),
            "18-2012.mp3".to_string(),
        ),
        (
            "Vous allez payer".to_string(),
            "19-vousallezpayer.mp3".to_string(),
        ),
        (
            "Le sang de vos morts".to_string(),
            "20-lesang.mp3".to_string(),
        ),
        (
            "La calotte de vos morts".to_string(),
            "21-lacalote.mp3".to_string(),
        ),
        (
            "Viens là toi !".to_string(),
            "22-vienslatoi.mp3".to_string(),
        ),
        (
            "La calotte de ses morts je lui suce".to_string(),
            "23-juisuce.mp3".to_string(),
        ),
        (
            "Le sang de leurs morts il mangera".to_string(),
            "24-lesangdeleurmort.mp3".to_string(),
        ),
        ("Ta femme...".to_string(), "25-tafemme.mp3".to_string()),
        (
            "Ferme ta gueule ©".to_string(),
            "26-fermetagueule.mp3".to_string(),
        ),
        ("J'ai pas peur moi".to_string(), "27-balles.mp3".to_string()),
        (
            "C'est pas deux coups de poings, une tapette...".to_string(),
            "28-2coupsdepoing.mp3".to_string(),
        ),
        (
            "Mon petit frère qui est ici là...".to_string(),
            "29-volees.mp3".to_string(),
        ),
        (
            "C'est la castagne".to_string(),
            "30-castagne.mp3".to_string(),
        ),
        ("?!?!".to_string(), "31-cri.mp3".to_string()),
        (
            "Je fais 75 kilos je fais".to_string(),
            "32-75kilos.mp3".to_string(),
        ),
        ("bonus".to_string(), "ol.mp3".to_string()),
    ]);
    (move || {
        let mut vec_clone = get_quotes.get().clone();
        vec_clone.shuffle(&mut rand::rng());
        set_quotes.update(|old| *old = vec_clone);
    })();

    view! {
        <div class="quoteboard">
            {move || {
                get_quotes
                    .get()
                    .into_iter()
                    .map(|q| {

                        view! {
                            <button on:click=move |_| {
                                log!("{}",q.1.as_str());
                                let audio = web_sys::HtmlAudioElement::new_with_src(
                                        format!("public/audio/{}", q.1.as_str()).as_str(),
                                    )
                                    .expect("audio introuvable");
                                let _ = audio.play().expect("impossible de jouer l'audio");
                            }>

                                <a href="#">{q.0}</a>

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
