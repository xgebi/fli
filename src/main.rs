use clap::{App, Arg};

fn main() {
    let matches = App::new("fli")
        .subcommand(App::new("new")
            .about("Create something new")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .subcommand(App::new("package")
                .about("Create new package")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(Arg::new("path")
                    .about("path to package"))))
        .get_matches();
    if let Some(ref first_matches) = matches.subcommand_matches("new") {
        if let Some(ref second_matches) = first_matches.subcommand_matches("package") {
            if let Some(path) = second_matches.value_of("path") {
                println!("Value for path: {}", path);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::weight_on_mars;

    #[test]
    fn test_100_kg_on_mars() {
        assert_eq!(1, 1);
    }
}
