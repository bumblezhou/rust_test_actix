use actix_web::{cookie::Cookie, web, Error, App, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use actix_files::Files;
use local_ip_address::local_ip;

mod websocket;
use websocket::WebSocketActor;
use actix_web_actors::ws;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    server_ip: String,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    server_ip: String,
    username: String,
}

// Dummy user database
static USERS: &[(&str, &str)] = &[("user1", "password1"), ("user2", "password2")];

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FormParams {
    username: String,
    password: String,
}

async fn login_get() -> impl Responder {
    let my_local_ip = local_ip().unwrap();
    let login_temp = LoginTemplate { server_ip: my_local_ip.to_string() };
    let rendered = login_temp.render().unwrap();

    HttpResponse::Ok().body(rendered)
}

async fn login_post(form: web::Form<FormParams>) -> impl Responder {
    let username = &form.username;
    let password = &form.password;

    let is_authenticated = USERS.iter().any(|&(user, pass)| {
        username == user && password == pass
    });

    if is_authenticated {
        let cookie = Cookie::build("username", username.clone())
            .path("/")
            .finish();
        HttpResponse::Ok().cookie(cookie).body("Authenticated")
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

async fn home(req: HttpRequest) -> impl Responder {
    if let Some(username_cookie) = req.cookie("username") {
        let is_valid_user = USERS.iter().any(|&(user, _pass)| {
            username_cookie.value().to_owned() == user
        });
        if is_valid_user {
            let my_local_ip = local_ip().unwrap();
            let home_temp = HomeTemplate { server_ip: my_local_ip.to_string(), username: username_cookie.value().to_owned() };
            let rendered = home_temp.render().unwrap();
            return HttpResponse::Ok().body(rendered);
        } else {
            return HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish();
        }
    } else {
        return HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish();
    }
}

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(WebSocketActor {}, &r, stream);
    res
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/login", web::get().to(login_get))
            .route("/login", web::post().to(login_post))
            // Serve CSS files
            .service(Files::new("/css", "static/css"))
            // Serve JavaScript files
            .service(Files::new("/js", "static/js"))
            // Serve image files
            .service(Files::new("/img", "static/img"))
            // Serve font files
            .service(Files::new("/font", "static/font"))
            // WebSocket route
            .route("/ws/", web::get().to(ws_index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
