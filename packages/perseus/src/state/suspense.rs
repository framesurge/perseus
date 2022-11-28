use super::{rx_result::RxResultIntermediate, Freeze, MakeRx, MakeRxRef, MakeUnrx};
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use sycamore::prelude::{RcSignal, Scope};
use sycamore_futures::spawn_local_scoped;

/// A utility function for calling suspense handlers and managing their errors.
/// This automatically implements the pattern of allowing suspense handlers to
/// return arbitrary errors that will be set, enabling the more convenient use
/// of `?` in those handlers.
///
/// Note that this function should only be used for suspense fields that are
/// also reactively nested, and therefore that use `RxResult`.
///
/// You shouldn't need to do this unless you're manually deriving the traits for
/// the reactive state platform.
///
/// Note that this will simply start an asynchronous call to run the suspense
/// handler, managing any errors.
///
/// The handler this takes is a future, so the asynchronous function handler
/// itself should be called without `.await` before being provided to this
/// function.
pub fn compute_nested_suspense<'a, T, E, F>(
    cx: Scope<'a>,
    state: RxResultIntermediate<T, E>,
    handler: F,
) where
    F: Future<Output = Result<(), E>> + 'a,
    T: MakeRx + Serialize + DeserializeOwned + Clone + 'static, /* Note this `Clone` bound!
                                                                 * (Otherwise cloning goes to
                                                                 * the undelrying `RcSignal`) */
    <T as MakeRx>::Rx: MakeUnrx<Unrx = T> + Freeze + MakeRxRef + Clone + 'static,
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    spawn_local_scoped(cx, async move {
        let res = handler.await;
        if let Err(err) = res {
            state.set(Err(err));
        }
    });
}

/// A utility function for calling suspense handlers and managing their errors.
/// This automatically implements the pattern of allowing suspense handlers to
/// return arbitrary errors that will be set, enabling the more convenient use
/// of `?` in those handlers.
///
/// Note that this function should only be used for suspense fields that are
/// *not* reactively nested, and that therefore use a standard `Result<T, E>`.
///
/// You shouldn't need to do this unless you're manually deriving the traits for
/// the reactive state platform.
///
/// Note that this will simply start an asynchronous call to run the suspense
/// handler, managing any errors.
///
/// The handler this takes is a future, so the asynchronous function handler
/// itself should be called without `.await` before being provided to this
/// function.
pub fn compute_suspense<'a, T, E, F>(cx: Scope<'a>, state: RcSignal<Result<T, E>>, handler: F)
where
    F: Future<Output = Result<(), E>> + 'a,
    T: Serialize + DeserializeOwned + Clone + 'static, /* Note this `Clone` bound! (Otherwise
                                                        * cloning goes to the undelrying
                                                        * `RcSignal`) */
    E: Serialize + DeserializeOwned + Clone + 'static,
{
    spawn_local_scoped(cx, async move {
        // let state_ref = create_ref(cx, state.clone());
        let res = handler.await;
        if let Err(err) = res {
            state.set(Err(err));
        }
    });
}
