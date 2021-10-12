use clap::{App, Arg, SubCommand};

#[test]
fn clap_1() {
    //---------------------
    println!("-----------start -----------");

    let m = App::new("my-app")
        // .author("Me, me@mail.com")
        // .version("1.0.2")
        // .about("Explains in brief what the program does")
        .arg(Arg::with_name("a").index(1))
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .get_matches();

    println!("-----------end -----------");

    for (k, v) in m.args {
        println!("-----------{:?}-: {:?}----------", k, v);
    }
}
