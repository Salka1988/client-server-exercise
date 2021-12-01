#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate lazy_static;

use std::{env, io};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use actix_web::{App, HttpServer, middleware, FromRequest, HttpRequest, dev};
use actix_web::HttpResponse;
use actix_web::web::Json;
use actix_web::web::Path;
use chrono::{DateTime, Utc};
use job_scheduler::{Job, JobScheduler};
use serde::{Deserialize, Serialize};

use server_client::Currency;
use server_client::types::summary::{CurrencyEnum, Currencies};
use std::error::Error;
use std::future::Ready;
use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
use actix_web::dev::Payload;


use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;
use std::borrow::Borrow;

pub const APPLICATION_JSON: &str = "application/json";


lazy_static! {
    static ref ARRAY: RwLock<Currencies> = RwLock::new(Currencies::new());
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // let mut currency_vector = Currencies::new();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(get_currency)
    })
        .bind("0.0.0.0:9090")?
        .run();
        // .await;

    let mut scheduler = JobScheduler::new();

    scheduler.add(Job::new("1/2 * * * * *".parse().unwrap(), || {

        let mut cur = ARRAY.try_write().unwrap();
        cur.currencies_vec.iter_mut().for_each(|a| {
            let mut guard = a.try_write().unwrap();
            guard.update();
        });

    }));

    loop {
        scheduler.tick();
        std::thread::sleep(Duration::from_millis(100));
    }

}

#[get("/currency/{id}")]
pub async fn get_currency(path: Path<(String,)>) -> HttpResponse {

    let mut guard = ARRAY.try_read().unwrap();
    let vec = &guard.currencies_vec;

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(vec)

    // match currency {
    //     Some(currency) => HttpResponse::Ok()
    //         .content_type(APPLICATION_JSON)
    //         .json(currency.currencies_vec),
    //     None => HttpResponse::NoContent()
    //         .content_type(APPLICATION_JSON)
    //         .await
    //         .unwrap(),
    // }
}


