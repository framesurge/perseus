use crate::templates::docs::get_file_at_version::get_file_at_version;
use crate::templates::docs::icons::{ERROR_ICON, WARNING_ICON};
use crate::templates::docs::template::DocsPageProps;
use lazy_static::lazy_static;
use perseus::{internal::get_path_prefix_server, t, RenderFnResult, RenderFnResultWithCause};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use sycamore::prelude::*;
use walkdir::WalkDir;

pub fn parse_md_to_html(markdown: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(markdown, opts);
    let mut html_contents = String::new();
    html::push_html(&mut html_contents, parser);

    html_contents
}

// By using a lazy static, we won't read from the filesystem in client-side code
lazy_static! {
    /// The latest version of the documentation. This will need to be updated as the docs are from the `docs/stable.txt` file.
    static ref DOCS_MANIFEST: DocsManifest = {
        let contents = fs::read_to_string("../../docs/manifest.json").unwrap();
        serde_json::from_str(&contents).unwrap()
    };
}

/// The stability of a version of the docs, which governs what kind of warning will be displayed.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DocsVersionStatus {
    /// This version is stable, and no warning is needed.
    Stable,
    /// This version is outdated, and the latest stable version is attached.
    Outdated,
    /// This version is released, but in beta, and the latest stable version is attached.
    Beta,
    /// This documentation is for the unreleased next version, and the latest stable version is attached.
    Next,
}
impl DocsVersionStatus {
    /// Renders the docs status to a Sycamore template for display.
    pub fn render<G: GenericNode>(&self) -> View<G> {
        match &self {
            // No message should be displayed if it's the correct version
            Self::Stable => view! {},
            Self::Outdated => {
                view! {
                    div(class = "ring-4 ring-red-400 p-4 rounded-lg mt-1") {
                        div(class = "flex flex-col 2xs:flex-row dark:text-white") {
                            span(
                                class = "self-center mr-2",
                                style = "fill: #f87171;",
                                dangerously_set_inner_html = ERROR_ICON
                            )
                            p(dangerously_set_inner_html = &t!("docs-status.outdated"))
                        }
                    }
                }
            }
            Self::Beta => {
                view! {
                    div(class = "ring-4 ring-yellow-300 p-4 rounded-lg mt-1") {
                        div(class = "flex flex-col 2xs:flex-row dark:text-white") {
                            span(
                                class = "self-center mr-2",
                                style = "fill: #fcd34d;",
                                dangerously_set_inner_html = WARNING_ICON
                            )
                            p(dangerously_set_inner_html = &t!("docs-status.beta"))
                        }
                    }
                }
            }
            Self::Next => {
                view! {
                    div(class = "ring-4 ring-orange-400 p-4 rounded-lg mt-1") {
                        div(class = "flex flex-col 2xs:flex-row dark:text-white") {
                            span(
                                class = "self-center mr-2",
                                style = "fill: #fb923c;",
                                dangerously_set_inner_html = ERROR_ICON
                            )
                            p(dangerously_set_inner_html = &t!("docs-status.next"))
                        }
                    }
                }
            }
        }
    }
}
/// Information about the current state of the documentation, including which versions are outdated and the like.
#[derive(Serialize, Deserialize, Clone)]
pub struct DocsManifest {
    pub stable: String,
    pub outdated: Vec<String>,
    pub beta: Vec<String>,
    /// A map of versions to points in the Git version history.
    pub history_map: HashMap<String, String>,
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    path: String,
    locale: String,
) -> RenderFnResultWithCause<DocsPageProps> {
    let path_vec: Vec<&str> = path.split('/').collect();
    // Localize the path again to what it'll be on the filesystem
    // TODO get Perseus to pass in props from build paths for ease of use?
    // We'll do that differently if it doesn't have a version in front of it, which would be the second part containing two dots
    // Or it could be `next`
    // If the path is just `/docs` though, we'll render the introduction page for the stable version
    let (version, fs_path): (&str, String) = if path == "docs" {
        (
            &DOCS_MANIFEST.stable,
            format!(
                "{}/{}/{}/{}",
                path_vec[0], // `docs`
                &DOCS_MANIFEST.stable,
                &locale,
                "intro"
            ),
        )
    } else if path_vec[1].split('.').count() == 3 || path_vec[1] == "next" {
        (
            path_vec[1],
            format!(
                "{}/{}/{}/{}",
                path_vec[0], // `docs`
                path_vec[1], // The version
                &locale,
                path_vec[2..].join("/") // The rest of the path
            ),
        )
    } else {
        (
            &DOCS_MANIFEST.stable,
            // If it doesn't have a version, we'll inject the latest stable one
            format!(
                "{}/{}/{}/{}",
                path_vec[0], // `docs`
                &DOCS_MANIFEST.stable,
                &locale,
                path_vec[1..].join("/") // The rest of the path
            ),
        )
    };
    let fs_path = format!("../../{}.md", fs_path);
    // Read that file
    let contents = fs::read_to_string(&fs_path)?;

    // Handle the directives to include code from another file
    // We only loop through the file's lines if it likely contains what we want
    let contents = if contents.contains("{{#") {
        let mut contents_with_incls = contents.clone();
        for line in contents.lines() {
            let line = line.trim();
            if line.starts_with("{{#include ") && line.ends_with("}}") {
                // Strip the directive to get the path of the file we're including
                let mut incl_path = line
                    .strip_prefix("{{#include ")
                    .unwrap()
                    .strip_suffix("}}")
                    .unwrap();
                // All the files here are in `docs/`, and they'll be including from outside there, so strip away any `../`s
                while let Some(new_path) = incl_path.strip_prefix("../") {
                    incl_path = new_path;
                }
                // If we're on the `next` version, read from the filesystem directly
                // Otherwise, use Git to get the appropriate version (otherwise we get #60)
                let incl_contents = if version == "next" {
                    // Add a `../../` to the front so that it's relative from `.perseus/`, where we are now
                    fs::read_to_string(format!("../../{}", &incl_path))?
                } else {
                    // Get the corresponding history point for this version
                    let history_point = DOCS_MANIFEST.history_map.get(version);
                    let history_point = match history_point {
                        Some(history_point) => history_point,
                        None => panic!("docs version '{}' not present in history map", version),
                    };
                    // We want the path relative to the root of the project directory (where the Git repo is)
                    get_file_at_version(incl_path, history_point, PathBuf::from("../../"))?
                };
                // Now replace the whole directive (trimmed though to preserve any whitespace) with the file's contents
                contents_with_incls = contents_with_incls.replace(&line, &incl_contents);
            } else if line.starts_with("{{#lines_include ") && line.ends_with("}}") {
                // Strip the directive to get the path of the file we're including
                let mut incl_path_with_lines_suffix = line
                    .strip_prefix("{{#lines_include ")
                    .unwrap()
                    .strip_suffix("}}")
                    .unwrap();
                // All the files here are in `docs/`, and they'll be including from outside there, so strip away any `../`s
                while let Some(new_path) = incl_path_with_lines_suffix.strip_prefix("../") {
                    incl_path_with_lines_suffix = new_path;
                }
                // Now remove the suffix that specifies the lines to get
                let (incl_path, lines_start, lines_end) = {
                    let vec: Vec<&str> = incl_path_with_lines_suffix.split(':').collect();
                    (vec[0], vec[1].parse::<usize>()?, vec[2].parse::<usize>()?)
                };
                // If we're on the `next` version, read from the filesystem directly
                // Otherwise, use Git to get the appropriate version (otherwise we get #60)
                let incl_contents_full = if version == "next" {
                    // Add a `../../` to the front so that it's relative from `.perseus/`, where we are now
                    fs::read_to_string(format!("../../{}", &incl_path))?
                } else {
                    // Get the corresponding history point for this version
                    let history_point = DOCS_MANIFEST.history_map.get(version);
                    let history_point = match history_point {
                        Some(history_point) => history_point,
                        None => panic!("docs version '{}' not present in history map", version),
                    };
                    // We want the path relative to the root of the project directory (where the Git repo is)
                    get_file_at_version(incl_path, history_point, PathBuf::from("../../"))?
                };
                // Get the specific lines wanted
                let incl_contents_lines = incl_contents_full
                    .lines()
                    .collect::<Vec<&str>>()
                    .get((lines_start - 1)..(lines_end))
                    .unwrap()
                    .join("\n");
                // Now replace the whole directive (trimmed though to preserve any whitespace) with the file's contents
                contents_with_incls = contents_with_incls.replace(&line, &incl_contents_lines);
            }
        }
        contents_with_incls
    } else {
        contents
    };

    // Parse any relative links to other pages in the docs
    // We add the base path, the locale, and the docs version
    // We use the special token `:` to denote these (e.g. `[static exporting](:exporting)`)
    let contents = contents.replace(
        "](:",
        &format!(
            "]({}/{}/docs/{}/",
            get_path_prefix_server(),
            &locale,
            &version
        ),
    );

    // Parse the file to HTML
    let html_contents = parse_md_to_html(&contents);
    // Get the title from the first line of the contents, stripping the initial `#`
    // This is brittle, but surprisingly quite reliable as long as documentation files have headings
    let title = contents.lines().collect::<Vec<&str>>()[0]
        .strip_prefix("# ")
        .unwrap();

    // Get the sidebar from `SUMMARY.md`
    let sidebar_fs_path = format!("../../docs/{}/{}/SUMMARY.md", &version, &locale);
    let sidebar_contents = fs::read_to_string(&sidebar_fs_path)?;
    // Replace all links in that file with localized equivalents with versions as well (with the base path added)
    // That means unversioned paths will redirect to the appropriate stable version
    let sidebar_contents = sidebar_contents.replace(
        "/docs",
        &format!("{}/{}/docs/{}", get_path_prefix_server(), &locale, &version),
    );
    let sidebar_html_contents = parse_md_to_html(&sidebar_contents);

    // Work out the status of this page
    let status = if version == "next" {
        DocsVersionStatus::Next
    } else if DOCS_MANIFEST.outdated.iter().any(|v| v == version) {
        DocsVersionStatus::Outdated
    } else if DOCS_MANIFEST.beta.iter().any(|v| v == version) {
        DocsVersionStatus::Beta
    } else if DOCS_MANIFEST.stable == version {
        DocsVersionStatus::Stable
    } else {
        panic!("version '{}' isn't listed in the docs manifest", version)
    };

    let props = DocsPageProps {
        title: title.to_string(),
        content: html_contents,
        sidebar_content: sidebar_html_contents,
        status,
        manifest: DOCS_MANIFEST.clone(),
        current_version: version.to_string(),
    };

    Ok(props)
}

pub async fn get_build_paths() -> RenderFnResult<Vec<String>> {
    // We start off by rendering the `/docs` page itself as an alias
    let mut paths = vec!["".to_string()];
    // Get the `docs/` directory (relative to `.perseus/`)
    let docs_dir = PathBuf::from("../../docs");
    // Loop through it
    for entry in WalkDir::new(docs_dir) {
        let entry = entry?;
        let path = entry.path();
        // Ignore any empty directories or the like
        if path.is_file() {
            // This should all pass, there are no non-Unicode filenames in the docs (and i18n titles are handled outside filenames)
            // Also, all these are relative, which means we can safely strip away the `../../docs/`
            // We also remove the file extensions (which are all `.md`)
            let path_str = path.to_str().unwrap().replace(".md", "");
            let path_str = path_str.strip_prefix("../../docs/").unwrap();
            // Only proceed for paths in the default locale (`en-US`), which we'll use to generate paths
            // Also disallow any of the `SUMMARY.md` files at this point (the extension has been stripped)
            // Also disallow the manifest file
            if path_str.contains("en-US/")
                && !path_str.ends_with("SUMMARY")
                && !path_str.ends_with("manifest.json")
            {
                // Now remove that locale (it'll be put at the front of the path in the URL)
                let path_str = path_str.replace("en-US/", "");
                // This path should be rendered!
                paths.push(path_str.clone());
                // If it's for the latest stable version though, we should also render it without that prefix
                // That way the latest stable verison is always at the docs without a version prefix (which I think is more sensible than having the unreleased version there)
                if path_str.starts_with(&DOCS_MANIFEST.stable) {
                    let unprefixed_path_str = path_str
                        .strip_prefix(&format!("{}/", &DOCS_MANIFEST.stable))
                        .unwrap();
                    paths.push(unprefixed_path_str.to_string());
                }
            }
        }
    }

    Ok(paths)
}
