use leptos::*;
use styled::style;

#[component]
pub fn MyComponent() -> impl IntoView {

    let styles = style!(
      div {
        background-color: red;
        color: white;
      }
    );

    styled::view! {
        styles,
        <div>"This text should be red with white text."</div>
    }
}

#[component]
pub fn AnotherComponent() -> impl IntoView {

    // note were using a plain div selector and it wont clash with MyComponent's div style!
    let styles = style!(
      div {
        background-color: blue;
        color: gray;
      }
    );

    styled::view! {
        
        styles,
        <div>"This text should be blue with gray text."</div>
    }
}

fn main() {  leptos::mount_to_body(|| view! { <App/> })}
