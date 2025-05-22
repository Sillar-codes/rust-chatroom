use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub users: Vec<String>,
}

#[function_component(UserList)]
pub fn user_list(props: &Props) -> Html {
    html!(
      <>
      <h4>{"UserList"}</h4>
        <ul>
        {
          props.users.iter().map(|user| html!(
            <li>{user}</li>
          )).collect::<Html>()
        }
        </ul>
      </>
    )
}
