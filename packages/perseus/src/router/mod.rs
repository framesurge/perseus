#[cfg(client)]
mod app_route;
mod match_route;
#[cfg(client)]
mod page_disposer;
mod route_verdict;
#[cfg(client)]
mod router_state;

#[cfg(client)]
pub(crate) use app_route::PerseusRoute;
pub(crate) use match_route::match_route;
pub use route_verdict::{FullRouteInfo, FullRouteVerdict, RouteInfo, RouteVerdict};
#[cfg(client)]
pub use router_state::{RouterLoadState, RouterState};

#[cfg(client)]
pub(crate) use page_disposer::PageDisposer;
