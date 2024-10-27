use my_project::benchmark_file_io;

#[test]
fn test_benchmark_file_io() {
    // Run the benchmark function
    let result = benchmark_file_io();

    assert!(result.is_ok(), "Benchmark file I/O should run successfully");
}
