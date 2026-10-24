# Project Euler

Personal Rust solutions to [Project Euler](https://projecteuler.net). Requires [Bun](https://bun.sh) and Rust (pinned in `mise.toml`).

```sh
./euler next            # fetch the next problem into problems/NNNN/ as crate pNNNN
./euler fetch 42        # fetch a specific problem
./euler run 1           # cargo run --release -p p0001, then check the last output line
./euler check 1 12345   # check an answer by hand
```

Each problem is a workspace member. Data files sit next to `README.md`; load them with
`include_str!("../0022_names.txt")`.

A correct answer creates `problems/NNNN/.solved`. Reference answers come from
[lucky-bai/projecteuler-solutions](https://github.com/lucky-bai/projecteuler-solutions)
and are cached only as SHA-256 hashes in `.cache/`.
