use clap::{arg, Command};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
struct DesktopEntry {
    name: String,
    exec: PathBuf,
    icon: Option<String>,
    entry_type: String,
    // categories: Vec<String>,
}

impl DesktopEntry {
    fn from(name: &str, icon: Option<String>, exec: PathBuf/*categories: Vec<&str>*/) -> Self {
        DesktopEntry {
            name: name.to_string(),
            icon: icon,    //.map(|s| s.to_string())
            exec,
            entry_type: String::from("Application"),
            // categories: categories.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    fn to_string(&self) -> String {
        let mut entry = format!("[Desktop Entry]\nName={}\nExec={}\nType={}\n", self.name, self.exec.to_string_lossy(), self.entry_type);

        if let Some(ref icon) = self.icon {
            entry.push_str(&format!("Icon={}\n", icon));
        }
        // if !self.categories.is_empty() {
        //     entry.push_str(&format!("Categories={};\n", self.categories.join(";")));
        // }
        entry
    }

    fn run(&self) {
        let desktop_file = self.to_string();
        let user = std::env::var("SUDO_USER").expect("Failed to get the current user");
        println!("User: {:?}", user);
        let path = PathBuf::from(format!("/home/{}/.local/share/applications/", user));
        let file_name = format!("{}.desktop", self.name);
        let file_path = path.join(file_name);

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directories");
        }

        std::fs::write(&file_path, desktop_file).expect("Unable to write file");
        println!("Desktop entry created at: {:?}", file_path);
    }
}

fn cli() -> Command {
    Command::new("desk")
        .about("A command line tool to create .desktop files")
        .arg(arg!(path: [PATH])
            .help("Path to the .desktop file")
            .required(true)
            .index(1)
            .value_parser(clap::value_parser!(PathBuf))) // Ensure path is parsed as PathBuf
        .arg(arg!(-n --name <NAME>)
            .help("Name of the application")
            .required(false))
        .arg(arg!(-i --icon <ICON>)
            .help("Path to the icon file")
            .required(false))
}

fn main() {
    let matches = cli().get_matches();

    match matches.get_one::<PathBuf>("path") {
        Some(path) => {
            let mut name = String::new();
            if let Some(name_) = matches.get_one::<String>("name"){
                name = name_.to_string();
            } else {
                if let Some(file_name) = path.file_name(){
                    let file_name = file_name.to_string_lossy();
                    let file_stem = file_name.split('.').next().unwrap_or(file_name.as_ref());

                    name = file_stem.to_string();
                } else {
                    eprintln!("Could not extract name from path");
                    return;
                }
            }
            
            let mut icon = None; // still thinking about this
            if let Some(icon_) = matches.get_one::<String>("icon"){
                icon = Some(icon_.to_string());
                // println!("Icon: {}", icon);
            }

            // let categories: Vec<&str> = vec!["Utility", "Development"];

            let entry = DesktopEntry::from(&name, icon,path.clone()).run();


            println!("Creating .desktop file at {:?}", path);
            println!("Entry: {:?}", entry);
        }
        None => {
            eprintln!("No path provided");
        }
    }
}
