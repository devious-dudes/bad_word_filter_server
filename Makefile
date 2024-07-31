# Variables
CARGO = cargo
TARGET_DEBUG = target/debug/bad_word_svr
TARGET_RELEASE = target/release/bad_word_svr

# Default target
all: build

# Build the project
build:
	$(CARGO) build

# Build for release
build-release:
	$(CARGO) build --release

# Run the project
run: build
	$(TARGET_DEBUG)

run-release: build-release
	$(TARGET_RELEASE)

# Clean the project
clean:
	$(CARGO) clean

# Format the code
format:
	$(CARGO) fmt

# Check the code for errors
check:
	$(CARGO) check

