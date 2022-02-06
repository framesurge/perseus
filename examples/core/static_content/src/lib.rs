mod error_pages;
mod templates;

use perseus::define_app;
define_app! {
    templates: [
        crate::templates::index::get_template::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages(),
    // This sets up a map of URLs in your app to files in your project's directory
    // For security reasons, you can't add files outside the current directory (though this *could* be circumvented, it should be avoided in general)
    //
    // Note that this is only needed for serving content at custom URLs in your app, anything in the `static/` filesystem directory will be served at the `/.perseus/static/` URL
    static_aliases: {
        "/test.txt" => "test.txt"
    }
}
