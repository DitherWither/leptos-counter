use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! {
        cx,
        <h1>"Counter"</h1>

        <div class="hcenter vcenter">
            <div class="margin-bottom-2">
                <button on:click=move |_| set_count.update(|count| *count += 1) class="counter-btn">
                    "+"
                </button>
            
                <p class="counter-text">{count}</p>
            
                <button on:click=move |_| set_count.update(|count| *count -= 1) class="counter-btn">
                    "-"
                </button>
            </div>
            <button on:click=move |_| set_count.set(0) class="btn-secondary">
            "Reset"
            </button>
        </div>
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
