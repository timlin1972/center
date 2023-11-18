use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

pub fn init() {
    let stdout = ConsoleAppender::builder().build();
    let logfile = FileAppender::builder().build(super::LOG_FILE).unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(super::LOG_LEVEL),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

// 建立一個輸出到文件的 Appender
// let file = FileAppender::builder()
//     .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
//     .build("log/output.log")
//     .unwrap();
