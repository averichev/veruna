use assert_str::assert_str_eq;
use url::Url;
use veruna_data::SiteRepository;
use veruna_kernel::sites::site_kit::SiteKitFactory;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


fn test() {
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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 20921))?
        .run()
        .await
}