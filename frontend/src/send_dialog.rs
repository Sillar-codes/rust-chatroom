use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[function_component(SendDialog)]
pub fn send_dialog(props: &Props) -> Html {
    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();
    let new_username_handle = use_state(|| props.username.clone());
    let new_username = (*new_username_handle).clone();
    let is_editing_username_handle: UseStateHandle<bool> = use_state(bool::default);
    let is_editing_username = (*is_editing_username_handle).clone();

    let cloned_new_message_handle = new_message_handle.clone();
    let on_message_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(textarea) = target {
            cloned_new_message_handle.set(textarea.value());
        }
    });

    let cloned_new_message = new_message.clone();
    let callback = props.send_message_callback.clone();
    let on_button_click = Callback::from(move |_: MouseEvent| {
        callback.emit(cloned_new_message.clone());
        new_message_handle.set("".to_string());
    });

    let cloned_is_editing_handle = is_editing_username_handle.clone();
    let on_username_change_click = Callback::from(move |_: MouseEvent| {
        cloned_is_editing_handle.set(true);
    });

    let cloned_is_editing_handle = is_editing_username_handle.clone();
    let callback = props.change_username_callback.clone();
    let cloned_new_username = new_username.clone();
    let on_username_apply_click = Callback::from(move |_: MouseEvent| {
        callback.emit(cloned_new_username.clone());
        cloned_is_editing_handle.set(false);
    });

    let on_username_cancel_click = Callback::from(move |_: MouseEvent| {
        is_editing_username_handle.set(false);
    });

    let on_username_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            new_username_handle.set(input.value());
        }
    });

    html!(
      <div class="input-group">
        if is_editing_username {
            <input type="text" class="form-control" value={new_username} onchange={on_username_change} />
            <button class="btn btn-secondary" onclick={on_username_apply_click}>{"Apply"}</button>
            <button class="btn btn-danger" onclick={on_username_cancel_click}>{"Cancel"}</button>
        } else {
            <button class="btn btn-secondary" onclick={on_username_change_click}>{props.username.clone()}</button>
        }
        <span class="input-group-text">{"Your message:"}</span>
        <textarea class="form-control" onchange={on_message_change} value={new_message}></textarea>
        <button class="btn btn-primary" type="submit" onclick={on_button_click}>{"Send"}</button>
      </div>
    )
}
