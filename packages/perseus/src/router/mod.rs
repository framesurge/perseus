mod app_route; // This just exposes a macro
mod match_route;
mod route_verdict;
mod router_component;
mod router_state;

pub use app_route::PerseusRoute;
pub use match_route::{
    get_template_for_path, get_template_for_path_atomic, match_route, match_route_atomic,
};
pub use route_verdict::{RouteInfo, RouteInfoAtomic, RouteVerdict, RouteVerdictAtomic};
pub use router_component::*; // TODO
pub use router_state::{RouterLoadState, RouterState};
