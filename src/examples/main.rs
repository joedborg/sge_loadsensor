extern crate sge_loadsensor;
use sge_loadsensor::sensor;

fn test() -> std::string::String {
    return "123".to_string();
}

fn main() {
    sensor("test".to_string(), &test);
}