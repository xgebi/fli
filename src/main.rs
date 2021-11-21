use clap::{App, Arg, ArgMatches};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let matches = App::new("fli")
        .subcommand(
            App::new("new")
                .about("Create something new")
                .version("1.0")
                .author("Sarah Gebauer <sarah@sarahgebauer.com>")
                .subcommand(
                    App::new("package")
                        .about("Create new package")
                        .version("1.0")
                        .author("Sarah Gebauer <sarah@sarahgebauer.com>")
                        .arg(Arg::new("path").about("path to package")),
                )
                .subcommand(
                    App::new("file")
                        .about("Create a new file")
                        .version("1.1")
                        .author("Sarah Gebauer <sarah@sarahgebauer.com>")
                        .arg(
                            Arg::new("name")
                                .about("Name of the python file")
                                .short("n".parse().unwrap())
                                .long("name")
                                .takes_value(true),
                        )
                        .arg(Arg::new("path").about("path to file's directory")),
                ),
        )
        .get_matches();
    if let Some(ref first_matches) = matches.subcommand_matches("new") {
        if let Some(ref second_matches) = first_matches.subcommand_matches("package") {
            create_new_package(second_matches);
        }
        if let Some(ref second_matches) = first_matches.subcommand_matches("file") {
            create_new_file(second_matches);
        }
    }
}

fn create_new_package(second_matches: &&ArgMatches) {
    if let Some(path) = second_matches.value_of("path") {
        match path.rfind(".") {
            None => {
                let dir_path = Path::new(path);
                if !dir_path.exists() {
                    let _ = fs::create_dir(dir_path);
                    let init_path = dir_path.clone().join("__init__.py");
                    let _ = File::create(init_path);
                    println!("{}/__init__.py was created", path);
                } else {
                    println!("Can't override existing module");
                }
            }
            Some(i) => {
                let name = String::from(&path[i + 1..]);
                let transformed = path.clone().replace(".", "/");
                let dir_path = Path::new(transformed.as_str());
                if !dir_path.exists() {
                    let _ = fs::create_dir_all(dir_path);
                    let init_path = dir_path.clone().join("__init__.py");
                    let mut init = File::create(init_path).unwrap();
                    let _ = init.write(format!("from flask import Blueprint\n{} = Blueprint('{}', __name__)\nfrom {} import routes", &name, &name, &path).as_ref());
                    println!("{}/__init__.py was created", transformed);

                    let routes_path = dir_path.clone().join("routes.py");
                    let mut routes = File::create(routes_path).unwrap();
                    let _ = routes.write(format!("from {} import {}", &path, &name).as_ref());
                    println!("{}/routes.py was created", transformed);
                } else {
                    println!("Can't override existing module");
                }
            }
        }
    }
}

fn create_new_file(second_matches: &&ArgMatches) {
    if let Some(name) = second_matches.value_of("name") {
        if let Some(path) = second_matches.value_of("path") {
            let path_string = path.clone().replace(".", "/");
            let dir_path = Path::new(&path_string);
            if !dir_path.exists() {
                let _ = fs::create_dir_all(dir_path);
            }
            let file_path = dir_path.clone().join(format!("{}.py", name));
            if !file_path.exists() {
                let _ = File::create(file_path);
                println!("{}/{}.py was created", path, name);
            }
        } else {
            let file_path_str = format!("./{}.py", name);
            let file_path = Path::new(&file_path_str);
            dbg!(file_path);
            if !file_path.exists() {
                let _ = File::create(file_path);
                println!("{}.py was created", name);
            }
        }
    }
}
