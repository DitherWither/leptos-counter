use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! {
        cx,
        <h1>"Counter"</h1>

        <button on:click=move |_| set_count.update(|count| *count += 1)>
            "+"
        </button>
        
        <p>{count}</p>

        <button on:click=move |_| set_count.update(|count| *count -= 1)>
            "-"
        </button>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <App/>
        }
    })
}
