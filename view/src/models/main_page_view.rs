use sailfish::TemplateOnce;
use crate::models::component_item::ComponentItem;

#[derive(TemplateOnce)]
#[template(path = "main_page.stpl")]
pub struct MainPageView {
    pub components: Vec<ComponentItem>,
}