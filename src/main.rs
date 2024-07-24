use reqwest::{blocking::{self, Client}};
use clap::Parser;
use json::{self, JsonValue};
// I like camelCase :)
#[allow(non_snake_case)]

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
    //I don't want to deal with data conversion so we are storing this in a string.
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
    let pac = GetPackage(cli.name, &c);
    print!("{}", pac);

}
///Gets list of packages named exactly the input
fn GetPackage(name: String, client: &Client) -> JsonValue{
let content_result =  client.get("https://archlinux.org/packages/search/json/?name=".to_owned()+&name).send();
let content = match content_result {
    Ok(response) => response,
    //Print out an error and exit
    Err(error) => panic!("Unable to get page at https://archlinux.org/packages/search/json/?name={name:?}: {error:?}"),
};
let json_content = json::parse(&content.text().unwrap());
return json_content.unwrap();
}
