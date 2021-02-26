mod checkvuln;
use checkvuln::checkvuln;
use checkvuln::file_list;
use clap::{Arg, App};

fn main() {

    let matches = App::new("VMware vCenter CVE-2021-21972 checker")
                    .version("0.2")
                    .author("bt0 bt0@tutanota.com")
                    .about("Check for CVE-2021-21972")
                    .arg(Arg::with_name("host")
                            .short("H")
                            .long("host")
                            .value_name("Host IP or domain")
                            .help("Host IP or Domain to be checked for CVE")
                            .takes_value(true))
                            //.required(true))
                    .arg(Arg::with_name("file")
                            .short("f")
                            .long("file")
                            .help("File containing a list of IPs")
                            .takes_value(true))
                    .get_matches();

    if matches.is_present("host") {
        let ip = matches.value_of("host").unwrap();
        checkvuln(String::from(ip));
    } else if matches.is_present("file"){
        println!("Using file with ip list");
        let filename = matches.value_of("file").unwrap();
        file_list(filename.to_string());
    };    

        
    
}
