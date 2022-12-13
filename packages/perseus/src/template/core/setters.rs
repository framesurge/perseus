use super::super::fn_types::*;
use super::Template;
#[cfg(not(target_arch = "wasm32"))]
use crate::errors::{BlamedError, ErrorBlame, GenericBlamedError, ServerError};
use crate::state::{
    AnyFreeze, BuildPaths, MakeRx, MakeRxRef, MakeUnrx, PssContains, TemplateStateWithType,
    UnreactiveState,
};
use crate::state::{StateGeneratorInfo, TemplateState, UnknownStateType};
use crate::utils::PerseusDuration;
#[cfg(not(target_arch = "wasm32"))]
use http::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use sycamore::web::SsrNode;
use sycamore::{prelude::Scope, view::View, web::Html};

impl<G: Html> Template<G> {
    // The server-only ones have a different version for Wasm that takes in an empty
    // function (this means we don't have to bring in function types, and therefore
    // we can avoid bringing in the whole `http` module --- a very significant
    // saving!) The macros handle the creation of empty functions to make user's
    // lives easier
    /// Sets the document `<head>` rendering function to use. The [`View`]
    /// produced by this will only be rendered on the engine-side, and will
    /// *not* be reactive (since it only contains metadata).
    ///
    /// This is for heads that do not require state. Those that do should use
    /// `.head_with_state()` instead.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn head<V: Into<GeneratorResult<View<SsrNode>>>>(
        mut self,
        val: impl Fn(Scope) -> V + Send + Sync + 'static,
    ) -> Template<G> {
        let template_name = self.get_path();
        self.head = Box::new(move |cx, _template_state| {
            let template_name = template_name.clone();
            val(cx).into().to_server_result("head", template_name)
        });
        self
    }
    /// Sets the document `<head>` rendering function to use. The [`View`]
    /// produced by this will only be rendered on the engine-side, and will
    /// *not* be reactive (since it only contains metadata).
    ///
    /// This is for heads that do not require state. Those that do should use
    /// `.head_with_state()` instead.
    #[cfg(target_arch = "wasm32")]
    pub fn head(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }
    /// Sets the function to set headers. This will override Perseus' inbuilt
    /// header defaults. This should only be used when your header-setting
    /// does not need state.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_headers<V: Into<GeneratorResult<HeaderMap>>>(
        mut self,
        val: impl Fn() -> V + Send + Sync + 'static,
    ) -> Template<G> {
        let template_name = self.get_path();
        self.set_headers = Box::new(move |_template_state| {
            let template_name = template_name.clone();
            val().into().to_server_result("set_headers", template_name)
        });
        self
    }
    /// Sets the function to set headers. This will override Perseus' inbuilt
    /// header defaults. This should only be used when your header-setting
    /// does not need state.
    #[cfg(target_arch = "wasm32")]
    pub fn set_headers(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *build paths* strategy with the given function.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build_paths_fn<V: Into<GeneratorResult<BuildPaths>>>(
        mut self,
        val: impl GetBuildPathsUserFnType<V> + Clone + Send + Sync + 'static,
    ) -> Template<G> {
        let template_name = self.get_path();
        self.get_build_paths = Some(Box::new(move || {
            let val = val.clone();
            let template_name = template_name.clone();
            async move {
                val.call()
                    .await
                    .into()
                    .to_server_result("build_paths", template_name)
            }
        }));
        self
    }
    /// Enables the *build paths* strategy with the given function.
    #[cfg(target_arch = "wasm32")]
    pub fn build_paths_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *incremental generation* strategy.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn incremental_generation(mut self) -> Template<G> {
        self.incremental_generation = true;
        self
    }
    /// Enables the *incremental generation* strategy.
    #[cfg(target_arch = "wasm32")]
    pub fn incremental_generation(self) -> Template<G> {
        self
    }

    /// Enables the *build state* strategy with the given function.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build_state_fn<S, B, V>(
        mut self,
        val: impl GetBuildStateUserFnType<S, B, V> + Clone + Send + Sync + 'static,
    ) -> Template<G>
    where
        S: Serialize + DeserializeOwned + MakeRx,
        B: Serialize + DeserializeOwned + Send + Sync + 'static,
        V: Into<BlamedGeneratorResult<S>>,
    {
        let template_name = self.get_path();
        self.get_build_state = Some(Box::new(
            move |info: StateGeneratorInfo<UnknownStateType>| {
                let val = val.clone();
                let template_name = template_name.clone();
                async move {
                    let user_info = info.change_type::<B>();
                    let user_state = val
                        .call(user_info)
                        .await
                        .into()
                        .to_server_result("build_state", template_name)?;
                    let template_state: TemplateState = user_state.into();
                    Ok(template_state)
                }
            },
        ));
        self
    }
    /// Enables the *build state* strategy with the given function.
    #[cfg(target_arch = "wasm32")]
    pub fn build_state_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *request state* strategy with the given function.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn request_state_fn<S, B, V>(
        mut self,
        val: impl GetRequestStateUserFnType<S, B, V> + Clone + Send + Sync + 'static,
    ) -> Template<G>
    where
        S: Serialize + DeserializeOwned + MakeRx,
        B: Serialize + DeserializeOwned + Send + Sync + 'static,
        V: Into<BlamedGeneratorResult<S>>,
    {
        let template_name = self.get_path();
        self.get_request_state = Some(Box::new(
            move |info: StateGeneratorInfo<UnknownStateType>, req| {
                let val = val.clone();
                let template_name = template_name.clone();
                async move {
                    let user_info = info.change_type::<B>();
                    let user_state = val
                        .call(user_info, req)
                        .await
                        .into()
                        .to_server_result("request_state", template_name)?;
                    let template_state: TemplateState = user_state.into();
                    Ok(template_state)
                }
            },
        ));
        self
    }
    /// Enables the *request state* strategy with the given function.
    #[cfg(target_arch = "wasm32")]
    pub fn request_state_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *revalidation* strategy (logic variant) with the given
    /// function.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn should_revalidate_fn<B, V>(
        mut self,
        val: impl ShouldRevalidateUserFnType<B, V> + Clone + Send + Sync + 'static,
    ) -> Template<G>
    where
        B: Serialize + DeserializeOwned + Send + Sync + 'static,
        V: Into<BlamedGeneratorResult<bool>>,
    {
        let template_name = self.get_path();
        self.should_revalidate = Some(Box::new(
            move |info: StateGeneratorInfo<UnknownStateType>, req| {
                let val = val.clone();
                let template_name = template_name.clone();
                async move {
                    let user_info = info.change_type::<B>();
                    val.call(user_info, req)
                        .await
                        .into()
                        .to_server_result("should_revalidate", template_name)
                }
            },
        ));
        self
    }
    /// Enables the *revalidation* strategy (logic variant) with the given
    /// function.
    #[cfg(target_arch = "wasm32")]
    pub fn should_revalidate_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }

    /// Enables the *revalidation* strategy (time variant). This takes a time
    /// string of a form like `1w` for one week.
    ///
    ///    - s: second,
    ///    - m: minute,
    ///    - h: hour,
    ///    - d: day,
    ///    - w: week,
    ///    - M: month (30 days used here, 12M ≠ 1y!),
    ///    - y: year (365 days always, leap years ignored, if you want them add
    ///      them as days)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn revalidate_after<I: PerseusDuration>(mut self, val: I) -> Template<G> {
        let computed_duration = match val.into_computed() {
            Ok(val) => val,
            // This is fine, because this will be checked when we try to build the app (i.e. it'll
            // show up before runtime)
            Err(_) => panic!("invalid revalidation interval"),
        };
        self.revalidate_after = Some(computed_duration);
        self
    }
    /// Enables the *revalidation* strategy (time variant). This takes a time
    /// string of a form like `1w` for one week.
    ///
    ///    - s: second,
    ///    - m: minute,
    ///    - h: hour,
    ///    - d: day,
    ///    - w: week,
    ///    - M: month (30 days used here, 12M ≠ 1y!),
    ///    - y: year (365 days always, leap years ignored, if you want them add
    ///      them as days)
    #[cfg(target_arch = "wasm32")]
    pub fn revalidate_after<I: PerseusDuration>(self, _val: I) -> Template<G> {
        self
    }

    /// Enables state amalgamation with the given function. State amalgamation
    /// allows you to have one template generate state at both build time
    /// and request time. The function you provide here is responsible for
    /// rationalizing the two into one single state to be sent to the client,
    /// and this will be run just after the request state function
    /// completes. See [`States`] for further details.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn amalgamate_states_fn<S, B, V>(
        mut self,
        val: impl AmalgamateStatesUserFnType<S, B, V> + Clone + Send + Sync + 'static,
    ) -> Template<G>
    where
        S: Serialize + DeserializeOwned + MakeRx + Send + Sync + 'static,
        B: Serialize + DeserializeOwned + Send + Sync + 'static,
        V: Into<BlamedGeneratorResult<S>>,
    {
        let template_name = self.get_path();
        self.amalgamate_states = Some(Box::new(
            move |info: StateGeneratorInfo<UnknownStateType>,
                  build_state: TemplateState,
                  request_state: TemplateState| {
                let val = val.clone();
                let template_name = template_name.clone();
                async move {
                    // Amalgamation logic will only be called if both states are indeed defined
                    let typed_build_state = build_state.change_type::<S>();
                    let user_build_state = match typed_build_state.to_concrete() {
                        Ok(state) => state,
                        Err(err) => panic!(
                            "unrecoverable error in state amalgamation parameter derivation: {:#?}",
                            err
                        ),
                    };
                    let typed_request_state = request_state.change_type::<S>();
                    let user_request_state = match typed_request_state.to_concrete() {
                        Ok(state) => state,
                        Err(err) => panic!(
                            "unrecoverable error in state amalgamation parameter derivation: {:#?}",
                            err
                        ),
                    };
                    let user_info = info.change_type::<B>();
                    let user_state = val
                        .call(user_info, user_build_state, user_request_state)
                        .await
                        .into()
                        .to_server_result("amalgamate_states", template_name)?;
                    let template_state: TemplateState = user_state.into();
                    Ok(template_state)
                }
            },
        ));
        self
    }
    /// Enables state amalgamation with the given function. State amalgamation
    /// allows you to have one template generate state at both build time
    /// and request time. The function you provide here is responsible for
    /// rationalizing the two into one single state to be sent to the client,
    /// and this will be run just after the request state function
    /// completes. See [`States`] for further details.
    #[cfg(target_arch = "wasm32")]
    pub fn amalgamate_states_fn(self, _val: impl Fn() + 'static) -> Template<G> {
        self
    }
    /// Allow the building of this page's templates to be rescheduled from
    /// build-tim to request-time.
    ///
    /// A page whose state isn't generated at request-tim and isn't revalidated
    /// can be rendered at build-time, unless it depends on capsules that
    /// don't have those properties. If a page that could be rendered at
    /// build-time were to render with a widget that revalidates later, that
    /// prerender would be invalidated later, leading to render errors. If
    /// that situation arises, and this hasn't been set, building will
    /// return an error.
    ///
    /// If you receive one of those errors, it's almost always absolutely fine
    /// to enable this, as the performance hit will usually be negligible.
    /// If you notice a substantial difference though, you may wish to
    /// reconsider.
    pub fn allow_rescheduling(mut self) -> Self {
        self.can_be_rescheduled = true;
        self
    }
}
