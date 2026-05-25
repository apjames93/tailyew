use std::future::Future;
use std::pin::Pin;
use yew::events::SubmitEvent;
use yew::prelude::*;

pub type FormSubmitCallback =
    Callback<SubmitEvent, Pin<Box<dyn Future<Output = Result<Option<String>, String>>>>>;

pub type FormSubmitFuture = Pin<Box<dyn Future<Output = Result<Option<String>, String>>>>;

pub fn async_callback<F, Fut>(f: F) -> FormSubmitCallback
where
    F: Fn(SubmitEvent) -> Fut + 'static,
    Fut: Future<Output = Result<Option<String>, String>> + 'static,
{
    Callback::from(move |e| {
        let fut = f(e);
        let boxed: Pin<Box<dyn Future<Output = Result<Option<String>, String>>>> = Box::pin(fut);
        boxed
    })
}
