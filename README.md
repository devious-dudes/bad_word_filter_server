# Bad Word Filter Server

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Server](https://github.com/devious-dudes/bad_word_filter_server/actions/workflows/rust.yml/badge.svg)](https://github.com/devious-dudes/bad_word_filter_server/actions/workflows/rust.yml)

A Rust-based server to filter bad words and phrases using a Trie data structure. It supports single and multi-word phrases and can run as a daemon.  The question is "why" build this?
If you have a heavily trafficked site and people are posting constantly, the web server has enough to do besides loading up a bunch of words and phrases and attempting to search for them.
A microservice is an obvious answer.  Any high performance language would do, but why not something capable of handling a LOT of requests?  So I kept this simple and my results are as
follows, around 12,000 requests a second, or each request is around/between 84-800 microseconds.

``` bash
Concurrency Level:      10
Time taken for tests:   0.838 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      1220000 bytes
Total body sent:        2620000
HTML transferred:       60000 bytes
Requests per second:    11933.60 [#/sec] (mean)
Time per request:       0.838 [ms] (mean)
Time per request:       0.084 [ms] (mean, across all concurrent requests)
Transfer rate:          1421.78 [Kbytes/sec] received
                        3053.32 kb/s sent
                        4475.10 kb/s total
```

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Architecture](#architecture)
- [Endpoints](#endpoints)

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB](https://www.mongodb.com/try/download/community)
- [Actix Web](https://actix.rs/)
- [dotenv](https://github.com/dotenv-rs/dotenv)
- [clap](https://crates.io/crates/clap)
- [daemonize](https://crates.io/crates/daemonize)

### Steps

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/bad_word_svr.git
    cd bad_word_svr
    ```

2. Install dependencies:
    ```bash
    cargo build
    ```

3. Create a `.env` file in the root directory with your MongoDB URI and database name:
    ```env
    MONGO_URI=mongodb://username:password@localhost:27017/your_database_name?authSource=admin
    MONGO_DBNAME=your_database_name
    BEARER_TOKEN=optional_auth_token
    ```
### Database
Using MongoDB simply create a collection called `badwords` and within that collection, each document
should have a `word` field, for example:
```json
[{"word":"badword"},{"word":"bad phrase"}]
```
The documents can have any other metadata that's required, only the `word` field is required.

## Usage

### Running the Server

#### Default Mode
Run the server with default settings (localhost:8080):
```bash
cargo run
```

#### Custom Host and Port
Specify a custom host and port:

```bash
cargo run -- --host 0.0.0.0 --port 8080
```

#### Daemon Mode
Run the server as a daemon:
``` bash
cargo run -- --daemon
```

### Command-Line Arguments
- `--host <HOST>`: Host to bind to (default: `localhost`)
- `--port <PORT>`: Port to bind to (default: `8080`)
- `--daemon or -d`: Run as a daemon

### Endpoints
#### Check Content
- Endpoint: /check
- Method: POST
- Description: Checks if the content contains any bad words or phrases.
- Request Body
``` json
{
  "content": "This is a test message."
}
```
- Response
  - `"ok"` if no bad words are found.
  - `"not ok"` if bad words are found.

#### Reload Trie
- Endpoint: `/reload`
- Method: POST
- Description: Reloads the Trie with updated bad words from the MongoDB collection.
- Response:
  - `"Trie reloaded"` upon successful reload

#### Health
- Endpoint: `/health`
- Method: GET
- Description: For a readiness or liveness probe or just to check word count and memory usage.
- Response:
  - `{"status":"ok","word_count":31,"memory_used_kb":5537864,"total_memory_kb":63199236}`

## Example CURL calls
1. Reload words from database with BEARER_TOKEN set within env (export BEARER_TOKEN=xxx)
``` bash
curl -H "Authorization: Bearer $BEARER_TOKEN" -X POST http://localhost:8080/reload -H "Content-Type: application/json"
```
2. Check health of application
``` bash
curl -H "Authorization: Bearer $BEARER_TOKEN" http://localhost:8080/health -H "Content-Type: application/json"
```

3. Check message for bad words or phrases
``` bash
curl -H "Authorization: Bearer $BEARER_TOKEN" -X POST -d '{"content":"This is a message to test for bad words."}' http://localhost:8080/reload -H "Content-Type: application/json"
```
Add one of your bad words to the content and test it.

## Building and Running the Docker Image
1. Build the Docker Image
``` bash
docker build -t bad_word_svr .
```
2. Run the Docker container:
```
docker run -d --name bad_word_svr -p 8080:8080 --env-file ./.env bad_word_svr
```
The Dockerfile creates a minimal image using Alpine Linux, ensuring the application runs efficiently in a containerized environment.
The docker container will not run without specifying the MONGO_URI and MONGO_DBNAME env vars, so the command above assumes you have
copied dot.env to .env and replaced the env variables with your own.

## Architecture

### Overview

The server is built using the Actix Web framework and leverages MongoDB for storing bad words and phrases. It uses a Trie data structure to efficiently check for the presence of bad words and phrases in the input text.

### Components

- **Trie**: A data structure used to store and search for bad words and phrases.
- **Actix Web**: A powerful, pragmatic, and extremely fast web framework for Rust.
- **MongoDB**: A NoSQL database used to store bad words and phrases.
- **dotenv**: Loads environment variables from a `.env` file.
- **clap**: A command-line argument parser for Rust.
- **daemonize**: A library to daemonize a Rust application on Unix-like systems.

### Directory Structure

```plaintext
bad_word_svr/
├── src/
│   ├── main.rs            # Entry point of the application
│   ├── db.rs              # Database-related functions
│   ├── trie.rs            # Trie data structure implementation
│   ├── nlp_processing.rs  # Text processing functions
├── target/                # Compilation output
├── .env                   # Environment variables file
├── Cargo.toml             # Cargo configuration file
├── Makefile               # Makefile for building and running the application
└── README.md              # This README file
```

### How It Works

1. **Text Processing**: Input text is processed to generate unigrams, bigrams, and trigrams.
2. **Trie Search**: The processed text is checked against the Trie to detect bad words or phrases.
3. **Database Interaction**: Bad words and phrases are stored in MongoDB and loaded into the Trie at startup or upon reload.

### Flow Diagram

```plaintext
+------------------+          +---------------------+
|                  |          |                     |
|  Client Request  +--------->+  Actix Web Server   |
|                  |          |                     |
+------------------+          +----------+----------+
                                      |
                                      v
                         +------------+-------------+
                         |                          |
                         |   Text Processing (NLP)  |
                         |                          |
                         +------------+-------------+
                                      |
                                      v
                         +------------+-------------+
                         |                          |
                         |   Trie Data Structure    |
                         |                          |
                         +------------+-------------+
                                      |
                                      v
                         +------------+-------------+
                         |                          |
                         |   MongoDB Interaction    |
                         |                          |
                         +--------------------------+
```
### License
This project is licensed under the MIT License.  See the [LICENSE](LICENSE) file for details.

### Contributing
Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

### Contact
For questions or support, pleae contact: [Derrick Woolworth](https://github.com/dwoolworth)

