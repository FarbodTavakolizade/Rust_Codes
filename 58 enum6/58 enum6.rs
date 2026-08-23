enum FileSize{
    Bytes(u64),
}

impl FileSize {
    fn format_size(&self) ->String{
        match self {
            FileSize::Bytes(bytes) =>{
                if *bytes <1_000{
                    format!("{} bytes",bytes)
                }else if *bytes<1_000_000 {
                    format!("{:.6} kilobytes", *bytes as f64 /1_000.0)
                }else if *bytes <1_000_000_000 {
                    format!("{:.6} megabytes" ,*bytes as f64 /1_000_000.0)
                }else {
                    format!("{:.6} gigabytes",*bytes as f64 /1_000_000_000.0)
                }
            }
            
        }
    }
}

fn main() {
    let size:u64 =34_887_387_837;
    let filesize =FileSize::Bytes(size);
    println!( "filesize : {}",filesize.format_size());
}
