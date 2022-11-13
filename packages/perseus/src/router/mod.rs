#[cfg(target_arch = "wasm32")]
mod app_route;
#[cfg(target_arch = "wasm32")]
mod get_initial_view;
#[cfg(target_arch = "wasm32")]
mod get_subsequent_view;
mod match_route;
mod route_verdict;
#[cfg(target_arch = "wasm32")]
mod router_component;
mod router_state;

#[cfg(target_arch = "wasm32")]
pub(crate) use app_route::PerseusRoute;
pub use match_route::{
    get_template_for_path, get_template_for_path_atomic, match_route, match_route_atomic,
};
pub use route_verdict::{RouteInfo, RouteInfoAtomic, RouteVerdict, RouteVerdictAtomic};
#[cfg(target_arch = "wasm32")]
pub(crate) use router_component::{perseus_router, PerseusRouterProps};
pub use router_state::{RouterLoadState, RouterState};

#[cfg(target_arch = "wasm32")]
pub(crate) use get_initial_view::{get_initial_view, InitialView};
#[cfg(target_arch = "wasm32")]
pub(crate) use get_subsequent_view::{get_subsequent_view, GetSubsequentViewProps, SubsequentView};
