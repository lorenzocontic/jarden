use std::collections::BTreeMap;

pub fn print(

    groups:&BTreeMap<String,Vec<String>>

){

    println!("Scanning Screenshots...\n");

    println!("Destination\n");

    let mut total = 0;

    for (month,files) in groups{

        println!("{}/",month);

        for file in files{

            println!("    {}",file);

            total+=1;

        }

        println!();

    }

    println!("Folders to create : {}",groups.len());

    println!("Files processed : {}",total);

}
