use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::{time::Duration, Key}, web, Error, App, HttpMessage as _, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use actix_files::Files;
use local_ip_address::local_ip;
use base64::prelude::*;

const THIRTY_MINUTES: Duration = Duration::minutes(30);

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

async fn login_post(form: web::Form<FormParams>, req: HttpRequest) -> impl Responder {
    let username = &form.username;
    let password = &form.password;

    let is_authenticated = USERS.iter().any(|&(user, pass)| {
        username == user && password == pass
    });

    if is_authenticated {
        Identity::login(&req.extensions(), username.clone().to_owned()).unwrap();
        HttpResponse::Ok().body("Authenticated")
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

async fn logout(id: Identity) -> impl Responder {
    id.logout();
    HttpResponse::Found().append_header(("Location", "/login")).finish()
}

async fn home(_req: HttpRequest, identity: Option<Identity>) -> impl Responder {
    match identity.map(|id| id.id()) {
        Some(Ok(id)) => {
            // println!("id: {}", id);
            let is_valid_user = USERS.iter().any(|&(user, _pass)| {
                id == user
            });
            if is_valid_user {
                let my_local_ip = local_ip().unwrap();
                let home_temp = HomeTemplate { server_ip: my_local_ip.to_string(), username: id };
                let rendered = home_temp.render().unwrap();
                return HttpResponse::Ok().body(rendered);
            } else {
                return HttpResponse::Found().append_header(("Location", "/login")).finish();
            }
        }
        _ => {
            return HttpResponse::Found().append_header(("Location", "/login")).finish();
        }
    }
}

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(WebSocketActor {}, &r, stream);
    res
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // If we generate the secret_key when restart the web, the login state of user will invalid after restarting.
    // let secret_key = Key::generate();

    // Generate a random secret key. Note that it is important to use a unique
    // secret key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    //
    // If the secret key is read from a file or the environment, make sure it is generated securely.
    // For example, a secure random key (in base64 format) can be generated with the OpenSSL CLI:
    // ```
    // openssl rand -base64 64
    // ```
    //
    // Then decoded and used converted to a Key:
    // ```
    // let secret_key = Key::from(base64::decode(&private_key_base64).unwrap());
    // ```
    let private_key_base64 = "Nwcz9uxDCnJue8DT3aiUo/bVxojMxqqvqDhfQcjOGLI1PSyNL1EtfcCC3Q0QOuvThIodFFuJ2l3lCiRySXqo8w==";
    // Decode Base64-encoded string
    let decoded = BASE64_STANDARD.decode(&private_key_base64);
    let secret_key = Key::from(&decoded.unwrap());

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(home))
            .route("/login", web::get().to(login_get))
            .route("/login", web::post().to(login_post))
            .route("/logout", web::get().to(logout))
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
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_name("username".to_owned())
                .cookie_secure(false)
                .session_lifecycle(PersistentSession::default().session_ttl(THIRTY_MINUTES))
                .build(),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
