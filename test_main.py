import os
from main import benchmark_file_io

def test_benchmark_file_io():
    benchmark_file_io()
    
    assert os.path.exists("data_sample.txt"), "Output file was not created"