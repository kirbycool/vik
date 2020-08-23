use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

pub fn init_logger() {
    let debug_encoder = PatternEncoder::new("{m}\n\n");
    let debug = FileAppender::builder()
        .append(false)
        .encoder(Box::new(debug_encoder))
        .build("debug.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("debug", Box::new(debug)))
        .build(Root::builder().appender("debug").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
