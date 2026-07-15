mod archive;
mod folders;
mod models;
mod organizer;
mod report;
mod sample;
mod scanner;

use organizer::group;
use report::print;
use scanner::Scanner;

fn main() {

    let files = sample::load();

    let scanned = Scanner::scan(files);

    let grouped = group(&scanned);

    archive::archive(&grouped);

    println!();

    print(&grouped);

}
