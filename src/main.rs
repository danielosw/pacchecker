use reqwest::blocking::Client;
use clap::Parser;
use json::{self, JsonValue};
use std::{env::current_dir, fs::{self, ReadDir}, thread, time::Duration};
//Setup clap
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    //Required name of package.
    

}
///pacman package struct
struct Package {
    name: String,
    //I don't want to deal with date conversion so we are storing this in a string.
    date: String,
    repository: String,

}
fn main() {
    //Build reqwest client
    let c_result = Client::builder()
        .build();
    let c = match c_result {
        Ok(client) => client,
        //If we can't make a client we can't do anything else so so just panic.
        Err(error) => panic!("Could not construct client: {error:?}"),
    };
    //parse cli
    //let cli = Cli::parse();
    //run the main program
    let mut pac = get_packages(&c);
    for item in pac{
        for (i, j) in item.entries(){
            println!("{}",i);
        }
    }
}
///Print out pkg data
fn pkg_print(pkg: Package){
println!("Package name: {0}", pkg.name);
println!("Last updated: {0}", pkg.date);
println!("Repository: {0}", pkg.repository)
}
///Extract pkg data from json
fn get_data(j: JsonValue) -> Package{
if !(j["results"].is_null() || j["results"][0].is_null()) {
let results = j["results"][0].clone();
Package{
name: results["pkgname"].to_string(),
date: results["last_update"].to_string(),
repository: results["repo"].to_string(),
}
}
else{
    panic!("Unable to get results.");
}
}

///Gets list of packages named exactly the input
fn get_packages(client: &Client) -> Vec<JsonValue>{

let content_result =  client.get("https://archlinux.org/packages/search/json/?q=").send();
let content = match content_result {
    Ok(response) => response,
    //Print out an error and exit
    Err(error) => panic!("Unable to get page at https://archlinux.org/packages/search/json/?q= {error:?}"),
};
let mut cached = true;
let json_content = unwap_json(content.text().unwrap());
let x: u8= json_content["num_pages"].to_string().parse().unwrap();
let test = fs::read_dir(current_dir().unwrap().to_str().unwrap().to_string()+&"/cache/").unwrap();
let coolvec: Vec<String> = test.map(|m| m.unwrap().path().to_string_lossy().to_string()).collect();
if coolvec.len().to_string() != x.to_string(){
    cached = false;
}
let mut jsonvec: Vec<JsonValue> = vec![];
if cached ==false{let mut i = 1;
while i <= x{
let url = "https://archlinux.org/packages/search/json/?q=&page=".to_string()+&i.to_string();
println!("{}",url);
let content_unsafe = client.get(url).send();
let temp = match content_unsafe {
    Ok(response) => response,
    Err(error) => panic!("Unable to get page at https://archlinux.org/packages/search/json?q=&page= : {error:?}"),
};
let text = temp.text().unwrap();
fs::write(current_dir().unwrap().to_str().unwrap().to_string()+"/cache/"+&i.to_string()+".json", text.clone());
thread::sleep(Duration::from_secs(3));
println!("got page");
i += 1;
}}
for i in coolvec{
    let temp = fs::read_to_string(i).unwrap();
    let jso = unwap_json(temp);
    let test = jso["results"].clone();
    jsonvec.push(test);
}
return jsonvec;
}

fn unwap_json(src: String) -> JsonValue{
    let unsafe_json = json::parse(&src);
    let json_content = match unsafe_json{
        Ok(jso) => jso,
        Err(error) => panic!("Unable to parse json: {error:?}"),
    };
    return json_content
}
