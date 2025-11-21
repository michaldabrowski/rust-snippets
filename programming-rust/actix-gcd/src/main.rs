use actix_web::{App, HttpResponse, HttpServer, web};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving on http://localhost:3000...");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(compute_gcd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}

async fn compute_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }
    let result = gcd(form.n, form.m);
    HttpResponse::Ok().content_type("text/html").body(format!(
        "The greatest common divisor of {} and {} is <b>{}</b>\n",
        form.n, form.m, result
    ))
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}
