use std::collections::BTreeMap;

use crate::models::Screenshot;

pub fn group(
    files:&Vec<Screenshot>
) -> BTreeMap<String,Vec<String>> {

    let mut map = BTreeMap::new();

    for file in files {

        map.entry(file.month.clone())

            .or_insert(Vec::new())

            .push(file.name.clone());

    }

    map

}
