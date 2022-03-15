use std::time::Duration;

use gloo::console;
use gloo::timers::callback::Interval;
use model::{Model, Variant, Word, ParseError};
use question::QuestionCard;
use yew::prelude::*;

use crate::failure::{FailureCard, FailureProperties};
use crate::header::CardHeader;
use crate::question::QuestionProperties;

mod failure;
mod header;
mod model;
mod question;
mod util;

pub enum Msg {
    Tick,
    Success(Word),
    Failure(Word),
    NextWord,
}

struct App {
    model: Model,
    header_color: &'static str,
    time: Duration,
    words: u32,
    stage: Stage,
}

#[derive(Debug, Clone)]
enum Stage {
    Question(Word, Vec<Variant>),
    Failure(Word),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_: ()| Msg::Tick);
        Interval::new(1000, move || {
            callback.emit(());
        })
        .forget();
        let (model, errors) = Model::new();
        log_errors(errors);
        let word = model.next();
        let variants = word.variants();
        App {
            model,
            header_color: "",
            time: Duration::ZERO,
            words: 0,
            stage: Stage::Question(word, variants),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.time += Duration::from_secs(1);
            }
            Msg::Failure(word) => {
                self.header_color = "is-danger";
                self.stage = Stage::Failure(word);
            }
            Msg::Success(_word) => {
                self.words += 1;
                ctx.link().send_message(Msg::NextWord);
            }
            Msg::NextWord => {               
                self.header_color = "";
                let word = self.model.next();
                let variants = word.variants();
                self.stage = Stage::Question(word, variants);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(|msg| msg);

        let card = match self.stage.clone() {
            Stage::Question(word, variants) => {
                let props = QuestionProperties {
                    callback,
                    word,
                    variants,
                };
                html!(<QuestionCard ..props/>)
            }
            Stage::Failure(word) => {
                let (seealso, opposite) = if word.group.is_some() {
                    (self.model.seealso(&word), self.model.opposite(&word))
                } else {
                    (Vec::new(), Vec::new())
                };
                let props = FailureProperties { callback, word, seealso, opposite };
                html!(<FailureCard ..props/>)
            }
        };

        html! {
            <>
                <main>
                    <section class={format!("mycard panel {}", self.header_color)}>
                        <CardHeader time={self.time} words={self.words}/>
                        {card}
                    </section>
                </main>
                <footer>
                    <div class="content has-text-centered">
                        <p>
                            <strong>{"Emphasis trainer"}</strong>{" by "}<a href="https://lowlevelvirtualman.com">{"LowLevelVirtualMan"}</a>{"."}
                            {"The source code is licensed "}<a href="http://opensource.org/licenses/mit-license.php">{"MIT"}</a>{"."}
                        </p>
                    </div>
                </footer>
            </>
        }
    }
}

fn log_errors(errors: Vec<ParseError>) {
    match errors.len() {
        0 => console::log!("Word data loaded with no errors."),
        n => {
            console::group!(collapsed format!("Word data loaded with {n} errors."));
            for error in errors {
                console::error!(error.to_string());
            }
            console::group_end!();
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
