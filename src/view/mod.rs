use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "main_page.stpl")]
pub(crate) struct MainPageView {
    pub(crate) title: String,
    pub(crate) site: Site,
}

pub(crate) struct Site {
    pub(crate) name: String,
    pub(crate) description: String,
}