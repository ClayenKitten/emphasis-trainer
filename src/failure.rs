//! Failure card provides user a correct word emphasis.

use yew::{html, Callback, Component, Context, Html, Properties};

use crate::model::Word;

#[derive(PartialEq, Properties)]
pub struct FailureProperties {
    pub callback: Callback<crate::Msg>,
    pub word: Word,
    pub seealso: Vec<Word>,
    pub opposite: Vec<Word>,
}

pub enum FailureMsg {
    Continue,
}

pub struct FailureCard;

impl Component for FailureCard {
    type Message = FailureMsg;
    type Properties = FailureProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        FailureCard
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FailureMsg::Continue => ctx.props().callback.emit(crate::Msg::NextWord),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_continue = ctx.link().callback(|_| FailureMsg::Continue);

        html! {
            <>
                <div class="panel-block failure">
                    <b class="failure-explanation">{&ctx.props().word}</b>
                </div>
                {seealso(ctx)}
                <div class="panel-block">
                    <button class="button is-primary is-fullwidth" onclick={on_continue} >
                    {"Продолжить"}
                    </button>
                </div>
            </>
        }
    }
}

fn seealso(ctx: &Context<FailureCard>) -> Html {
    let seealso = &ctx.props().seealso;
    let opposite = &ctx.props().opposite;
    if seealso.is_empty() && opposite.is_empty() {
        html! {}
    } else {
        html! {
            <div class="panel-block failure-seealso">
                {
                    if !seealso.is_empty() {
                        html! {
                            <>
                            <p>{"А также"}</p>
                            <div>
                                {seealso.iter().map(|w| html!(<p>{w}</p>)).collect::<Html>()}
                            </div>
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if !opposite.is_empty() {
                        html! {
                            <>
                            <p>{"Но"}</p>
                            <div>
                                {opposite.iter().map(|w| html!(<p>{w}</p>)).collect::<Html>()}
                            </div>
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}
