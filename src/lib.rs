fn parse_config(args: &[String]) -> (&str, &str){
    // &args[0] = /path/to/rust/binary/main.rs
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}