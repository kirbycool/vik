use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
};

pub fn init_logger() {
    let debug = FileAppender::builder()
        .append(true)
        .build("debug.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("debug", Box::new(debug)))
        .build(Root::builder().appender("debug").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
