use std::rc::Rc;
use sycamore::prelude::*;

#[cfg(client)]
use sycamore::futures::spawn_local;

#[cfg(client)]
use wasm_bindgen::JsCast;

#[cfg(client)]
use wasm_bindgen::prelude::*;

// JavaScript binding for document.execCommand (deprecated but still works)
#[cfg(client)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = document, js_name = execCommand)]
    fn exec_command_js(command: &str, show_ui: bool, value: &str) -> bool;
}

/// WYSIWYG Editor Properties
#[derive(Props)]
pub struct EditorProps {
    /// Initial HTML content
    #[prop(default)]
    pub initial_content: String,
    /// Placeholder text when empty
    #[prop(default = "Start writing...".to_string())]
    pub placeholder: String,
    /// Callback when content changes - receives HTML string
    pub on_change: Option<Rc<dyn Fn(String) + 'static>>,
    /// Minimum height in pixels
    #[prop(default = 300)]
    pub min_height: u32,
    /// Enable image upload
    #[prop(default = true)]
    pub enable_images: bool,
    /// Image upload endpoint URL
    #[prop(default = "/uploads/image".to_string())]
    pub upload_url: String,
}

/// WYSIWYG Editor Component
#[component]
pub fn WysiwygEditor(props: EditorProps) -> View {
    let editor_ref = create_node_ref();
    let content = create_signal(props.initial_content.clone());
    let is_uploading = create_signal(false);
    let upload_progress = create_signal(0u32);
    let show_link_modal = create_signal(false);
    let link_url = create_signal(String::new());
    let link_text = create_signal(String::new());

    let min_height = props.min_height;
    let enable_images = props.enable_images;
    let upload_url = props.upload_url.clone();
    let on_change = props.on_change;
    let initial_content = props.initial_content.clone();

    // Execute formatting command
    #[cfg(client)]
    let exec_command = |command: &str, value: Option<&str>| {
        exec_command_js(command, false, value.unwrap_or(""));
    };

    // Notify content change - wrapped in Rc for cloning
    let notify_change: Rc<dyn Fn()> = {
        let editor_ref = editor_ref.clone();
        let on_change = on_change.clone();
        Rc::new(move || {
            #[cfg(client)]
            {
                if let Some(ref on_change) = on_change {
                    let node = editor_ref.get();
                    // Use dyn_ref instead of dyn_into to avoid consuming the node
                    if let Some(element) = node.dyn_ref::<web_sys::HtmlElement>() {
                        let html = element.inner_html();
                        content.set(html.clone());
                        on_change(html);
                    }
                }
            }
        })
    };

    // Format button handlers - each clones notify_change
    let format_bold = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("bold", None);
            notify_change();
        }
    };

    let format_italic = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("italic", None);
            notify_change();
        }
    };

    let format_underline = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("underline", None);
            notify_change();
        }
    };

    let format_strikethrough = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("strikeThrough", None);
            notify_change();
        }
    };

    let format_h1 = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("formatBlock", Some("h1"));
            notify_change();
        }
    };

    let format_h2 = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("formatBlock", Some("h2"));
            notify_change();
        }
    };

    let format_h3 = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("formatBlock", Some("h3"));
            notify_change();
        }
    };

    let format_paragraph = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("formatBlock", Some("p"));
            notify_change();
        }
    };

    let format_ul = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("insertUnorderedList", None);
            notify_change();
        }
    };

    let format_ol = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("insertOrderedList", None);
            notify_change();
        }
    };

    let format_quote = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("formatBlock", Some("blockquote"));
            notify_change();
        }
    };

    let format_code = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("formatBlock", Some("pre"));
            notify_change();
        }
    };

    let format_align_left = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("justifyLeft", None);
            notify_change();
        }
    };

    let format_align_center = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("justifyCenter", None);
            notify_change();
        }
    };

    let format_align_right = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("justifyRight", None);
            notify_change();
        }
    };

    let format_undo = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("undo", None);
            notify_change();
        }
    };

    let format_redo = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("redo", None);
            notify_change();
        }
    };

    let format_remove_format = {
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            exec_command("removeFormat", None);
            notify_change();
        }
    };

    // Link modal handlers
    let open_link_modal = move |_| {
        link_url.set(String::new());
        link_text.set(String::new());

        #[cfg(client)]
        {
            // Get selected text
            if let Some(window) = web_sys::window() {
                if let Some(selection) = window.get_selection().ok().flatten() {
                    let selected: String = selection.to_string().into();
                    if !selected.is_empty() {
                        link_text.set(selected);
                    }
                }
            }
        }

        show_link_modal.set(true);
    };

    let close_link_modal = move |_| {
        show_link_modal.set(false);
    };

    let insert_link: Rc<dyn Fn()> = {
        let notify_change = notify_change.clone();
        Rc::new(move || {
            let url = link_url.get_clone();
            let text = link_text.get_clone();

            if !url.is_empty() {
                #[cfg(client)]
                {
                    let html = if text.is_empty() {
                        format!("<a href=\"{}\" target=\"_blank\">{}</a>", url, url)
                    } else {
                        format!("<a href=\"{}\" target=\"_blank\">{}</a>", url, text)
                    };
                    exec_command("insertHTML", Some(&html));
                }
            }

            show_link_modal.set(false);
            notify_change();
        })
    };

    // Image upload via file input
    let file_input_ref = create_node_ref();

    let trigger_image_upload = {
        let file_input_ref = file_input_ref.clone();
        move |_| {
            #[cfg(client)]
            {
                let node = file_input_ref.get();
                if let Ok(input) = node.dyn_into::<web_sys::HtmlInputElement>() {
                    input.click();
                }
            }
        }
    };

    let handle_file_select = {
        let upload_url = upload_url.clone();
        let editor_ref = editor_ref.clone();
        let notify_change = notify_change.clone();
        move |_| {
            #[cfg(client)]
            {
                let file_input_ref = file_input_ref.clone();
                let upload_url = upload_url.clone();
                let editor_ref = editor_ref.clone();
                let notify_change = notify_change.clone();

                spawn_local(async move {
                    let node = file_input_ref.get();
                    if let Ok(input) = node.dyn_into::<web_sys::HtmlInputElement>() {
                        if let Some(files) = input.files() {
                            if let Some(file) = files.get(0) {
                                is_uploading.set(true);
                                upload_progress.set(0);

                                match upload_image(&upload_url, &file).await {
                                    Ok(url) => {
                                        // Insert image into editor
                                        let img_html = format!(
                                            "<img src=\"{}\" alt=\"Uploaded image\" style=\"max-width: 100%; height: auto;\"/>",
                                            url
                                        );

                                        // Focus editor and insert
                                        let node = editor_ref.get();
                                        if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>()
                                        {
                                            element.focus().ok();
                                            exec_command_js("insertHTML", false, &img_html);
                                        }
                                        notify_change();
                                    }
                                    Err(e) => {
                                        web_sys::console::error_1(
                                            &format!("Upload failed: {}", e).into(),
                                        );
                                    }
                                }

                                is_uploading.set(false);
                                // Reset file input
                                input.set_value("");
                            }
                        }
                    }
                });
            }
        }
    };

    // Handle paste with images
    let handle_paste = {
        let upload_url = upload_url.clone();
        let editor_ref = editor_ref.clone();
        let notify_change = notify_change.clone();
        move |e: sycamore::web::events::Event| {
            #[cfg(client)]
            if enable_images {
                let upload_url = upload_url.clone();
                let editor_ref = editor_ref.clone();
                let notify_change = notify_change.clone();

                if let Ok(clipboard_event) = e.dyn_into::<web_sys::ClipboardEvent>() {
                    if let Some(data) = clipboard_event.clipboard_data() {
                        let items = data.items();
                        for i in 0..items.length() {
                                if let Some(item) = items.get(i) {
                                    let item_type = item.type_();
                                    if item_type.starts_with("image/") {
                                        clipboard_event.prevent_default();

                                        if let Some(file) = item.get_as_file().ok().flatten() {
                                            spawn_local(async move {
                                                is_uploading.set(true);

                                                match upload_image(&upload_url, &file).await {
                                                    Ok(url) => {
                                                        let img_html = format!(
                                                            "<img src=\"{}\" alt=\"Pasted image\" style=\"max-width: 100%; height: auto;\"/>",
                                                            url
                                                        );

                                                        let node = editor_ref.get();
                                                        if let Ok(element) =
                                                            node.dyn_into::<web_sys::HtmlElement>()
                                                        {
                                                            element.focus().ok();
                                                            exec_command_js("insertHTML", false, &img_html);
                                                        }
                                                        notify_change();
                                                    }
                                                    Err(e) => {
                                                        web_sys::console::error_1(
                                                            &format!("Paste upload failed: {}", e)
                                                                .into(),
                                                        );
                                                    }
                                                }

                                                is_uploading.set(false);
                                            });
                                        }
                                        break;
                                    }
                            }
                        }
                    }
                }
            }
        }
    };

    // Handle content input
    let handle_input = {
        let notify_change = notify_change.clone();
        move |_| {
            notify_change();
        }
    };

    let editor_style = format!(
        "min-height: {}px; padding: 1rem; border: 1px solid #ddd; border-radius: 0 0 8px 8px; outline: none; overflow-y: auto;",
        min_height
    );

    // Prevent focus loss on mousedown
    let prevent_focus_loss = |e: web_sys::MouseEvent| {
        e.prevent_default();
    };

    view! {
        div(class="wysiwyg-editor") {
            // Toolbar
            div(class="wysiwyg-toolbar") {
                // Text formatting group
                div(class="toolbar-group") {
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Bold (Ctrl+B)",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_bold
                    ) { "B" }
                    button(
                        r#type="button",
                        class="toolbar-btn toolbar-italic",
                        title="Italic (Ctrl+I)",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_italic
                    ) { "I" }
                    button(
                        r#type="button",
                        class="toolbar-btn toolbar-underline",
                        title="Underline (Ctrl+U)",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_underline
                    ) { "U" }
                    button(
                        r#type="button",
                        class="toolbar-btn toolbar-strike",
                        title="Strikethrough",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_strikethrough
                    ) { "S" }
                }

                div(class="toolbar-separator") {}

                // Headings group
                div(class="toolbar-group") {
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Heading 1",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_h1
                    ) { "H1" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Heading 2",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_h2
                    ) { "H2" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Heading 3",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_h3
                    ) { "H3" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Paragraph",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_paragraph
                    ) { "P" }
                }

                div(class="toolbar-separator") {}

                // List group
                div(class="toolbar-group") {
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Bullet List",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_ul
                    ) { "UL" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Numbered List",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_ol
                    ) { "OL" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Quote",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_quote
                    ) { "\"" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Code Block",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_code
                    ) { "</>" }
                }

                div(class="toolbar-separator") {}

                // Alignment group
                div(class="toolbar-group") {
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Align Left",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_align_left
                    ) { "L" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Align Center",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_align_center
                    ) { "C" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Align Right",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_align_right
                    ) { "R" }
                }

                div(class="toolbar-separator") {}

                // Insert group
                div(class="toolbar-group") {
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Insert Link",
                        on:mousedown=prevent_focus_loss,
                        on:click=open_link_modal
                    ) { "Link" }
                    (if enable_images {
                        view! {
                            button(
                                r#type="button",
                                class="toolbar-btn",
                                title="Insert Image",
                                on:mousedown=prevent_focus_loss,
                                on:click=trigger_image_upload,
                                disabled=is_uploading.get()
                            ) {
                                (if is_uploading.get() { "..." } else { "Img" })
                            }
                        }
                    } else {
                        view! {}
                    })
                }

                div(class="toolbar-separator") {}

                // Undo/Redo group
                div(class="toolbar-group") {
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Undo (Ctrl+Z)",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_undo
                    ) { "Undo" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Redo (Ctrl+Y)",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_redo
                    ) { "Redo" }
                    button(
                        r#type="button",
                        class="toolbar-btn",
                        title="Remove Formatting",
                        on:mousedown=prevent_focus_loss,
                        on:click=format_remove_format
                    ) { "Clear" }
                }
            }

            // Hidden file input for image upload
            input(
                r#type="file",
                r#ref=file_input_ref,
                accept="image/*",
                style="display: none;",
                on:change=handle_file_select
            )

            // Editor content area
            div(
                r#ref=editor_ref,
                class="wysiwyg-content",
                contenteditable="true",
                style=editor_style,
                data-placeholder=props.placeholder,
                on:input=handle_input,
                on:paste=handle_paste,
                dangerously_set_inner_html=initial_content
            )

            // Link modal
            (if show_link_modal.get() {
                view! {
                    div(class="modal-overlay", on:click=close_link_modal) {
                        div(class="modal-content", on:click=|e: web_sys::MouseEvent| e.stop_propagation()) {
                            h3 { "Insert Link" }
                            div(class="form-group") {
                                label { "URL" }
                                input(
                                    r#type="url",
                                    placeholder="https://example.com",
                                    bind:value=link_url
                                )
                            }
                            div(class="form-group") {
                                label { "Text (optional)" }
                                input(
                                    r#type="text",
                                    placeholder="Link text",
                                    bind:value=link_text
                                )
                            }
                            div(class="modal-actions") {
                                button(
                                    r#type="button",
                                    class="secondary-btn",
                                    on:click=close_link_modal
                                ) { "Cancel" }
                                button(
                                    r#type="button",
                                    on:click={
                                        let insert_link = insert_link.clone();
                                        move |_| insert_link()
                                    }
                                ) { "Insert" }
                            }
                        }
                    }
                }
            } else {
                view! {}
            })

            // Upload progress indicator
            (if is_uploading.get() {
                view! {
                    div(class="upload-overlay") {
                        div(class="upload-spinner") {}
                        p { "Uploading image..." }
                    }
                }
            } else {
                view! {}
            })
        }
    }
}

/// Upload image to server
#[cfg(client)]
async fn upload_image(upload_url: &str, file: &web_sys::File) -> Result<String, String> {
    use gloo_net::http::Request;
    use wasm_bindgen::JsValue;

    let form_data = web_sys::FormData::new().map_err(|_| "Failed to create FormData")?;
    form_data
        .append_with_blob("file", file)
        .map_err(|_| "Failed to append file")?;

    let response = Request::post(upload_url)
        .body(JsValue::from(form_data))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("Upload failed with status: {}", response.status()));
    }

    #[derive(serde::Deserialize)]
    struct UploadResponse {
        success: bool,
        url: Option<String>,
        message: String,
    }

    let result: UploadResponse = response.json().await.map_err(|e| e.to_string())?;

    if result.success {
        result.url.ok_or_else(|| "No URL in response".to_string())
    } else {
        Err(result.message)
    }
}

/// Get current HTML content from editor
#[cfg(client)]
pub fn get_editor_content(editor_ref: &NodeRef) -> String {
    let node = editor_ref.get();
    if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
        element.inner_html()
    } else {
        String::new()
    }
}

/// Set HTML content in editor
#[cfg(client)]
pub fn set_editor_content(editor_ref: &NodeRef, content: &str) {
    let node = editor_ref.get();
    if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
        element.set_inner_html(content);
    }
}
