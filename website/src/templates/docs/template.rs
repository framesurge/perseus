use crate::templates::docs::container::{DocsContainer, DocsContainerProps};
use crate::templates::docs::generation::{
    get_build_paths, get_build_state, DocsManifest, DocsVersionStatus,
};
use perseus::{t, GenericNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::Template as SycamoreTemplate;
use sycamore::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DocsPageProps {
    // We don't need to use translation IDs here because the docs are i18ned at the filesystem level
    pub title: String,
    pub content: String,
    pub sidebar_content: String,
    pub status: DocsVersionStatus,
    pub manifest: DocsManifest,
    pub current_version: String,
}

#[perseus::template(DocsPage)]
#[component(DocsPage<G>)]
pub fn docs_page(props: DocsPageProps) -> SycamoreTemplate<G> {
    // These come pre-translated for the current locale
    // Note that all the docs files have a title emblazoned at the top already, so we only need the title in the `<head>`
    let DocsPageProps {
        content,
        sidebar_content,
        status,
        manifest,
        current_version,
        ..
    } = props;
    template! {
        DocsContainer(DocsContainerProps {
            docs_links: sidebar_content,
            children: template! {
                div(class = "markdown", dangerously_set_inner_html = &content)
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

#[perseus::head]
pub fn head(props: DocsPageProps) -> SycamoreTemplate<SsrNode> {
    template! {
        title { (format!("{} | {}", props.title, t!("docs-title-base"))) }
        link(rel = "stylesheet", href = ".perseus/static/styles/markdown.css")
        link(rel = "stylesheet", href = ".perseus/static/styles/docs_links_markdown.css")
        link(rel = "stylesheet", href = ".perseus/static/prism.css")
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("docs")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .template(docs_page)
        .head(head)
}
