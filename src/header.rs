//! Header of any card that shows number of passed words and time.

use std::time::Duration;

use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties)]
pub struct HeaderProperties {
    pub time: Duration,
    pub words: u32,
}

#[function_component(CardHeader)]
pub fn card_header(props: &HeaderProperties) -> Html {
    let time = props.time;
    let hours = time.as_secs() / 3600;
    let minutes = time.as_secs() % 3600 / 60;
    let seconds = time.as_secs() % 60;
    let formatted = if hours == 0 {
        format!("{:02}:{:02}", minutes, seconds)
    } else {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    };

    html! {
        <div class={format!("is-flex panel-heading")}>
            <span>{props.words}{" "}{words_ending(props.words)}</span>
            <span>{formatted}</span>
        </div>
    }
}

fn words_ending(num: u32) -> &'static str {
    let ending = num % 10;
    if num >= 5 && num < 20 {
        "слов"
    } else {
        match ending {
            0 => "слов",
            1 => "слово",
            2..=4 => "слова",
            5..=9 => "слов",
            _ => unreachable!(),
        }
    }
}
