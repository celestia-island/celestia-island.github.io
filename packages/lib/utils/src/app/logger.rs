use chrono::Local;
use once_cell::sync::Lazy;
use std::{fs::File, io::Write, path::PathBuf};

pub static LOG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let path = {
        #[cfg(debug_assertions)]
        {
            let mut ret = std::env::current_dir().unwrap();
            ret.push("target");
            ret.push("log");
            ret
        }
        #[cfg(not(debug_assertions))]
        {
            let mut ret = std::env::current_dir().unwrap();
            ret.push("log");
            ret
        }
    };

    // If directory does not exist, create it
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path
});

pub fn create_logger(name: String) {
    let target = Box::new(
        File::create({
            let mut path = LOG_DIR.clone();
            path.push(format!(
                "{}-{}.log",
                name,
                chrono::Utc::now().format("%Y-%m-%d-%H-%M-%S")
            ));
            path
        })
        .expect("Can't create file"),
    );
    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, log::LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}
