mod simplelog_1;
mod log4rs_1;

#[allow(unused_imports)]
#[allow(dead_code)]
pub fn test_log() {

     use log::*;
    // use simplelog::*;

    error!("----------Bright red error");
    error!("----------Bright red error");

    info!("----------This only appears in the log file");
    info!("----------This only appears in the log file");
}