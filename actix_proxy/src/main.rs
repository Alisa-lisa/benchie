#[macro_use]
extern crate diesel;
use actix::prelude::*;
use actix::sync::SyncArbiter;
use actix_web::http::Method;
use actix_web::{server, App, AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, Path, Result};
use actix_web::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use futures::future::Future;
use serde_derive::{Deserialize, Serialize};
use std::boxed::Box;
mod models;
mod schema;

struct DbExecutor(PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

struct CollectPast {
    passenger: i32,
    city: String,
}

#[derive(Default, Serialize, Deserialize, Queryable)]
struct PastDestination {
    pub id: i32,
    pub passenger: i32,
    pub city: String,
    pub street: String,
    pub street_number: i32,
    pub times_visited: i32,
}


#[derive(Serialize)]
struct Address {
    address: Vec<String>,
}


impl Message for CollectPast {
    type Result = Result<Address, DieselError>;
}

impl Handler<CollectPast> for DbExecutor {
    type Result = Result<Address, DieselError>;

    fn handle(&mut self, msg: CollectPast, _: &mut Self::Context) -> Self::Result {
        use self::schema::pastdestinations::dsl::*;

        let items = pastdestinations
            .filter(passenger.eq(&msg.passenger))
            .filter(city.eq(&msg.city))
            .load::<PastDestination>(&self.0)
            .expect("Error loading destinations");
        
        let mut res = Address{address: Vec::new()};
        for i in items {
            res.address.push(format!("{} {}", i.street, i.street_number.to_string()));            
        }
        
        Ok(res)
    }
}

struct State {
    db: Addr<DbExecutor>,
}

fn index(req: &HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    // get path parameters
    let passenger = &req.match_info()["passenger"];
    let pas = passenger.to_owned().parse::<i32>().unwrap();
    let city = &req.match_info()["city"];

    req.state()
        .db
        .send(CollectPast {
            passenger: pas,
            city: city.to_string(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(pastdestinations) => Ok(HttpResponse::Ok().json(pastdestinations)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn get_address(path_info: Path<(u32, String)>) -> Result<Json<Address>> {
    let mut res = Vec::new();
    res.push("Ololo Past 123".to_string());
    Ok(Json(Address {address: res}))
}

fn main() {
    let sys = actix::System::new("showcase");
    let db_uri = "postgres://postgres:postgres@172.20.0.2:5432/pastdestinations";
    // Avoid triggering "FATAL: the database system is starting up" error from
    // postgres.
    // TODO: add num_cpus crate instead fixed workers number
    let addr = SyncArbiter::start(4, move || {
        DbExecutor(
            PgConnection::establish(&db_uri).expect(&format!("Error connecting to {}", db_uri)),
        )
    });

    server::new(move || {
        App::with_state(State { db: addr.clone() })
            .resource("/{passenger}/{city}", |r| r.method(Method::GET).a(index))
            .resource("/hi", |r| r.method(Method::GET).with(get_address))
    })
    .bind("0.0.0.0:8089")
    .unwrap()
    .run();
}
