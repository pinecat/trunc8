# trunc8

This is a tiny library for [Rust](https://www.rust-lang.org/), used to truncate text lines to a specific length. This library is still a work in progress, so please check back for updates.

## Installation

Inside your rust project:

```
cargo add trunc8
```

## Usage

```rust
fn main() {
    let lines = trunc8::truncate("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ultricies mi eget mauris pharetra et ultrices. Hendrerit dolor magna eget est lorem. Ullamcorper eget nulla facilisi etiam dignissim diam quis enim. Cursus vitae congue mauris rhoncus aenean vel elit scelerisque. Diam vulputate ut pharetra sit amet aliquam id diam maecenas. Molestie ac feugiat sed lectus. Ac turpis egestas integer eget. Blandit libero volutpat sed cras ornare arcu dui vivamus. Volutpat diam ut venenatis tellus. Id porta nibh venenatis cras sed felis eget velit.", 80);

    lines.iter().for_each(|line| {
        ...
    });
}
```

## Development

After checking out the repo, make any additions or fixes in the `src` folder. Make sure to add appropriate tests in `lib.rs`, and ensure your changes work before submitting a pull request, by running `cargo test`.

## Contributing

Bug reports and pull requests are welcome on GitHub @ https://github.com/pinecat/trunc8.

## License

This library is available as open source under the term of the [BSD 3 Clause License](https://opensource.org/license/bsd-3-clause/).
