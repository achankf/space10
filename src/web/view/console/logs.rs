use yew::prelude::*;

use crate::{
    context::{
        console::{Message, Payload},
        ControllerType,
    },
    parser::command::Error,
    view::console::command::CommandView,
};

#[function_component]
pub fn Logs() -> Html {
    let view_context = use_context::<ControllerType>().expect("no context found");

    let controller = view_context.borrow();
    let elements: Vec<_> = controller
        .console
        .messages
        .iter()
        .map(|message| {
            let message = message.clone();
            let key = message.id;
            html! {<Log {key} {message} />}
        })
        .collect();

    html! {
        <div class="log-container">{for elements}</div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct LogProps {
    message: Message,
}

#[function_component]
fn Log(props: &LogProps) -> Html {
    let Message {
        id,
        buffer,
        payload,
    } = &props.message;

    let (is_correct, element) = match payload {
        Payload::Command(command) => (
            true,
            html! {
              <CommandView command={command.clone()} />
            },
        ),
        Payload::Error(error) => match error {
            Error::Generic(message) => {
                (false, html! { <GenericError message={message.clone()} /> })
            }
            Error::Parse(_) => (
                false,
                html! {
                    <GenericError message={"unable to parse command".to_owned()} />
                },
            ),
        },
    };

    let (correct_class, symbol) = if is_correct {
        ("correct", "✓")
    } else {
        ("incorrect", "✖")
    };

    html! {
        <div class="log">
            <div class={classes!("controls", correct_class)}>
                <div class={classes!("left", "buffer")}>
                    <div>{symbol}</div>
                    <div class="text">{buffer}</div>
                </div>
                <div class="right">
                    <div>{id}</div>
                    <div>{"<>"}</div>
                    <div>{"✖"}</div>
                </div>
            </div>
            <div class="message-container">{element}</div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct GenericErrorProps {
    message: String,
}

#[function_component]
fn GenericError(props: &GenericErrorProps) -> Html {
    let message = &props.message;

    html! {
        <div class="error-message">
            {"Error:"}
            <span>{message}</span>
        </div>
    }
}
