use crate::models::Resource;
use tabled::{Table, Tabled};
use uuid::Uuid;

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}
#[derive(Tabled)]
pub struct ResourcePrint {
    rid: String,
    rname: String,
    rpath: String,
}

pub fn print_resources(resources: Vec<Resource>) -> () {
    let mut resource_print: Vec<ResourcePrint> = Vec::new();

    for resource in resources {
        resource_print.push(ResourcePrint {
            rid: resource.rid,
            rname: resource.rname,
            rpath: resource.rpath,
        })
    }

    let table = Table::new(resource_print);

    println!("{}", table);
}
