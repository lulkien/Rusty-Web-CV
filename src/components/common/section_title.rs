use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SectionTitleProps {
    pub title: String,

    #[prop_or(None)]
    pub class: Option<String>,
}

#[function_component(SectionTitle)]
pub fn section_title(props: &SectionTitleProps) -> Html {
    let classes = format!(
        "section-title{}",
        props
            .class
            .as_ref()
            .map(|c| format!(" {}", c))
            .unwrap_or_default()
    );

    html! {
        <h2 class={ classes }>{ &props.title }</h2>
    }
}
