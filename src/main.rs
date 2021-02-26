mod checkvuln;
use checkvuln::{checkvuln};
use std::env;
use clap::{Arg, App};


fn main() {

    let matches = App::new("VMware vCenter CVE-2021-21972 checker")
                    .version("0.1")
                    .author("bt0 bt0@tutanota.com")
                    .about("Check for CVE-2021-21972")
                    .arg(Arg::with_name("host")
                            .short("H")
                            .long("host")
                            .value_name("Host IP or domain")
                            .help("Host IP or Domain to be checked for CVE")
                            .takes_value(true)
                            .required(true))
                    .arg(Arg::with_name("ID")
                            .short("i")
                            .long("test-id")
                            .value_name("ID [1-3]")
                            .help("Chose the URL to test by number 1-3")
                            .takes_value(true))
                    .arg(Arg::with_name("all")
                            .short("a")
                            .long("all")
                            .help("Perform all tests on all URLs")
                            .takes_value(true))
                    .get_matches();

    let ip = matches.value_of("host").unwrap();

    checkvuln(ip.to_string());    
    
}
