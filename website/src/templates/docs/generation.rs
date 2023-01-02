#[cfg(engine)]
use crate::templates::docs::get_file_at_version::get_file_at_version;
use crate::templates::docs::icons::{ERROR_ICON, WARNING_ICON};
#[cfg(engine)]
use crate::templates::docs::template::DocsPageProps;
use lazy_static::lazy_static;
#[cfg(engine)]
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs; /* The lazy static will never be evaluated on the web, but we still need the
              * import (TODO improve this...a lot) */
use perseus::prelude::*;
#[cfg(engine)]
use std::path::PathBuf;
use sycamore::prelude::*;
#[cfg(engine)]
use walkdir::WalkDir;

#[cfg(engine)] // This is a generation helper
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
// (because these variables are never requested on the client-side)
lazy_static! {
    /// The current documentation manifest, which contains details on all versions of Perseus.
    static ref DOCS_MANIFEST: DocsManifest = {
        let contents = fs::read_to_string("../docs/manifest.json").unwrap();
        serde_json::from_str(&contents).unwrap()
    };
    static ref STABLE_VERSION_NAME: String = get_stable_version(&DOCS_MANIFEST).0;
    static ref OUTDATED_VERSIONS: HashMap<String, VersionManifest> = get_outdated_versions(&DOCS_MANIFEST);
    static ref BETA_VERSIONS: HashMap<String, VersionManifest> = get_beta_versions(&DOCS_MANIFEST);
}

/// The stability of a version of the docs, which governs what kind of warning
/// will be displayed.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DocsVersionStatus {
    /// This version is stable, and no warning is needed.
    Stable,
    /// This version is outdated, and the latest stable version is attached.
    Outdated,
    /// This version is released, but in beta, and the latest stable version is
    /// attached.
    Beta,
    /// This documentation is for the unreleased next version, and the latest
    /// stable version is attached.
    Next,
}
impl DocsVersionStatus {
    /// Renders the docs status to a Sycamore template for display.
    pub fn render<G: GenericNode>(&self, cx: Scope, stable_version: String) -> View<G> {
        match &self {
            // No message should be displayed if it's the correct version
            Self::Stable => View::empty(),
            Self::Outdated => {
                view! { cx,
                    div(class = "ring-4 ring-red-400 p-4 rounded-lg mt-1") {
                        div(class = "flex flex-col 2xs:flex-row dark:text-white") {
                            span(
                                class = "self-center mr-2",
                                style = "fill: #f87171;",
                                dangerously_set_inner_html = ERROR_ICON
                            )
                            p(dangerously_set_inner_html = &t!(
                                "docs-status.outdated",
                                {
                                    "stable" = &stable_version
                                },
                                cx
                            ))
                        }
                    }
                }
            }
            Self::Beta => {
                view! { cx,
                    div(class = "ring-4 ring-yellow-300 p-4 rounded-lg mt-1") {
                        div(class = "flex flex-col 2xs:flex-row dark:text-white") {
                            span(
                                class = "self-center mr-2",
                                style = "fill: #fcd34d;",
                                dangerously_set_inner_html = WARNING_ICON
                            )
                            p(dangerously_set_inner_html = &t!(
                                "docs-status.beta",
                                {
                                    "stable" = &stable_version
                                },
                                cx
                            ))
                        }
                    }
                }
            }
            Self::Next => {
                view! { cx,
                    div(class = "ring-4 ring-orange-400 p-4 rounded-lg mt-1") {
                        div(class = "flex flex-col 2xs:flex-row dark:text-white") {
                            span(
                                class = "self-center mr-2",
                                style = "fill: #fb923c;",
                                dangerously_set_inner_html = ERROR_ICON
                            )
                            p(dangerously_set_inner_html = &t!(
                                "docs-status.next",
                                {
                                    "stable" = &stable_version
                                },
                                cx
                            ))
                        }
                    }
                }
            }
        }
    }
}
/// Information about the current state of the documentation, including which
/// versions are outdated and the like.
pub type DocsManifest = HashMap<String, VersionManifest>;

/// Information about a single version in the documentation manifest.
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq)]
pub struct VersionManifest {
    /// The state of this version.
    pub state: VersionState,
    /// The location in the Git history to get examples from for this version.
    pub git: String,
    /// The version to use on docs.rs for this version. This will be
    /// interpolated into all docs.rs links in this version's docs.
    pub docs_rs: String,
}
/// The possible states a version can be in. Note that there can only be one
/// stable version at a time, and that the special `next` version is not
/// accounted for here.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum VersionState {
    /// The version is outdated, and should no longer be used if possible.
    Outdated,
    /// The version is currently stable.
    Stable,
    /// The version is currently released, but in a beta form.
    Beta,
}

/// Gets the latest stable version of the docs from the given manifest. This
/// returns the version's name and metadata.
pub fn get_stable_version(manifest: &DocsManifest) -> (String, VersionManifest) {
    let mut stable: Option<(String, VersionManifest)> = None;
    for (name, details) in manifest.iter() {
        if details.state == VersionState::Stable {
            stable = Some((name.clone(), details.clone()));
            break;
        }
    }
    if let Some(stable) = stable {
        stable
    } else {
        panic!("no stable version set for docs");
    }
}
/// Gets the outdated versions of the docs from the given manifest. This returns
/// a `HashMap` of their names to their manifests.
pub fn get_outdated_versions(manifest: &DocsManifest) -> HashMap<String, VersionManifest> {
    let mut versions = HashMap::new();
    for (name, details) in manifest.iter() {
        if details.state == VersionState::Outdated {
            versions.insert(name.clone(), details.clone());
        }
    }

    versions
}
/// Gets the beta versions of the docs from the given manifest. This returns a
/// `HashMap` of their names to their manifests.
pub fn get_beta_versions(manifest: &DocsManifest) -> HashMap<String, VersionManifest> {
    let mut versions = HashMap::new();
    for (name, details) in manifest.iter() {
        if details.state == VersionState::Beta {
            versions.insert(name.clone(), details.clone());
        }
    }

    versions
}

#[engine_only_fn]
pub async fn get_build_state(
    StateGeneratorInfo { path, locale, .. }: StateGeneratorInfo<()>,
) -> Result<DocsPageProps, BlamedError<std::io::Error>> {
    use perseus::utils::get_path_prefix_server;
    use regex::Regex;

    // Compat from earlier Perseus versions
    // TODO Remove
    let path = format!("docs/{}", path);
    let path = path.strip_suffix("/").unwrap_or(&path).to_string();

    let path_vec: Vec<&str> = path.split('/').collect();
    // TODO Use build helper state for all this
    // Localize the path again to what it'll be on the filesystem
    // We'll do that differently if it doesn't have a version in front of it, which
    // would be the second part containing two dots Or it could be `next`
    // If the path is just `/docs` though, we'll render the introduction page for
    // the stable version
    let (version, fs_path): (&str, String) = if path == "docs" {
        (
            STABLE_VERSION_NAME.as_str(),
            format!(
                "{}/{}/{}/{}",
                path_vec[0], // `docs`
                STABLE_VERSION_NAME.as_str(),
                &locale,
                "intro"
            ),
        )
    } else if path_vec[1].split('.').count() >= 3 || path_vec[1] == "next" {
        // This conditional depends on checking for a semantic version number in the URL
        // (e.g. `0.3.x`, `0.3.0-v0.3.2`)
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
            STABLE_VERSION_NAME.as_str(),
            // If it doesn't have a version, we'll inject the latest stable one
            format!(
                "{}/{}/{}/{}",
                path_vec[0], // `docs`
                STABLE_VERSION_NAME.as_str(),
                &locale,
                path_vec[1..].join("/") // The rest of the path
            ),
        )
    };
    let fs_path = format!("../{}.md", fs_path);
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
                // All the files here are in `docs/`, and they'll be including from outside
                // there, so strip away any `../`s
                while let Some(new_path) = incl_path.strip_prefix("../") {
                    incl_path = new_path;
                }
                // If we're on the `next` version, read from the filesystem directly
                // Otherwise, use Git to get the appropriate version (otherwise we get #60)
                let incl_contents = if version == "next" {
                    // Add a `../` to the front so that it's relative from the website root, where
                    // we are now
                    let path = format!("../{}", &incl_path);
                    match fs::read_to_string(&path) {
                        Ok(contents) => contents,
                        // If there's an error (which there will be after any major refactor), we'll
                        // tell the user which file couldn't be found
                        Err(err) => {
                            eprintln!("File not found: {} in page {}.", &path, &fs_path);
                            return Err(err.into());
                        }
                    }
                } else {
                    // Get the corresponding history point for this version
                    let version_manifest = DOCS_MANIFEST.get(version);
                    let history_point = match version_manifest {
                        Some(version_manifest) => &version_manifest.git,
                        None => panic!("docs version '{}' not present in manifest", version),
                    };
                    // We want the path relative to the root of the project directory (where the Git
                    // repo is)
                    get_file_at_version(incl_path, history_point, PathBuf::from("../"))?
                };
                // Now replace the whole directive (trimmed though to preserve any whitespace)
                // with the file's contents
                contents_with_incls = contents_with_incls.replace(&line, &incl_contents);
            } else if line.starts_with("{{#lines_include ") && line.ends_with("}}") {
                // Strip the directive to get the path of the file we're including
                let mut incl_path_with_lines_suffix = line
                    .strip_prefix("{{#lines_include ")
                    .unwrap()
                    .strip_suffix("}}")
                    .unwrap();
                // All the files here are in `docs/`, and they'll be including from outside
                // there, so strip away any `../`s
                while let Some(new_path) = incl_path_with_lines_suffix.strip_prefix("../") {
                    incl_path_with_lines_suffix = new_path;
                }
                // Now remove the suffix that specifies the lines to get
                let (incl_path, lines_start, lines_end) = {
                    let vec: Vec<&str> = incl_path_with_lines_suffix.split(':').collect();
                    (
                        vec[0],
                        vec[1].parse::<usize>().map_err(|_| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "invalid opening line bound",
                            )
                        })?,
                        vec[2].parse::<usize>().map_err(|_| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "invalid closing line bound",
                            )
                        })?,
                    )
                };
                // If we're on the `next` version, read from the filesystem directly
                // Otherwise, use Git to get the appropriate version (otherwise we get #60)
                let incl_contents_full = if version == "next" {
                    // Add a `../../` to the front so that it's relative from `.perseus/`, where we
                    // are now
                    let path = format!("../{}", &incl_path);
                    match fs::read_to_string(&path) {
                        Ok(contents) => contents,
                        // If there's an error (which there will be after any major refactor), we'll
                        // tell the user which file couldn't be found
                        Err(err) => {
                            eprintln!("File not found: {} in page {}.", &path, &fs_path);
                            return Err(err.into());
                        }
                    }
                } else {
                    // Get the corresponding history point for this version
                    let version_manifest = DOCS_MANIFEST.get(version);
                    let history_point = match version_manifest {
                        Some(version_manifest) => &version_manifest.git,
                        None => panic!("docs version '{}' not present in manifest", version),
                    };
                    // We want the path relative to the root of the project directory (where the Git
                    // repo is)
                    get_file_at_version(incl_path, history_point, PathBuf::from("../"))?
                };
                // Get the specific lines wanted
                let incl_contents_lines = match incl_contents_full
                    .lines()
                    .collect::<Vec<&str>>()
                    .get((lines_start - 1)..(lines_end))
                {
                    Some(incl_contents_lines) => incl_contents_lines.join("\n"),
                    None => {
                        eprintln!(
                            "File {} couldn't be included from lines {}-{} in {}.",
                            &incl_path,
                            lines_start - 1,
                            lines_end,
                            &fs_path
                        );
                        panic!("file couldn't be included from lines");
                    }
                };
                // Now replace the whole directive (trimmed though to preserve any whitespace)
                // with the file's contents
                contents_with_incls = contents_with_incls.replace(&line, &incl_contents_lines);
            }
        }
        contents_with_incls
    } else {
        contents
    };

    // Parse any relative links to other pages in the docs
    // We add the base path, the locale, and the docs version
    // We use the special token `:` to denote these (e.g. `[static
    // exporting](:exporting)`)
    let contents = contents.replace(
        "](:",
        &format!(
            "]({}/{}/docs/{}/",
            get_path_prefix_server(),
            &locale,
            &version
        ),
    );

    // Parse any links to docs.rs (of the form `[`Error`](=enum.Error@perseus)`,
    // where `perseus` is the package name) Versions are interpolated
    // automatically
    let docs_rs_version = if version == "next" {
        // Unfortunately, `latest` doesn't take account of beta versions, so we use
        // either the latest beta version or the stable version
        let mut beta_versions = BETA_VERSIONS.values().collect::<Vec<&VersionManifest>>();
        beta_versions.sort_by(|a, b| b.partial_cmp(a).unwrap());
        if beta_versions.is_empty() {
            get_stable_version(&DOCS_MANIFEST).1.docs_rs
        } else {
            beta_versions[0].docs_rs.to_string()
        }
    } else {
        match &DOCS_MANIFEST.get(version) {
            Some(version) => version.docs_rs.to_string(),
            None => panic!("docs version '{}' not present in manifest", version),
        }
    };
    let contents = Regex::new(r#"\]\(=(?P<path>.*?)@(?P<pkg>.*?)\)"#)
        .unwrap()
        .replace_all(
            &contents,
            format!(
                "](https://docs.rs/${{pkg}}/{}/${{pkg}}/${{path}}.html)",
                docs_rs_version
            ),
        );

    // Parse the file to HTML
    let html_contents = parse_md_to_html(&contents);
    // Get the title from the first line of the contents, stripping the initial `#`
    // This is brittle, but surprisingly quite reliable as long as documentation
    // files have headings
    let title = contents.lines().collect::<Vec<&str>>()[0]
        .strip_prefix("# ")
        .unwrap();

    // Get the sidebar from `SUMMARY.md`
    let sidebar_fs_path = format!("../docs/{}/{}/SUMMARY.md", &version, &locale);
    let sidebar_contents = fs::read_to_string(&sidebar_fs_path)?;
    // Replace all links in that file with localized equivalents with versions as
    // well (with the base path added) That means unversioned paths will
    // redirect to the appropriate stable version
    let sidebar_contents = sidebar_contents.replace(
        "/docs",
        &format!("{}/{}/docs/{}", get_path_prefix_server(), &locale, &version),
    );
    let sidebar_html_contents = parse_md_to_html(&sidebar_contents);

    // Work out the status of this page
    let status = if version == "next" {
        DocsVersionStatus::Next
    } else if OUTDATED_VERSIONS.keys().any(|v| v == version) {
        DocsVersionStatus::Outdated
    } else if BETA_VERSIONS.keys().any(|v| v == version) {
        DocsVersionStatus::Beta
    } else if STABLE_VERSION_NAME.as_str() == version {
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

#[engine_only_fn]
pub async fn get_build_paths() -> Result<BuildPaths, walkdir::Error> {
    // We start off by rendering the `/docs` page itself as an alias
    let mut paths = vec!["".to_string()];
    // Get the `docs/` directory (relative to `.perseus/`)
    let docs_dir = PathBuf::from("../docs");
    // Loop through it
    for entry in WalkDir::new(docs_dir) {
        let entry = entry?;
        let path = entry.path();
        // Ignore any empty directories or the like
        if path.is_file() {
            // This should all pass, there are no non-Unicode filenames in the docs (and
            // i18n titles are handled outside filenames) Also, all these are
            // relative, which means we can safely strip away the `../docs/`
            // We also remove the file extensions (which are all `.md`)
            let path_str = path.to_str().unwrap().replace(".md", "");
            let path_str = path_str.strip_prefix("../docs/").unwrap();
            // Only proceed for paths in the default locale (`en-US`), which we'll use to
            // generate paths Also disallow any of the `SUMMARY.md` files at
            // this point (the extension has been stripped) Also disallow the
            // manifest file
            if path_str.contains("en-US/")
                && !path_str.ends_with("SUMMARY")
                && !path_str.ends_with("manifest.json")
            {
                // Now remove that locale (it'll be put at the front of the path in the URL)
                let path_str = path_str.replace("en-US/", "");
                // This path should be rendered!
                paths.push(path_str.clone());
                // If it's for the latest stable version though, we should also render it
                // without that prefix That way the latest stable verison is
                // always at the docs without a version prefix (which I think is more sensible
                // than having the unreleased version there)
                if path_str.starts_with(STABLE_VERSION_NAME.as_str()) {
                    let unprefixed_path_str = path_str
                        .strip_prefix(&format!("{}/", STABLE_VERSION_NAME.as_str()))
                        .unwrap();
                    paths.push(unprefixed_path_str.to_string());
                }
            }
        }
    }

    Ok(BuildPaths {
        paths,
        extra: ().into(),
    })
}
