use reqwest::ClientBuilder;
use reqwest::Error;
use reqwest::header;
use colorful::{Color, Colorful};

#[tokio::main]
pub async fn checkvuln(host: String) -> Result<(), Error> {

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Length", header::HeaderValue::from_static("0"));

    let url1 = format!("https://{}/ui/vropspluginui/rest/services/getstatus", host);
    let url2 = format!("https://{}/ui/vropspluginui/rest/services/uploadova", host);
    let url3 = format!("https://{}/ui/vropspluginui/rest/services/uploadova", host);
  
    println!("{}", "VMware vCenter CVE-2021-21972 checker".color(Color::Yellow).bold());
    println!("{}", "v.0.1 by bt0".color(Color::Yellow).bold());
    println!("{}", "=====================================".color(Color::Yellow).bold());
    println!("{}", "Disclaimer: Do not use it on hosts that you don't own!".color(Color::Red).bold());
    println!("{}", "Just for Educational Purposes".color(Color::Red).bold());
    println!("\n[+] Testing Host: {}\n",host.to_string().color(Color::Black).bg_color(Color::Yellow).bold());
   
    
    // Testing 1st URL using get method
    println!("[+] Testing url 1: {}",url1);
    let client = ClientBuilder::new().danger_accept_invalid_certs(true).build()?;
    let response = client.get(&url1).send().await?;

    if response.status().is_success() {
        println!("{}", "[-] Host Not Vulnerable\n".color(Color::Green));
    } else if response.status().is_server_error() {
        let body = response.text().await?;
        if body.contains("uploadFile") {
            println!("{}", "[+] Host is Vulnerable\n".color(Color::Red).bold());
        } else {
        println!("{}", "[-] Host Not Vulnerable\n".color(Color::Green));
        }
    } else {
        println!("{}", "[-] Host Not Vulnerable\n".color(Color::Green));
    }

    // Testing 2nd URL using get method
    println!("[+] Testing url 2: {}",url2);
    let client = ClientBuilder::new().danger_accept_invalid_certs(true).build()?;
    let response = client.get(&url2).send().await?;

    if response.status().is_success() {
        println!("{}", "[-] Host Not Vulnerable\n".color(Color::Green));
    } else if response.status().is_server_error() {
        let body = response.text().await?;
        if body.contains("uploadFile") {
            println!("{}", "[+] Host is Vulnerable\n".color(Color::Red).bold());
        } else {
        println!("{}", "[-] Host Not Vulnerable\n".color(Color::Green));
        }
    } else {
        println!("{}", "[-] Host Not Vulnerable\n".color(Color::Green));
    }

    // Testing 3rd URL using post method
    
    println!("[+] Testing url 3: {}",url3);
    let client = ClientBuilder::new().danger_accept_invalid_certs(true).default_headers(headers).build()?;
    let response = client.post(&url3).header("Content-Lenght", "0").send().await?;
    
    if response.status().is_success() {
        println!("{}", "[-] Host Not Vulnerable\n".color(Color::Green));
    } else if response.status().is_server_error() {
        let body = response.text().await?;
        if body.contains("uploadFile") {
            println!("{}", "[+] Host is Vulnerable\n".color(Color::Red).bold());
        } else {
        println!("{}", "[-] Host Not Vulnerable\n".color(Color::Green));
        }
    } else {
        println!("{}", "[-] Host Not Vulnerable\n".color(Color::Green));
    }

    Ok(())
}