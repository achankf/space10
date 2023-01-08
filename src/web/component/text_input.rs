use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TextInputProps {
    pub name: String,
    pub handle: UseStateHandle<String>,
    pub required: Option<bool>,
}

#[function_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    let TextInputProps {
        name,
        handle,
        required,
    } = props;

    let oninput = {
        let handle = props.handle.clone();

        move |e: InputEvent| {
            let input = e
                .target_dyn_into::<HtmlInputElement>()
                .expect("should be HtmlInputElement");
            handle.set(input.value());
        }
    };

    let name = name.to_owned();
    let value = handle.to_string();
    let required = required.unwrap_or_default();

    html! {
        <input type="text" {name} {oninput} {value} {required} />
    }
}
