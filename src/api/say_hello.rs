use leptos::prelude::*;

#[cfg(feature = "ssr")]
use worker::console_log;

#[server(SayHello)]
pub async fn say_hello(num: i32) -> Result<String, ServerFnError> {
    console_log!("Hello from the API!!! I got {}", num);
    Ok(format!("Hello from the API!!! I got {num}"))
}
