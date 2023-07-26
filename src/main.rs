/*
* this project will be a working websocket powered chat client, with message history.
*
* instead of sending text over the socket, it will send json in the form of rust structs that then can be parsed in order
* to make a message history in a simple TUI.
*
* there will be only one binary, but it can switch between being a server and being a client using a simple commandline flag.
*/

use std::process::exit;
mod client;
mod server;
mod types;

use client::run_client;
use server::run_server;

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let cli_type = args.next().unwrap_or_default();
    match cli_type.as_str() {
        "-client" | "-c" => run_client(),
        "-server" | "-s" => run_server(),
        _ => {
            println!("usage: wschat -[client|server]");
            exit(0);
        }
    }
}
