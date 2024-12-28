use leptos::{
    ev::{MouseEvent, SubmitEvent},
    html::{self},
    prelude::*,
};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    let log2_count = Signal::derive(move || ((count.get() as f32).log2() * 100.0) as i32);

    view! {
        <button on:click=move |_| *set_count.write() += 1>Increase by 1</button>
        <button on:click=move |_| *set_count.write() -= 1>Decrease by 1</button>
        <button on:click=move |_| *set_count.write() *= 2>Double</button>
        <p class=("red", move || count.get() % 2 == 1)>"Current value : " {count}</p>
        <p>"Double current value : " {move || count.get() * 2}</p>
        <div>
            <ProgressBar value=count />
            <ProgressBar max=10 value=count />
            <ProgressBar max=800 value=log2_count />
        </div>
        <ButtonArray />
        <ButtonArray length=2 />

        <TextField />
        <TextForm />

        <Selector length=6 />

        <NumberInput />

        <ContextDemo />
    }
}

#[component]
fn TextField() -> impl IntoView {
    let (name, set_name) = signal("Jane Doe".to_string());

    view! {
        <input
            type="text"
            on:input:target=move |ev| { set_name.set(ev.target().value()) }
            prop:value=name
        />
        <p>"Name is " {name}</p>
    }
}

#[component]
fn TextForm() -> impl IntoView {
    let (name, set_name) = signal("Jane Doe".to_string());
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should have been mounted already")
            .value();
        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="Submit" />
        </form>
        <p>"Name is " {name}</p>
    }
}

#[component]
fn ButtonArray(#[prop(default = 5)] length: u16) -> impl IntoView {
    let counters = (1..=length).map(|_| RwSignal::new(0));

    view! {
        <div>
            {counters
                .map(|signal| {
                    view! {
                        <button on:click=move |_| {
                            *signal.write() += 1;
                        }>"Click me: " {signal}</button>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn ProgressBar(
    /// Value to display on the progress bar
    #[prop(into)]
    value: Signal<i32>,
    /// Maximum value of the progress bar
    #[prop(default = 50)]
    max: u16,
) -> impl IntoView {
    view! {
        <progress max=max value=value />
        <br />
    }
}

#[component]
fn Selector(#[prop()] length: u16) -> impl IntoView {
    let (selected, set_selected) = signal(1u16);

    let options_view = (1..=length)
        .map(|idx| {
            view! { <option value=idx>{idx}</option> }
        })
        .collect_view();

    let reset_selector = move |_: MouseEvent| {
        set_selected.set(1u16);
    };
    let increment_selector = move |_: MouseEvent| {
        set_selected.update(|idx| *idx = (*idx % length) + 1);
    };

    view! {
        <select
            prop:value=selected
            on:change:target=move |ev| {
                set_selected.set(ev.target().value().parse().unwrap());
            }
        >
            {options_view}
        </select>
        <button on:click=reset_selector>"Reset selector"</button>
        <button on:click=increment_selector>"Increment selector"</button>
    }
}

#[component]
fn NumberInput() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));

    let success_text = view! { <p>"You entered "{value}</p> };

    let error_message = |errors: ArcRwSignal<Errors>| {
        view! {
            <div class="error">
                <p>"Not a number! Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect::<Vec<_>>()
                    }}
                </ul>
            </div>
        }
    };

    view! {
        <h1>"Error handling"</h1>
        <input
            type="text"
            on:input:target=move |ev| { set_value.set(ev.target().value().parse()) }
        />
        <ErrorBoundary fallback=error_message>{success_text}</ErrorBoundary>
    }
}

#[component]
fn ContextDemo() -> impl IntoView {
    let (toggled, set_toggled) = signal(false);
    provide_context(set_toggled);

    view! {
        <ContextDemoButton />
        <ContextDemoText toggled=toggled />
    }
}

#[component]
fn ContextDemoText(#[prop()] toggled: ReadSignal<bool>) -> impl IntoView {
    let fallback_text = || view! { "Nope" };
    let success_text = || view! { "Yep" };

    view! {
        <p>
            "Toggled? "<Show when=move || { toggled.get() } fallback=fallback_text>
                {success_text}
            </Show>
        </p>
    }
}

#[component]
fn ContextDemoButton() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("No setter provided");
    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}
