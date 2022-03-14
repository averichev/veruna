use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "product.stpl")]
pub struct ProductView {
    pub id: i32,
    pub name: String
}