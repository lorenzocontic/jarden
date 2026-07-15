use std::collections::BTreeMap;

pub fn archive(
    groups:&BTreeMap<String,Vec<String>>
){

    for month in groups.keys(){

        crate::folders::create(month);

    }

}
