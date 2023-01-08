use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SectionProps {
    pub children: Children,
}

#[styled_component]
pub fn Section(SectionProps { children }: &SectionProps) -> Html {
    html! {
        <div class="section">
          { for children.iter() }
        </div>
    }
}
