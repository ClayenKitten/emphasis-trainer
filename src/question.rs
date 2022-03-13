//! Question card provides user with a word and variants of emphasis.
//!
//! Correct answer moves them to `success card`, incorrect one moves them to `failure card`.

use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use yew::{html, Callback, Component, Context, Html, MouseEvent, Properties};

use crate::model::{Variant, Word};

#[derive(PartialEq, Properties)]
pub struct QuestionProperties {
    pub callback: Callback<crate::Msg>,
    pub word: Word,
    pub variants: Vec<Variant>,
}

pub enum QuestionMsg {
    Answer { emphasis: usize },
    Continue,
}

pub struct QuestionCard;

impl Component for QuestionCard {
    type Message = QuestionMsg;
    type Properties = QuestionProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        QuestionCard
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            QuestionMsg::Answer { emphasis } => {
                let word = ctx.props().word.clone();
                let msg = if word.emphasis == emphasis {
                    crate::Msg::Success(word)
                } else {
                    crate::Msg::Failure(word)
                };
                ctx.props().callback.emit(msg);
            }
            QuestionMsg::Continue => ctx
                .props()
                .callback
                .emit(crate::Msg::Failure(ctx.props().word.clone())),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_continue = ctx.link().callback(|_| QuestionMsg::Continue);

        html! {
            <>
                <div class="panel-block">
                    {ctx.props().variants.iter().map(|v| render_variant(ctx, v)).collect::<Html>()}
                </div>
                <div class="panel-block">
                    <button class="button is-danger is-outlined is-fullwidth" onclick={on_continue} >
                    {"Пропустить"}
                    </button>
                </div>
            </>
        }
    }
}

fn render_variant(ctx: &Context<QuestionCard>, variant: &Variant) -> Html {
    let onclick = ctx.link().batch_callback(|e: MouseEvent| {
        e.target()
            .and_then(|t| t.dyn_into::<HtmlButtonElement>().ok())
            .map(|b| {
                let _ = b.blur();
                b.id()
            })
            .and_then(|id| {
                if id.is_empty() || !id.contains(':') {
                    None
                } else {
                    Some(id)
                }
            })
            .map(|id| id.split_once(':').unwrap().1.to_owned())
            .and_then(|n| n.parse().ok())
            .map(|emphasis: usize| QuestionMsg::Answer { emphasis })
    });
    let id = format!("variant:{}", variant.emphasis);
    html!(<button id={id} class="button is-link is-outlined" {onclick}>{&variant}</button>)
}
