use binwalk::Binwalk;
use clap::Parser;
use std::{fs, path::Path, sync::LazyLock};

static SUPPORTED_EXTENSION: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "png",
         "jpg",
         "jpeg",
         "gif",
         "svg",
         "pdf"
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
});

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn is_image_injected(path: &str) -> bool {
    let file_path = Path::new(&path).display().to_string();

    let binwalker = Binwalk::configure(
        Some(file_path),
        None,
        Some(SUPPORTED_EXTENSION.to_vec()),
        None,
        None,
        false,
    )
    .unwrap();

    let file_data = std::fs::read(&binwalker.base_target_file).expect("Failed to read from file");
    let scan_results = binwalker.scan(&file_data);

    let total_size = fs::metadata(&binwalker.base_target_file).unwrap().len() as usize;
    let actual_size = scan_results
        .clone()
        .into_iter()
        .map(|x| x.size)
        .reduce(|acc, e| acc + e)
        .unwrap_or_default();

    if total_size == actual_size {
        return false;
    }

    let info = scan_results
        .iter()
        .map(|x| x.description.to_string())
        .reduce(|acc, e| acc + &e)
        .unwrap_or_default();
    println!("{}", info);
    println!("Unknown data size: {} bytes", total_size - actual_size);

    true
}

fn main() {
    let args = Args::parse();
    let path = args.path;

    let injected = match Path::new(&path).extension() {
        Some(ext) => {
            let extension = ext.to_str().unwrap();

            if SUPPORTED_EXTENSION.iter().any(|x| x.eq(extension)) {
                is_image_injected(&path)
            } else {
                println!("unsupported extension: {}", extension);
                false
            }
        }
        None => false,
    };

    println!();
    println!("file injected: {}", injected);
}
