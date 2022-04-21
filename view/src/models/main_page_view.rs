use std::collections::HashMap;
use sailfish::TemplateOnce;
use serde_json::{Value, Map, Number};
use crate::models::model_list::ModelList;

#[derive(TemplateOnce)]
#[template(path = "main_page.stpl")]
pub struct MainPageView {
    pub components: Vec<ModelList>,
}