use perseus::{state::GlobalStateCreator, RenderFnResult};

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[perseus::make_rx(AppStateRx)]
pub struct AppState {
    pub test: String,
}

#[perseus::global_build_state]
pub async fn get_build_state() -> RenderFnResult<AppState> {
    Ok(AppState {
        test: "Hello World!".to_string(),
    })
}
