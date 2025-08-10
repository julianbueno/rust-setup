use std::fmt;

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSize::Bytes(bytes) => write!(f, "{} bytes", bytes),
            FileSize::Kilobytes(bytes) => write!(f, "{:.2} KB", *bytes as f64 / 1_000.0),
            FileSize::Megabytes(bytes) => write!(f, "{:.2} MB", *bytes as f64 / 1_000_000.0),
            FileSize::Gigabytes(bytes) => write!(f, "{:.2} GB", *bytes as f64 / 1_000_000_000.0),
            FileSize::Terabytes(bytes) => write!(f, "{:.2} TB", *bytes as f64 / 1_000_000_000_000.0),
        }
    }
}

fn format_size(size: u64) -> String {
    // The logic has been corrected to pass the original size to the enum variant
    // and perform the division during formatting for better precision.
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size),
        1_000_000..=999_999_999 => FileSize::Megabytes(size),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size),
        _ => FileSize::Terabytes(size),
    };
    filesize.to_string()
}

// Returns the file size in the largest possible binary unit (KiB, MiB, etc.).
fn format_size_binary(size: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    const TIB: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

    let size_f = size as f64;

    if size_f < KIB {
        format!("{} Bytes", size)
    } else if size_f < MIB {
        format!("{:.2} KiB", size_f / KIB)
    } else if size_f < GIB {
        format!("{:.2} MiB", size_f / MIB)
    } else if size_f < TIB {
        format!("{:.2} GiB", size_f / GIB)
    } else {
        format!("{:.2} TiB", size_f / TIB)
    }
}

fn main() {
    // Test with a value in Gigabytes (base-10)
    println!("Gigabyte test: {}", format_size(6_888_837_399));
    // Test with a new value in Terabytes (base-10)
    println!("Terabyte test: {}", format_size(2_500_000_000_000));

    // Test the new binary formatting function
    println!("\n--- Binary Unit Tests (base-2) ---");
    println!("2500 bytes is: {}", format_size_binary(2500));
    println!("1048576 bytes is: {}", format_size_binary(1_048_576));
    println!("6888837399 bytes is: {}", format_size_binary(6_888_837_399));
}
