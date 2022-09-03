// extern crate leaf;
// use crate::leaf::Manager;
use std::env;

use leaf::Manager;

use log::LevelFilter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut log_level = Some(LevelFilter::Info);
    if args.len() > 1 {
        if args[1] == "debug" {
            log_level = Some(LevelFilter::Debug);
        } else if args[1] == "brooke" {
            println!("hi brooke");
        } else {
            println!("Get fucked that's not a valid log level");
        }
    }
    let mut app: leaf::Application = leaf::Application::create_application(log_level);
    app.startup();
    app.run();
}
