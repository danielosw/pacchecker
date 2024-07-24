use reqwest::{blocking::{self, Client}};
use clap::Parser;
use json::{self, JsonValue};

//Setup clap
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    ///Required name of package.
    #[arg(short,long)]
    name: String,

    ///Include AUR
    #[arg(short, long, default_value_t = false)]
    AUR: bool,

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
        //If we can't make a client we can't do anything elso so just panic.
        Err(error) => panic!("Could not construct client: {error:?}"),
    };
    //parse cli
    let cli = Cli::parse();
    //run the main program
    let pac = get_package(cli.name, &c);
    let pkg = get_data(pac);
    pkg_print(pkg);
}
///Print out pkg data
fn pkg_print(pkg: Package){
println!("Package name: {0}", pkg.name);
println!("Last updated: {0}", pkg.date);
println!("Repository: {0}", pkg.repository)
}
///Extract pkg data from json
fn get_data(j: JsonValue) -> Package{
if(!(j["results"].is_null())){
let results = j["results"][0].clone();
let pkg = Package{
name: results["pkgname"].to_string(),
date: results["last_update"].to_string(),
repository: results["repo"].to_string(),
};
return pkg;
}
else{
    panic!("Unable to get results.");
}
}

///Gets list of packages named exactly the input
fn get_package(name: String, client: &Client) -> JsonValue{
let content_result =  client.get("https://archlinux.org/packages/search/json/?name=".to_owned()+&name).send();
let content = match content_result {
    Ok(response) => response,
    //Print out an error and exit
    Err(error) => panic!("Unable to get page at https://archlinux.org/packages/search/json/?name={name:?}: {error:?}"),
};
let json_content = json::parse(&content.text().unwrap());
return json_content.unwrap();
}
