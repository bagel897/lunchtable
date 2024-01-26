mod config;
mod database;
mod graphql;
mod redis;
mod rocket;
pub(crate) use rocket::run_server;
