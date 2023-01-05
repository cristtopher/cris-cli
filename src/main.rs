use std::env;
use std::collections::HashMap;

mod job;

#[derive(Debug)]
struct Service {
  name: String,
  endpoints: Vec<String>,
  mappings: HashMap<String, String>,
}

fn main() {
  let svc = Service {
    name: "nginx".to_string(),
    endpoints: vec!["192.168.0.1".to_string(), "192.168.0.2".to_string()],
    mappings: HashMap::from([
        ("pod-0".to_string(), "192.168.0.1".to_string()),
        ("pod-1".to_string(), "192.168.0.2".to_string()),
    ]),
  };
  let args: Vec<String> = env::args().collect();

  let query = &args[1];
  let file_path = &args[2];

  println!("Searching for {}", query);
  println!("In file {}", file_path);
  println!("Name Service {:?}", svc.name);
  println!("endpoint Service {:?}", svc.endpoints);
  println!("maaping Service {:?}", svc.mappings);

  job::add();
}
