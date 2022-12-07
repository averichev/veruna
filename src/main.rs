use std::ops::Deref;
use veruna_kernel::sites::{site_kit, SiteBuilderImpl, SiteReadOption};
use assert_str::assert_str_eq;
use url::Url;
use veruna_data::SiteRepository;
use veruna_kernel::sites::site_kit::SiteKitFactory;


fn main() {
    let mut site_kit = SiteKitFactory::build(SiteRepository::new());
    let site_builder = site_kit.site_builder();
    let site = site_builder.build();
    let domain = site.domain();
    assert_str_eq!("domain.com", domain);
    let site_id = site_kit.create(site);
    let site_id_value = site_id.value();
    assert_eq!(site_id_value, 42);
    let url = Url::parse("http://averichev.tech").unwrap();
    let site = site_kit.get_site(url);
    assert_eq!(42, site.1.value());

    let site_id = site_kit.site_id_builder().build(56);
    let site_id_value = site_id.value();
    assert_eq!(site_id_value, 56);

    let reader = site_kit.reader();
    let site = reader.read(site_id);
}
