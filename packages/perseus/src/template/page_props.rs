/// The properties that every page will be initialized with. You shouldn't ever
/// need to interact with this unless you decide not to use the template macros.
///
/// *Note: this used to include global state, which is now handled separately.*
#[derive(Clone, Debug)]
pub struct PageProps {
    /// The path it's rendering at.
    pub path: String,
    /// The state provided to the page. This will be `Some(_)` if state was
    /// generated, we just can't prove that to the compiler.
    pub state: Option<String>,
}
