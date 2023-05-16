use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "main_page.stpl")]
pub(crate) struct MainPageView {
    pub(crate) title: String,
}