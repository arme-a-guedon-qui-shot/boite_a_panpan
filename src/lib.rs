use getrandom::fill;
use leptos::prelude::ElementChild;

use leptos::prelude::Update;
use leptos::*;

use leptos_meta::provide_meta_context;

use prelude::{signal, ClassAttribute, CollectView, Get, OnAttribute};

fn random_u32(a: u32, b: u32) -> u32 {
    let mut buf = [0u8; 4];
    fill(&mut buf).unwrap();
    let random_u32 = u32::from_be_bytes(buf);
    (random_u32 % (b - a + 1)) + a
}
fn shuffle_vec<T>(v: &mut Vec<T>) {
    for i in 0..v.len() {
        let j = random_u32(0_u32, i as u32);
        v.swap(j as usize, i);
    }
}
#[component]
pub fn Quotes() -> impl IntoView {
    let mut v = vec![
        (
            "Je m'appelle Lopez, Joe".to_string(),
            "audio/01-jmappelle.mp3".to_string(),
        ),
        (
            "Je suis obligé de faire ça...".to_string(),
            "audio/02-suisoblige.mp3".to_string(),
        ),
        (
            "Qui a pris une volée !".to_string(),
            "audio/03-kapriunevolee.mp3".to_string(),
        ),
        (
            "Et Hofmann de tes morts !".to_string(),
            "audio/04-hehofman.mp3".to_string(),
        ),
        (
            "Lopez de vos morts".to_string(),
            "audio/05-lopezdevosmorts.mp3".to_string(),
        ),
        (
            "Je vais vous enculer vos morts".to_string(),
            "audio/06-jvaisvousenculer.mp3".to_string(),
        ),
        (
            "Tu viens quand tu veux !".to_string(),
            "audio/07-tuviensquand.mp3".to_string(),
        ),
        (
            "il a fallu que je vienne...".to_string(),
            "audio/08-50detesfreres.mp3".to_string(),
        ),
        (
            "Tu vois ce que tu me forces à faire ?".to_string(),
            "audio/09-tuvoiscequtumforce.mp3".to_string(),
        ),
        (
            "Moi je viens tout seul !".to_string(),
            "audio/10-moijevienstoutseul.mp3".to_string(),
        ),
        (
            "Viens on va en finir".to_string(),
            "audio/11-viensonvaenfinir.mp3".to_string(),
        ),
        (
            "Moitié route, sur mon défunt père".to_string(),
            "audio/12-moitieroute.mp3".to_string(),
        ),
        (
            "Et celui qui vient pas, ...".to_string(),
            "audio/13-etcuilaquivientpas.mp3".to_string(),
        ),
        (
            "On se donne rendez-vous !".to_string(),
            "audio/14-onsdonnerdv.mp3".to_string(),
        ),
        (
            "Je veux en finir !".to_string(),
            "audio/15-jeveuxenfinir.mp3".to_string(),
        ),
        (
            "J'ai jamais pris une volée par un homme".to_string(),
            "audio/16-jaijamaispris.mp3".to_string(),
        ),
        ("J'en veux Joe !".to_string(), "17-jenveux.mp3".to_string()),
        (
            "De toutes façons on est en 2012...".to_string(),
            "audio/18-2012.mp3".to_string(),
        ),
        (
            "Vous allez payer".to_string(),
            "audio/19-vousallezpayer.mp3".to_string(),
        ),
        (
            "Le sang de vos morts".to_string(),
            "audio/20-lesang.mp3".to_string(),
        ),
        (
            "La calotte de vos morts".to_string(),
            "audio/21-lacalote.mp3".to_string(),
        ),
        (
            "Viens là toi !".to_string(),
            "audio/22-vienslatoi.mp3".to_string(),
        ),
        (
            "La calotte de ses morts je lui suce".to_string(),
            "audio/23-juisuce.mp3".to_string(),
        ),
        (
            "Le sang de leurs morts il mangera".to_string(),
            "audio/24-lesangdeleurmort.mp3".to_string(),
        ),
        ("Ta femme...".to_string(), "25-tafemme.mp3".to_string()),
        (
            "Ferme ta gueule ©".to_string(),
            "audio/26-fermetagueule.mp3".to_string(),
        ),
        ("J'ai pas peur moi".to_string(), "27-balles.mp3".to_string()),
        (
            "C'est pas deux coups de poings, une tapette...".to_string(),
            "audio/28-2coupsdepoing.mp3".to_string(),
        ),
        (
            "Mon petit frère qui est ici là...".to_string(),
            "audio/29-volees.mp3".to_string(),
        ),
        (
            "C'est la castagne".to_string(),
            "audio/30-castagne.mp3".to_string(),
        ),
        ("?!?!".to_string(), "31-cri.mp3".to_string()),
        (
            "Je fais 75 kilos je fais".to_string(),
            "audio/32-75kilos.mp3".to_string(),
        ),
        ("bonus".to_string(), "audio/ol.mp3".to_string()),
        ("<3".to_string(), "audio/om.mp4".to_string()),
    ];
    v = [&v[..], &v[..]].concat();
    v = [&v[..], &v[..]].concat();
    v = [&v[..], &v[..]].concat();
    v = [&v[..], &v[..]].concat();
    v = [&v[..], &v[..]].concat();
    v = [&v[..], &v[..]].concat();
    shuffle_vec(&mut v);
    let (get_quotes, set_quotes) = signal(v);
    let (get_screamer, set_screamer) = signal(false);

    view! {
        <main>
            <div class="quoteboard">
                {move || {
                    get_quotes
                        .get()
                        .into_iter()
                        .map(|q| {

                            view! {
                                <button on:click=move |_| {
                                    let path = format!("public/{}", q.1.as_str());
                                    if path.ends_with(".mp3") {
                                        let audio = web_sys::HtmlAudioElement::new_with_src(
                                                path.as_str(),
                                            )
                                            .expect("audio introuvable");
                                        let _ = audio.play().expect("impossible de jouer l'audio");
                                    } else if path.ends_with(".mp4") {
                                        set_screamer
                                            .update(|old| {
                                                *old = true;
                                            })
                                    }
                                }>

                                    <a href="#">{q.0}</a>

                                </button>
                            }
                        })
                        .collect_view()
                }}

            </div>
            {move || {
                if get_screamer.get() {
                    Some(

                        view! {
                            <video
                                class="fullScreenVideo"
                                autoplay=true
                                on:ended=move |_| {
                                    set_screamer
                                        .update(|old| {
                                            *old = false;
                                        })
                                }
                            >
                                <source src="public/video/om.mp4" type="video/mp4" />
                            </video>
                        },
                    )
                } else {
                    None
                }
            }}
        </main>
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
