use clap::clap_app;
use is_executable::IsExecutable;
use regex::Regex;
use walkdir::WalkDir;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Jérome E. <jeertmans@icloud.com>")
        (about: "Locates binary files")
        (@arg DIR: +takes_value default_value["."] -d --dir "Sets search top-directory")
        (@arg NAME: +required "Sets the input file to use")
    )
    .get_matches();

    let name = matches.value_of("NAME").unwrap();
    let directory = matches.value_of("DIR").unwrap();
    let re = Regex::new(name).unwrap();

    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if re.is_match(&f_name) {
            let path = entry.path();
            if path.is_executable() {
                println!("{}", path.to_string_lossy());
            }
        }
    }
}
