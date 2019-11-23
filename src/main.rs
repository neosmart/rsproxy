extern crate getopts;

use getopts::Options;
use std::env;
use std::path::PathBuf;

fn print_usage(program: &str, opts: Options) {
    let program_path = std::path::PathBuf::from(program);
    let program_name = program_path.file_stem().unwrap().to_str().unwrap();
    let brief = format!("Usage: {} [-b BIND_ADDR] -l LOCAL_PORT -h REMOTE_ADDR -r REMOTE_PORT",
                        program_name);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt("l",
                "local-port",
                "The local port to which udpproxy should bind to",
                "LOCAL_PORT");
    opts.reqopt("r",
                "remote-port",
                "The remote port to which UDP packets should be forwarded",
                "REMOTE_PORT");
    opts.reqopt("h",
                "host",
                "The remote address to which packets will be forwarded",
                "REMOTE_ADDR");
    opts.optopt("b",
                "bind",
                "The address on which to listen for incoming requests",
                "BIND_ADDR");
    opts.optflag("d", "debug", "Enable debug mode");
    opts.optflag("t", "tcp", "Run in TCP proxy mode");
    opts.optflag("u", "udp", "Run in UDP proxy mode");

    let matches = opts.parse(&args[1..])
        .unwrap_or_else(|_| {
            print_usage(&program, opts);
            std::process::exit(-1);
        });
    if matches.opt_present("t") && matches.opt_present("u") {
        println!("Cannot use both --t/--tcp and -u/--udp at the same time!");
        std::process::exit(-1);
    }

    let protocol = match matches.opt_present("u") {
        true => Protocol::Udp,
        false => Protocol::Tcp, //because it's the default
    };

    let server = match protocol {
        Protocol::Tcp => "tcpproxy",
        Protocol::Udp => "udpproxy",
    };

    if is_executable(server).is_none() {
        cargo_install(server).expect(&format!("Failed to install {} package via cargo!", server));
    }

    let mut child = std::process::Command::new(server)
        .arg("-l")
        .arg(matches.opt_str("l").unwrap())
        .arg("-r")
        .arg(matches.opt_str("r").unwrap())
        .arg("-h")
        .arg(matches.opt_str("h").unwrap())
        .arg(match matches.opt_present("b") {
            true => "-b",
            false => "",
        })
        .arg(match matches.opt_present("b") {
            true => matches.opt_str("b").unwrap(),
            false => "".to_owned(),
        })
        .arg(match matches.opt_present("d") {
            true => "-d",
            false => "",
        })
        .spawn()
        .expect(&format!("{} failed to run!", server));
    let exit_status = child.wait().unwrap();

    std::process::exit(exit_status.code().unwrap_or(-1));
}

enum Protocol {
    Tcp,
    Udp,
}

fn cargo_install(package: &str) -> std::io::Result<()> {
    if is_executable("cargo").is_none() {
        panic!("Could not find cargo package manager in $PATH!");
    }

    let mut child = std::process::Command::new("cargo")
        .arg("install")
        .arg(package)
        .spawn()
        .expect(&format!("Failed to install {} via cargo package manager!", package));
    child.wait().unwrap();

    return Ok(());
}

//checks if an exe is executable, and if so, returns the full path
fn is_executable(name: &str) -> Option<PathBuf> {
    let env_path = env::var("PATH").expect("Could not query $PATHS");
    let mut paths: Vec<PathBuf> =
        env_path.split(|c| [';', ':'].iter().find(|x| **x == c).is_some())
            .map(|p| PathBuf::from(p))
            .collect();
    paths.push(env::current_dir().unwrap());

    let mut search_name = name.to_owned();
    if cfg!(windows) {
        search_name.push_str(".exe");
    };

    for path in paths {
        let full_path = path.join(&search_name);
        if full_path.exists() {
            return Some(full_path);
        }
    }

    return None;
}
