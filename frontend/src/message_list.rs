use common::ChatMessage;
use yew::prelude::*;

// #[derive(Properties, PartialEq)]
// pub struct Props {
//     pub messages: Vec<ChatMessage>,
// }

#[function_component(MesasgeList)]
pub fn message_list(props: &Props) -> Html {
    html!(
      <div class="list-group">
      {
        props.messages.iter().map(|m| html!{
            <div class="list-group-item list-group-item-action">
                <div class="d-flex w-100 justify-content-between">
                    <h5>{m.author.clone()}</h5>
                    <span>{m.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</span>
                </div>
                {m.message.clone()}
            </div>
        }).collect::<Html>()
      }
      </div>
    )
}
