use std::io::{stdin, stdout, Write};
use reqwest::Client;

#[tokio::main]
async fn main(){
    loop{
        print!("Enter a Number: ");
        stdout().flush().expect("TODO: panic message");
        let mut int = String::new();
        stdin().read_line(&mut int).expect("cannot readline");
        if int.parse::<isize>().is_ok() {
            let url = format!("http://numbersapi.com/{}", int);
            let res =  Client::new().get(url).send()
                .await.expect("cannot").text().await.expect("cannot");
            println!("{}", res)
        } else {
            println!("please enter an integer")
        }
    }
}
