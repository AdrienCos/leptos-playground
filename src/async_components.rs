use gloo_timers::future::TimeoutFuture;
use leptos::{ev::MouseEvent, prelude::*};

use crate::Card;

async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
pub fn AsyncLoader() -> impl IntoView {
    let (value, set_value) = signal(0);

    let async_data = LocalResource::new(move || load_data(value.get()));
    let static_data = LocalResource::new(|| load_data(0));
    let increment = move |_: MouseEvent| *set_value.write() += 1;

    let async_result = move || {
        async_data
            .get()
            .as_deref()
            .map(|e| format!("{}", e))
            .unwrap_or("Loading...".to_string())
    };
    let static_result = move || static_data.get().as_deref().copied();
    view! {
        <Card title="Async Loader">
            <button class="btn" on:click=increment>
                "Run"
            </button>
            <p>"Count: "{value}</p>
            <p>"Dynamic data: "{async_result}</p>
            <p>"Static data: "{static_result}</p>
        </Card>
    }
}

#[component]
pub fn SuspendedAsyncLoader() -> impl IntoView {
    let (count, set_count) = signal(0);
    let increment_count = move |_: MouseEvent| *set_count.write() += 1;
    let async_count = LocalResource::new(move || load_data(count.get()));

    let (price, set_price) = signal(0);
    let increment_price = move |_: MouseEvent| *set_price.write() += 1;
    let async_price = LocalResource::new(move || load_data(price.get()));

    let fallback = move || view! { "Loading..." };

    let resolved_value = move || {
        Suspend::new(async move {
            let price_res = async_price.await;
            let count_res = async_count.await;
            price_res * count_res
        })
    };

    view! {
        <Card title="Suspended Async">
            <button class="btn" on:click=increment_count>
                "Increment count"
            </button>
            <button class="btn" on:click=increment_price>
                "Increment price"
            </button>

            <Suspense fallback=fallback>"Total price is " {resolved_value}</Suspense>
        </Card>
    }
}

#[component]
pub fn TransitionAsyncLoad() -> impl IntoView {
    let (count, set_count) = signal(0);
    let increment_count = move |_: MouseEvent| *set_count.write() += 1;
    let async_count = LocalResource::new(move || load_data(count.get()));

    let fallback = move || view! { "Loading..." };

    let resolved_value = move || Suspend::new(async move { async_count.await });

    view! {
        <Card title="Transition Suspended Async">
            <button class="btn" on:click=increment_count>
                "Increment count"
            </button>

            <Transition fallback=fallback>"Count is " {resolved_value}</Transition>
        </Card>
    }
}
