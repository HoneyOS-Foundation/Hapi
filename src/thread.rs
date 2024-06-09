use std::{ffi::c_void, pin::Pin};

use futures::Future;

/// A wrapper around async closures
pub trait AsyncFnOnce {
    fn call_once_async(self) -> Pin<Box<dyn Future<Output = ()>>>;
}

/// Spawn a function on a new thread
pub fn spawn<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    let ptr = Box::into_raw(Box::new(Box::new(f) as Box<dyn FnOnce()>));
    unsafe { crate::ffi::hapi_thread_spawn(ptr as *const c_void) }
}

/// Spawn an async function on a new thread
pub fn spawn_async<F>(f: F)
where
    F: AsyncFnOnce,
{
    let f = Box::new(move || {
        futures::executor::block_on(async {
            f.call_once_async().await;
        })
    });

    let ptr = Box::into_raw(Box::new(Box::new(f) as Box<dyn FnOnce()>));
    unsafe { crate::ffi::hapi_thread_spawn(ptr as *const c_void) }
}

impl<A, Fut> AsyncFnOnce for A
where
    A: FnOnce() -> Fut,
    Fut: Future<Output = ()> + 'static,
{
    fn call_once_async(self) -> Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(self())
    }
}
