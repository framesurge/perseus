#[cfg(any(client, doc))]
mod app_route;
mod match_route;
#[cfg(any(client, doc))]
mod page_disposer;
mod route_verdict;
#[cfg(any(client, doc))]
mod router_state;

#[cfg(any(client, doc))]
pub(crate) use app_route::PerseusRoute;
pub(crate) use match_route::match_route;
pub use route_verdict::{FullRouteInfo, FullRouteVerdict, RouteInfo, RouteVerdict};
#[cfg(any(client, doc))]
pub use router_state::{RouterLoadState, RouterState};

#[cfg(any(client, doc))]
pub(crate) use page_disposer::PageDisposer;
