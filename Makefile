install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	python -m pytest -vv --cov=main test_*.py

format:	
	black *.py 

lint:
	#disable comment to test speed
	#pylint --disable=R,C --ignore-patterns=test_.*?py *.py mylib/*.py
	#ruff linting is 10-100X faster than pylint
	ruff check *.py

container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor: format lint

deploy:
	#deploy goes here
		
all: install lint test format deploy


rust_build:
	cd my_project && cargo build

rust_run:
	cd my_project && cargo run

rust_test:
	cd my_project && cargo test

rust_clean:
	cd my_project && cargo clean

rust_format:
	cd my_project && cargo fmt

rust_lint:
	cd my_project && cargo clippy -- -D warnings

all: rust_build rust_run rust_test rust_clean rust_format rust_lint
