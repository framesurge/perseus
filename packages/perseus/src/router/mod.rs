#[cfg(target_arch = "wasm32")]
mod app_route;
mod match_route;
#[cfg(target_arch = "wasm32")]
mod page_disposer;
mod route_verdict;
#[cfg(target_arch = "wasm32")]
mod router_state;

#[cfg(target_arch = "wasm32")]
pub(crate) use app_route::PerseusRoute;
pub(crate) use match_route::match_route;
pub use route_verdict::{FullRouteInfo, FullRouteVerdict, RouteInfo, RouteVerdict};
#[cfg(target_arch = "wasm32")]
pub use router_state::{RouterLoadState, RouterState};

#[cfg(target_arch = "wasm32")]
pub(crate) use page_disposer::PageDisposer;
