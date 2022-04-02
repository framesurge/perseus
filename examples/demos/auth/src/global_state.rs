use perseus::{state::GlobalStateCreator, RenderFnResult};
use serde::{Deserialize, Serialize};

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[perseus::autoserde(global_build_state)]
pub async fn get_build_state() -> RenderFnResult<AppState> {
    Ok(AppState {
        // We explicitly tell the first page that no login state has been checked yet
        auth: AuthData {
            state: LoginState::Server,
            username: String::new(),
        },
    })
}

#[perseus::make_rx(AppStateRx)]
#[rx::nested("auth", AuthDataRx)]
pub struct AppState {
    /// Authentication data accessible to all pages.
    pub auth: AuthData,
}

/// The possible login states, including one for the server.
// A better structure might have `Yes` have an attached `AuthData` and use this as the top-level element, but then we'd have to implement `MakeRx`/`MakeUnrx` manually on this (`make_rx`
// can't handle `enum`s)
#[derive(Clone, Serialize, Deserialize)]
pub enum LoginState {
    Yes,
    No,
    Server,
}

/// Authentication data for the app.
// In a real app, you might store privileges here, or user preferences, etc. (all the things you'd need to have available constantly and everwhere)
#[perseus::make_rx(AuthDataRx)]
pub struct AuthData {
    /// The actual login status.
    pub state: LoginState,
    /// The user's username.
    pub username: String,
}
// We implement a custom function on the reactive version of the global state here (hence the `.get()`s and `.set()`s, all the fields become `Signal`s)
// There's no point in implementing it on the unreactive version, since this will only be called from within the browser, in which we have a reactive version
impl AuthDataRx {
    /// Checks whether or not the user is logged in and modifies the internal state accordingly. If this has already been run, it won't do anything (aka. it will only run if it's `Server`)
    #[cfg(target_arch = "wasm32")] // This just avoids an unused function warning (since we have to gate the `.update()` call)
    pub fn detect_state(&self) {
        // If we've checked the login status before, then we should assume the status hasn't changed (we'd change this in a login/logout page)
        if let LoginState::Yes | LoginState::No = *self.state.get() {
            return;
        }

        // See the docs page on authentication to learn how to put something *secure* here
        // This example is NOT production-safe, and would result in absolutely terrible security!!!

        // All we're doing in here is checking for the existence of a storage entry that contains a username (any attacker could trivially fake this)
        // Note that this storage API may be inaccessible, which we completely ignore here for simplicity
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let auth_token = storage.get("username").unwrap(); // This is a `Result<Option<T>, E>`

        if let Some(username) = auth_token {
            self.username.set(username.to_string());
            self.state.set(LoginState::Yes);
        } else {
            self.username.set(String::new());
            self.state.set(LoginState::No)
        }
    }

    /// Logs the user in with the given username.
    pub fn login(&self, username: &str) {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.set("username", username).unwrap();
        self.state.set(LoginState::Yes);
        self.username.set(username.to_string());
    }
    /// Logs the user out.
    pub fn logout(&self) {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.delete("username").unwrap();
        self.state.set(LoginState::No);
        self.username.set(String::new());
    }
}
