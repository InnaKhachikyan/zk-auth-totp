mod auth;
mod storage;
mod network;

use auth::register_user;

fn main() {

    register_user();    
}
