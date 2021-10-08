use clap::clap_app;
use is_executable::IsExecutable;
use itertools::Either;
use regex::Regex;
use jwalk::WalkDir;

fn main() {
    let matches = clap_app!(myapp =>
        (name: "locate")
        (version: "1.0")
        (author: "Jérome E. <jeertmans@icloud.com>")
        (about: "Recursively locates filenames that match regex pattern")
        (@arg DIR: +takes_value default_value["."] -d --dir "Sets search top-directory")
        (@arg MIN_DEPTH: +takes_value default_value["0"] --min_depth "Sets minimum search depth")
        (@arg MAX_DEPTH: +takes_value default_value["-1"] --max_depth "Sets maximum search depth")
        (@arg FOLLOW_LINKS: -f --follow_links "Follow links")
        (@arg EXE: -e --executable "Only returns files that are executable")
        (@arg PATTERN: +required "Sets the regex pattern filenames must match")
    )
    .get_matches();

    let pattern = matches.value_of("PATTERN").unwrap();
    let directory = matches.value_of("DIR").unwrap();
    let re = Regex::new(pattern).unwrap();
    let min_depth: usize = matches.value_of("MIN_DEPTH").unwrap().parse().expect(
        "Could not parse MIN_DEPTH into a `usize` value: please enter a valid unsigned integer.",
    );
    let max_depth_str = matches.value_of("MAX_DEPTH").expect(
        "Could not parse MAX_DEPTH into a `usize` value: please enter a valid unsigned integer.",
    );
    let max_depth: usize = if max_depth_str.eq("-1") {
        ::std::usize::MAX
    } else {
        max_depth_str.parse().unwrap()
    };
    let follow_links = matches.is_present("FOLLOW_LINKS");

    let iterator = WalkDir::new(directory)
        .min_depth(min_depth)
        .max_depth(max_depth)
        .follow_links(follow_links)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| re.is_match(&e.file_name().to_string_lossy()));

    let iterator = if matches.is_present("EXE") {
        Either::Left(iterator.filter(|e| e.path().is_executable()))
    } else {
        Either::Right(iterator)
    };

    for entry in iterator {
        println!("{}", entry.path().to_string_lossy());
    }
}
