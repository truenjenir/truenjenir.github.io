// /src/app.rs

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
            <Button variant={button::Variant::PRIMARY}/>
            <Button variant={button::Variant::SECONDARY}/>
            <Button variant={button::Variant::ALERT}/>
    }
}
