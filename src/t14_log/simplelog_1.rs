extern crate log;
extern crate simplelog;


 use log::*;
#[allow(unused_imports)]
#[allow(dead_code)]
use simplelog::*;
#[allow(unused_imports)]
#[allow(dead_code)]
use std::fs::File;

#[test]
fn simple_log_1() {
    fn x() {
        CombinedLogger::init(vec![
            // #[cfg(feature = "termcolor")]
            //     TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),

            TermLogger::new(LevelFilter::Info,
                            Config::default(),
                            TerminalMode::Mixed),
            WriteLogger::new(LevelFilter::Info,
                             Config::default(),
                             File::create("simple.log").unwrap()),
        ])
            .unwrap();

        error!("Bright red error");
        error!("Bright red error");

        info!("This only appears in the log file");
        info!("This only appears in the log file");

        debug!("This level is currently not enabled for any logger");
        debug!("This level is currently not enabled for any logger");
    }

    x();

    super::test_log();
}
