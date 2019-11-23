use std::process::Command;
use std::process;
use std::env;
use colored::*;

pub fn help_menu() {
    let version = String::from("0.0.1");
    println!("{}{}","NovusCLI Version: ",version);
    println!("Usage [options] [command]\n");

    println!("NovusCLI (nvs) is a commandline package manager. It uses dgpkg as its backend \nif you are familiar with Debian's apt. The usage remains the same. .deb files are able to be installed.\n");
    
    println!("Most used commands:");
    println!("\ list - lists all available packages")
}