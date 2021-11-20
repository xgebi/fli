use std::path::{Path};
use std::fs::{self, File};
use std::env;
use std::io::Write;
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
                match path.rfind(".") {
                    None => {
                        let dir_path = Path::new(path);
                        if !dir_path.exists() {
                            fs::create_dir(dir_path);
                            let init_path = dir_path.clone().join("__init__.py");
                            File::create(init_path);
                        } else {
                            println!("Can't override existing module");
                            return;
                        }
                    }
                    Some(i) => {
                        let name = String::from(&path[i + 1..]);
                        let transformed = path.clone().replace(".", "/");
                        let dir_path = Path::new(transformed.as_str());
                        if !dir_path.exists() {
                            fs::create_dir_all(dir_path);
                            let init_path = dir_path.clone().join("__init__.py");
                            let mut init = File::create(init_path).unwrap();
                            init.write(format!("from flask import Blueprint\n{} = Blueprint('{}', __name__)\nfrom {} import routes", &name, &name, &path).as_ref());

                            let routes_path = dir_path.clone().join("routes.py");
                            let mut routes = File::create(routes_path).unwrap();
                            routes.write(format!("from {} import {}", &path, &name).as_ref());
                        } else {
                            println!("Can't override existing module");
                            return;
                        }
                    }
                }
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
