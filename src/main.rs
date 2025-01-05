use async_components::{AsyncLoader, SuspendedAsyncLoader, TransitionAsyncLoad};
use leptos::{
    ev::{self, MouseEvent, SubmitEvent},
    html::{self, button, div, h1, span},
    prelude::*,
};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod async_components;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="grid grid-cols-4 gap-4 p-4 bg-base">
                <Routes fallback=|| "Not found...">
                    <Route path=path!("/") view=Counter />
                    <Route path=path!("/about") view=RadialProgress />
                    <Route path=path!("/async") view=AsyncPage />
                </Routes>
            </div>

            <div class="grid grid-cols-4 gap-4 p-4 bg-base">
                <Counter />
                <ButtonArray />
                <ButtonArray length=2 />

                <TextField />
                <TextForm />

                <Selector length=6 />

                <NumberInput />

                <ContextDemo />

                <TakesChildren render_prop=|| {
                    view! { <p>"This is a render prop"</p> }
                }>"This is a child prop"</TakesChildren>

                {no_macro_counter(1, 17, 3)}

                <RadialProgress />

                <AsyncLoader />
                <SuspendedAsyncLoader />
                <TransitionAsyncLoad />
            </div>
        </Router>
    }
}

#[component]
fn AsyncPage() -> impl IntoView {
    view! {
        <AsyncLoader />
        <SuspendedAsyncLoader />
        <TransitionAsyncLoad />
    }
}

#[component]
fn Card(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="card shadow-lg bg-base-300">
            <div class="card-body">
                <h1 class="card-title">{title}</h1>
                {children()}
            </div>
        </div>
    }
}

#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    let log2_count = Signal::derive(move || ((count.get() as f32).log2() * 100.0) as i32);

    view! {
        <Card title="Counter">
            <button class="btn" on:click=move |_| *set_count.write() += 1>
                Increase by 1
            </button>
            <button class="btn" on:click=move |_| *set_count.write() -= 1>
                Decrease by 1
            </button>
            <button class="btn" on:click=move |_| *set_count.write() *= 2>
                Double
            </button>
            <p class=("red", move || count.get() % 2 == 1)>"Current value : " {count}</p>
            <p>"Double current value : " {move || count.get() * 2}</p>
            <div>
                <ProgressBar value=count />
                <ProgressBar max=10 value=count />
                <ProgressBar max=800 value=log2_count />
            </div>

        </Card>
    }
}

#[component]
fn TextField() -> impl IntoView {
    let (name, set_name) = signal("Jane Doe".to_string());

    view! {
        <Card title="Text Field">
            <input
                class="input"
                type="text"
                on:input:target=move |ev| { set_name.set(ev.target().value()) }
                prop:value=name
            />
            <p>"Name is " {name}</p>
        </Card>
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
        <Card title="Text Form">
            <form class="form-control space-y-2" on:submit=on_submit>
                <input class="input" type="text" value=name node_ref=input_element />
                <input class="btn btn-sm btn-outline" type="submit" value="Submit" />
            </form>
            <p>"Name is " {name}</p>
        </Card>
    }
}

#[component]
fn ButtonArray(#[prop(default = 5)] length: u16) -> impl IntoView {
    let counters = (1..=length).map(|_| RwSignal::new(0));

    view! {
        <Card title="Button Array">
            {counters
                .map(|signal| {
                    view! {
                        <button
                            class="btn bg-secondary text-secondary-content"
                            on:click=move |_| {
                                *signal.write() += 1;
                            }
                        >
                            "Click me: "
                            {signal}
                        </button>
                    }
                })
                .collect_view()}
        </Card>
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
        <progress class="progress" max=max value=value />
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
        <Card title="Selector">
            <select
                class="select select-secondary"
                prop:value=selected
                on:change:target=move |ev| {
                    set_selected.set(ev.target().value().parse().unwrap());
                }
            >
                {options_view}
            </select>
            <button class="btn" on:click=reset_selector>
                "Reset selector"
            </button>
            <button class="btn" on:click=increment_selector>
                "Increment selector"
            </button>
        </Card>
    }
}

#[component]
fn NumberInput() -> impl IntoView {
    let (value, set_value) = signal(Ok(0i64));

    let success_text = view! { <p>"You entered "{value}</p> };

    let error_message = |errors: ArcRwSignal<Errors>| {
        view! {
            <p>
                "Not a number! Errors: "
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect::<Vec<_>>()
                    }}
                </ul>
            </p>
        }
    };

    view! {
        <Card title="Error Handling">
            <input
                class="input input-bordered"
                type="text"
                on:input:target=move |ev| { set_value.set(ev.target().value().parse()) }
                value=0
            />
            <ErrorBoundary fallback=error_message>{success_text}</ErrorBoundary>
        </Card>
    }
}

#[component]
fn ContextDemo() -> impl IntoView {
    let (toggled, set_toggled) = signal(false);
    provide_context(set_toggled);

    view! {
        <Card title="Context Demo">
            <ContextDemoButton />
            <ContextDemoText toggled=toggled />
        </Card>
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
    view! {
        <button class="btn btn-primary" on:click=move |_| setter.update(|value| *value = !*value)>
            "Toggle"
        </button>
    }
}

#[component]
fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV + std::marker::Send + 'static,
    IV: IntoView + 'static,
{
    view! {
        <Card title="Takes Children">
            <h2 class="text-lg">Render Props</h2>
            {render_prop()}
            <h2 class="text-lg">Children</h2>
            {children()}
        </Card>
    }
}

fn no_macro_counter(initial: i32, max: i32, step: i32) -> impl IntoView {
    let (value, set_value) = signal(initial);

    let reset = move |_| *set_value.write() = initial;
    let increment = move |_| set_value.update(|value| *value = (*value + step) % max);
    let decrement = move |_| set_value.update(|value| *value = (*value - step) % max);

    div()
        .class("card shadow-lg bg-base-300")
        .child(
            div().class("card-body").child((
                h1().class("card-title").child("NoMacro"),
                button().class("btn").on(ev::click, reset).child("Reset"),
                button()
                    .class("btn")
                    .on(ev::click, increment)
                    .child(("+", step)),
                button()
                    .class("btn")
                    .on(ev::click, decrement)
                    .child(("-", step)),
                span().child(("Value: ", value)),
            )),
        )
        .into_view()
}

#[component]
fn RadialProgress() -> impl IntoView {
    let (value, set_value) = signal(0u32);

    let increment_1 = move |_: MouseEvent| *set_value.write() += 1;
    let increment_5 = move |_: MouseEvent| *set_value.write() += 5;
    let increment_10 = move |_: MouseEvent| *set_value.write() += 10;
    let reset = move |_: MouseEvent| set_value.set(0);

    view! {
        <Card title="Radial Progress">
            <div class="grid justify-items-center p-4">
                <div
                    role="progressbar"
                    style=("--value", move || value.get().to_string())
                    class="radial-progress bg-secondary text-secondary-content border-4 border-secondary"
                >
                    {move || value.get()}
                    "%"
                </div>
            </div>
            <div class="grid grid-cols-2 gap-4">
                <button class="btn" on:click=increment_1>
                    "+1"
                </button>
                <button class="btn" on:click=increment_5>
                    "+5"
                </button>
                <button class="btn" on:click=increment_10>
                    "+10"
                </button>
                <button class="btn" on:click=reset>
                    "Reset"
                </button>
            </div>
        </Card>
    }
}
