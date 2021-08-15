// This page illustrates SSR

use serde::{Serialize, Deserialize};
use sycamore::prelude::{template, component, GenericNode, Template as SycamoreTemplate};
use perseus::template::Template;

#[derive(Serialize, Deserialize)]
pub struct IpPageProps {
    ip: String,
}

#[component(IpPage<G>)]
pub fn dashboard_page(props: IpPageProps) -> SycamoreTemplate<G> {
	template! {
		p {
            (
				format!("Your IP address is {}.", props.ip)
			)
        }
	}
}

pub fn get_page<G: GenericNode>() -> Template<G> {
    Template::new("ip")
        .request_state_fn(Box::new(get_request_state))
        .template(template_fn())
}

pub async fn get_request_state(_path: String) -> Result<String, String> {
    Ok(serde_json::to_string(
        &IpPageProps {
            ip: "x.x.x.x".to_string()
        }
    ).unwrap())
}

pub fn template_fn<G: GenericNode>() -> perseus::template::TemplateFn<G> {
    Box::new(|props: Option<String>| template! {
        IpPage(
            serde_json::from_str::<IpPageProps>(&props.unwrap()).unwrap()
        )
    })
}