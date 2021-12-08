use std::fs::File;
use std::io::BufReader;

use rss::Channel;

extern crate clap;
use clap::{Arg, App};

use colored::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn check_for_link(channel: Channel) -> i32 {
    let mut errors = 0;
    
    if channel.link.len()>0 {
        println!("{}  {}", "Link:", channel.link.green());
    }else{
        println!("{}", "No link found!".red());
        errors += 1;
    }
    return errors;
}

fn check_for_title(channel: Channel) -> i32 {
    let mut errors = 0;
    
    if channel.title.len()>0 {
        println!("{} {}", "Title:", channel.title.green());
    }else{
        println!("{}", "No title found!".red());
        errors += 1;
    }
    return errors;
}

// check for a <itunes:image> or <image>
fn check_for_image(channel: Channel) -> i32 {
    let mut errors = 0;
    
    let imagereport = match channel.image {
        Some(image) => format!("<image> tag with url:\n    \"{}\"",
                                image.url).green(),
        None => match channel.itunes_ext() {
            None => {errors += 1; "no <image> or itunes extensions found".red()},
            Some(ext) => match ext.image() {
                None => {errors += 1; "no <image> or <itunes:image> tag found".red()},
                Some(imageurl) => format!("<itunes:image> tag with url:\n    \"{}\"",
                                        imageurl).green(),
            },
        },
    };
    println!("Image: {}", imagereport);
    
    return errors;
}

fn main() {
    let matches = App::new("PodCheck")
                          .version(VERSION)
                          .author("Thatcher Chamberlin <j.thatcher.c@gmail.com>")
                          .about("Podcast XML file validator")
                          .arg(Arg::with_name("FILE")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .get_matches();
                               
    let f = File::open(matches.value_of("FILE").unwrap()).expect("couldn't open file");
    let f = BufReader::new(f);

    let channel = Channel::read_from(f).unwrap();
    
    let mut errors = 0;

    println!("");

    // requires a <link>
    errors += check_for_link(channel.clone());
    
    // requires a <title>
    errors += check_for_title(channel.clone());
    
    // requires a <itunes:image> or <image>
    errors += check_for_image(channel.clone());
    
    let numitems = channel.items.len();
    println!("\n{} Items:", numitems);
    
    // item checks:
    for i in 0..numitems {
        let item = &channel.items[i];
        
        // requires a <title>
        let title = match &item.title {
            Some(title) => format!("({}/{}) \"{}\"",
                                    i+1, numitems, title).green(),
            None => {errors += 1; format!("({}/{}) No title!",i+1, numitems).red()},
        };
        
        // requires an <enclosure>
        let report = match &item.enclosure {
            Some(enclosure) => format!("    url:  {}\n    type: {}\n    length: {} bytes",
                                        enclosure.url, enclosure.mime_type, enclosure.length).green(),
            None => {errors += 1; "    No enclosure found".red()},
        };
        println!("{}", title);
        println!("{}", report);
    }

    println!("{} errors", errors);
}
