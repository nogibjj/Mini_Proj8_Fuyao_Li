use my_project::benchmark_file_io; 

fn main() {
    if let Err(e) = benchmark_file_io() {
        eprintln!("Error: {}", e);
    }
}
