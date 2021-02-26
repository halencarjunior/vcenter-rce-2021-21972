use reqwest::ClientBuilder;
use reqwest::Error;
use reqwest::header;
use colorful::{Color, Colorful};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;

#[tokio::main]
pub async fn checkvuln(host: String) -> Result<(), Error> {
    //let timeout = Duration::new(2,0);

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Length", header::HeaderValue::from_static("0"));

    let url1 = format!("https://{}/ui/vropspluginui/rest/services/getstatus", host);
    let url2 = format!("https://{}/ui/vropspluginui/rest/services/uploadova", host);
  
    println!("{}", "VMware vCenter CVE-2021-21972 checker".color(Color::Yellow).bold());
    println!("{}", "v.0.2 by bt0".color(Color::Yellow).bold());
    println!("{}", "=====================================".color(Color::Yellow).bold());
    println!("{}", "Disclaimer: Do not use it on hosts that you don't own!".color(Color::Red).bold());
    println!("{}", "Just for Educational Purposes".color(Color::Red).bold());
    println!("\n[+] Testing Host: {}\n",host.to_string().color(Color::Black).bg_color(Color::Yellow).bold());
   
    
    // Testing 1st URL using get method
    println!("[+] Checking url: {}",url1);
    let client = ClientBuilder::new().danger_accept_invalid_certs(true).timeout(Duration::from_secs(2)).build()?;
    let response = client.get(&url1).send().await?;

    if response.status().is_success() {
        println!("{}", " ==> [+] Host is accepting requests without any auth. Will Test for POST\n".color(Color::Red));
        
        /* Testing URL2 with POST method */
        // Testing 2nd URL using post method
    
        println!("[+] Testing url for Vuln: {}",url2);
        let client = ClientBuilder::new().danger_accept_invalid_certs(true).default_headers(headers).timeout(Duration::from_secs(2)).build()?;
        let response = client.post(&url2).header("Content-Lenght", "0").send().await?;
        
        if response.status().is_success() {
            println!("{}", " [-] Host Not Vulnerable\n".color(Color::Green));
        } else if response.status().is_server_error() {
            let body = response.text().await?;
            if body.contains("uploadFile") {
                println!("{}", " [+] Host is Vulnerable\n".color(Color::Red).bold());
                println!("Workarounds: \nhttps://kb.vmware.com/s/article/82374\nhttps://kb.vmware.com/s/article/76372\n")
                
            } else {
            println!("{}", " [-] Host Not Vulnerable\n".color(Color::Green));
            }
        } else {
            println!("{}", " [-] Host Not Vulnerable\n".color(Color::Green));
        }
    } else if response.status().is_server_error() {
        let body = response.text().await?;
        if body.contains("uploadFile") {
            println!("{}", "  [+] Host is Vulnerable\n".color(Color::Red).bold());
        } else {
        println!("{}", "  [-] Host Not Vulnerable\n".color(Color::Green));
        }
    } else {
        println!("{}", "  [-] Host Not Vulnerable\n".color(Color::Green));
    }

    Ok(())
}

pub fn file_list(filename: String) {

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                checkvuln(ip);
                //println!("IP: {}", ip);
            }
        }
    }

    //let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //println!("Text:\n{}", contents);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}