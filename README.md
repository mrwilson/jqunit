# jqunit

![Build and Test](https://github.com/mrwilson/jqunit/actions/workflows/build.yaml/badge.svg)
![crates.io](https://img.shields.io/crates/v/jqunit.svg)

A test framework for JQ, written in Rust, on top of libjq.

## Installing

You can install `jqunit` directly from crates.io

```
cargo install jqunit@<version>
```

Or you can build it from this source

```
git clone git@github.com:mrwilson/jqunit-rs.git
cargo install --path jqunit-rs
```

## What is a test?

`jqunit` views a test as a function starting with `should_` in a file ending with `_test.jq`.
The test function fails if it returns an error, and passes if it returns anything else.

```jq
def should_pass_test: 1; 

def should_fail_test: error("Fails test");
```

You can implement your own `assert` until there is one in the `jq` standard build.

```jq
def assert($description; $assertion):
    if $assertion then . else error("\($description)") end;
```

## Example

Running against Day 11 of my [advent-of-code-2021](https://github.com/mrwilson/advent-of-code-2021) repository.

```
$ jqunit --libraries ../advent-of-code-2021 ../advent-of-code-2021/* --module day_11_test

test day_11_test::should_parse_input ... ok (5ms)
test day_11_test::should_test_if_any_octopodes_ready_to_flash ... ok (5ms)
test day_11_test::should_get_neighbouring_octopodes ... ok (5ms)
test day_11_test::should_tick ... ok (7ms)
test day_11_test::should_tick_test_input ... ok (5ms)
test day_11_test::should_count_flashes_after_1_tick ... ok (5ms)
test day_11_test::should_count_flashes_after_2_ticks ... ok (12ms)
test day_11_test::should_count_flashes_after_10_ticks ... ok (38ms)
test day_11_test::should_count_flashes_after_100_ticks ... ok (301ms)
test day_11_test::should_find_first_simultaneous_flash ... ok (537ms)
```


## Usage

- `--libraries`: Space separated list of directories to import when running tests. Analogous to jq's `-L`
- `--module`: Test module to execute. If not present, `jqunit` will traverse the directory tree and search for files with names matching `*_test`