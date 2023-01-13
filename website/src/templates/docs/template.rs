use crate::templates::docs::container::{DocsContainer, DocsContainerProps};
use crate::templates::docs::generation::{
    get_build_paths, get_build_state, DocsManifest, DocsVersionStatus,
};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, UnreactiveState)]
pub struct DocsPageProps {
    // We don't need to use translation IDs here because the docs are i18ned at the filesystem
    // level
    pub title: String,
    pub content: String,
    pub sidebar_content: String,
    pub status: DocsVersionStatus,
    pub manifest: DocsManifest,
    pub current_version: String,
}

pub fn docs_page<G: Html>(cx: Scope, props: DocsPageProps) -> View<G> {
    // These come pre-translated for the current locale
    // Note that all the docs files have a title emblazoned at the top already, so
    // we only need the title in the `<head>`
    let DocsPageProps {
        content,
        sidebar_content,
        status,
        manifest,
        current_version,
        ..
    } = props;
    view! { cx,
        DocsContainer(DocsContainerProps {
            docs_links: sidebar_content,
            children: view! { cx,
                // Because this is in a grid, we have to make sure it doesn't overflow by specifying this minimum width
                div(class = "markdown min-w-0 pb-10", dangerously_set_inner_html = &content)
            },
            status,
            manifest,
            current_version
        })
        // Because of how Perseus currently shifts everything, we need to re-highlight
        // And if the user starts on a page with nothing, they'll see no highlighting on any other pages, so we rerun every time the URL changes
        // This will be relative to the base URI
        script(src = ".perseus/static/prism.js", defer = true)
        script {
            "window.Prism.highlightAll();"
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, props: DocsPageProps) -> View<SsrNode> {
    use perseus::t;

    view! { cx,
        title { (format!("{} | {}", props.title, t!(cx, "docs-title-base"))) }
        link(rel = "stylesheet", href = ".perseus/static/styles/markdown.css")
        link(rel = "stylesheet", href = ".perseus/static/styles/docs_links_markdown.css")
        link(rel = "stylesheet", href = ".perseus/static/prism.css")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("docs")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .view_with_unreactive_state(docs_page)
        .head_with_state(head)
        .build()
}
