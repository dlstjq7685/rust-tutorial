mod file_api;
mod socket_api;
mod sort_api;
mod search_api;
mod db_api;
mod rand_api;
mod conf_api;
mod log_api;
mod math_api;

use crate::file_api::module::call as fs;
use crate::socket_api::module::call as sock;
use crate::sort_api::module::call as sort;
use crate::search_api::module::call as search;
use crate::db_api::module::call as db;
use crate::conf_api::module::call as conf;
use crate::rand_api::module::call as rand;
use crate::log_api::module::call as log;
use crate::math_api::module::call as math;

fn main() {
    println!("Hello, world!");
    fs();
    sock();
    sort();
    search();
    db();
    conf();
    rand();
    log();
    math();
}
