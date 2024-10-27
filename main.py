import time

def benchmark_file_io():
    """Benchmark the file writing operation and return the duration."""
    start_time = time.time()
    with open("data_sample.txt", "w") as f:
        for i in range(500000):
            f.write(f"Sample data line {i}\n")
    end_time = time.time()
    duration = end_time - start_time
    print(f"Python File Write Time: {duration:.2f} seconds")
    return duration


if __name__ == "__main__":
    benchmark_file_io()