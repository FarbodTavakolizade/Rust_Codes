enum FileSize{
    Byte(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}
fn format_size(size :u64) ->String{
    let filesize=match size {
        0..=999 =>FileSize::Byte(size),
        1000..=999_999 =>FileSize::KiloBytes(size /1000),
        1_000_000 ..=999_999_999 =>FileSize::MegaBytes(size / 1_000_000),
        _ =>FileSize::GigaBytes(size / 1_000_000_000),
        
    };

    match filesize {
        FileSize::Byte(bytes) =>format!("{} bytes",bytes),
        FileSize::KiloBytes(kb) =>format!("{:.4} kilobytes " ,kb as f64 /1000.0),
        FileSize::MegaBytes(mb) =>format!("{:.4} megabytes " ,mb as f64 /1000.0),
        FileSize::GigaBytes(gb) =>format!("{:.4} gigabytes " ,gb as f64 /1000.0),
    }
}
fn main() {
    let result =format_size(145484514);
    println!("{}",result);

}
