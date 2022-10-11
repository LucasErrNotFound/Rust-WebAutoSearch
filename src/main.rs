use std::process::{Command, exit};
use text_io::read;
mod edge;
mod chrome;
mod fire;

fn main(){

    Command::new("clear").status().unwrap();

    println!("\n ACTIVATED! PLEASE CHOOSE YOUR DESIRED OPTIONS:\n");
    println!(" [1] << AUTOCHROME\t<< www.google.com\n [2] << AUTOMSEDGE\t<< www.bing.com\n [3] << AUTOFIREFOX\t<< www.duckduckgo.com\n [4] << CONTRIBUTORS\n [5] << HELP\n [6] << LICENSE\n [7] << ABOUT\n [8] << EXIT");

        let choose: u16 = read!(); 

        match choose{

            1 => {
                Command::new("clear").status().unwrap();
                chrome::google();}

            2 => {
                Command::new("clear").status().unwrap();
                edge::bing();},

            3 => {
                Command::new("clear").status().unwrap();
                fire::firefox();},

            4 => {
                println!("Sorry not available yet...");
            },

            5 => {
                println!("Sorry not available yet...");
            },

            8 => {exit(0x0100);},


            _ => {
                println!("Nope");}
        };
        
}
