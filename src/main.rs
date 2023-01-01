use std::io::{stdin, stdout, Write};
use reqwest::Client;

#[tokio::main]
async fn main(){
    loop{
        print!("Enter a Number: ");
        stdout().flush().expect("error flushing");
        let mut int = String::new();
        
        // Enter an integer
        stdin().read_line(&mut int).expect("Cannot read your input!");
        
        if int.parse::<isize>().is_ok() {
            let url = format!("http://numbersapi.com/{}", int);
            
            // Gets the Response
            let res =  Client::new().get(url).send()
                .await.expect("Cannot connect to the API!").text().await.expect("Cannot get the text response!");
            println!("{}", res)
        } else {
            println!("The number must be an integer!")
        }
    }
}
