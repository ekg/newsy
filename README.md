Here is an updated README for the Rust news article downloader application called newsy:

# newsy

This is a simple command line application written in Rust that downloads news articles from NewsAPI.org and saves them as text files.

## Usage

The application requires a NewsAPI.org API key, which can be obtained for free from their website.

```
newsy --api-key <your-api-key> 
```

This will download the top 100 US headlines and save them in the current directory. 

To search for articles on a topic:

```
newsy --api-key <your-api-key> --search "rust programming"
```

You can also set the number of articles to retrieve with `--limit`, and output directory with `--output-dir`.

Run `newsy --help` to see all options.

Articles are saved as text files named `<article title>.txt` containing the article content.

## Building

This project requires Rust to be installed. To build:

```
cargo build --release
```

The executable will be in `target/release/`

## About 

- Author: Erik Garrison (erik.garrison@gmail.com)
- License: MIT

## Contributing

Contributions are welcome! Please open an issue or pull request on GitHub. 

## License

This project is MIT licensed. See `LICENSE` for details.
