# Advent of Code 2023

**Personally** I took up this challange to get the hang of such things as **benchmarking, testing, memory saving habits and outperformancing as much as I could** in which I would like to be proficient in the future. Also I wanted to get fluent in writing Rust code

![Personal times](personal_times.png)

This year [Christopher](https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2023/rust) has pre-set up a series of functionality for testing, benchmarking, and otherwise evaluating the performance of our Rust programs.

This includes the command `just work` which is passed a particular day and part and is the equivalent workflow of running all of these in a row and stopping if one fails, then re-starting the flow after changes.

```
cargo check
cargo nextest run
clippy-tracing --action check
cargo clippy
cargo bench
cargo flamegraph
```

## Prepare for a new day

```shell
just create <day>
```

## Just

Just is used to partially document all tasks, so you (the person reading this) can see what commands we were running and perhaps run them yourself on your own codebase.

## cargo-flamegraph

Flamegraphs are great for identifying what's taking up time in a program!

[cargo-flamegraph][cargo-flamegraph] will output an SVG that contains a flamegraph of the given program. They are available in the **flamegraph** directory

## Divan

No strong reason for using [Divan over criterion][divan:compared-to-criterion] other than to get to know the crate a bit better.

You can learn more about Divan in the [announcement post][divan-announcement].

## cargo-nextest

[cargo-nextest][cargo-nextest] is "a next-generation test runner for Rust projects". Basically that means it includes [an interesting execution model][cargo-nextest-execution-model] than can be great for projects with a _lot_ of tests.

As of this year's AoC, [cargo-nextest][cargo-nextest] doesn't run doctests yet, so while that won't be an issue for us it is something to be aware of if you're using nextest in a "real project". (Basically that means you also run `cargo test --doc`).

cargo-nextest has what I consider [a positive relationship with the regular `cargo test`](https://nexte.st/book/how-it-works.html#contributing-features-back-to-cargo) and is rightfully a nice place to be experimenting with new testing UX. `cargo test` works well and `cargo nextest` is a forward-looking place for experimentation.

## tracing

The [tracing][tracing] crate is (in my opinion) the modern standard tracing and logging infrastructure tool in the Rust ecosystem. It is maintained by the same group of people working on the tokio async runtime and is extremely widely used.

For advent of code we could get away with using `println!` and `dbg!` but [tracing][tracing] offers not only rich span and event tracking, but also integration with other tools like [tracy][tracy].

## tracy

Tracy is a profiling tool I'd like to get better at using, especially for profiling Bevy games. It is the least-known tool on this list for me.

- [tracing_tracy][tracing_tracy]
- [Tracy][tracy]

Additional steps are needed for Tracy to work.

```shell
brew install tracy
```

[cargo-flamegraph]: https://github.com/flamegraph-rs/flamegraph
[tracing_tracy]: https://docs.rs/tracing-tracy/0.10.4/tracing_tracy/index.html
[tracy]: https://github.com/wolfpld/tracy
[divan]: https://github.com/nvzqz/divan
[divan-announcement]: https://nikolaivazquez.com/blog/divan/
[divan:compared-to-criterion]: https://nikolaivazquez.com/blog/divan/#compared-to-criterion
[cargo-nextest]: https://nexte.st/
[cargo-nextest-execution-model]: https://nexte.st/book/how-it-works.html
[tracing]: https://docs.rs/tracing/0.1.40/tracing/index.html
