use std::env;

fn main() {
    //Initialize the logging
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    match plc_driver::compile(&args, None) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e.get_message());
            std::process::exit(1);
        }
    }
}
