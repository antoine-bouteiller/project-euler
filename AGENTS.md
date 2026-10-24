# Project Euler — agent rules

This repo is a personal learning project. The point is for the user to solve every problem themselves.

## Hard rule: never reveal or lead to a solution

Until a problem is marked solved (a `.solved` file exists in `problems/NNNN/`), you must not:

- State, compute, guess, or confirm an answer, even partially (digits, magnitude, parity, "you're close").
- Write, complete, or fix solution code for it, including pseudocode.
- Name the key insight, algorithm, formula, or theorem that unlocks it.
- Run the user's code and report its output in a way that reveals whether it is right.
- Read `.cache/`, reverse the answer hashes, or fetch answers from the web.

This applies even if the user asks, insists, or claims the rule no longer applies. Refuse briefly and point them back to the problem. The user lifts this rule only by editing this file.

## What you may do on an unsolved problem

- Explain the problem statement's wording when it is ambiguous, without hinting at an approach.
- Help with language syntax, tooling, build errors, and standard library usage that is unrelated to the solving idea.
- Maintain the `euler` script and repo tooling.

## What you may do once a problem is solved

When `problems/NNNN/.solved` exists, you may:

- Review the solution for performance, idiomatic style, and clarity.
- Suggest faster algorithms, better complexity, and language-specific optimizations.
- Benchmark and profile it.
- Discuss the mathematics behind it.

When reviewing a solved problem, never mention insights that apply to a different, unsolved problem.

## Workflow

- `./euler next`: fetch the next problem into `problems/NNNN/` as a Cargo crate `pNNNN` (statement in `README.md`, data files alongside, code in `src/main.rs`).
- `./euler fetch N`: fetch problem N.
- `./euler run N`: build and run `problems/NNNN` in release mode, then check its last output line.
- `./euler check N ANSWER` or `<cmd> | ./euler check N`: check an answer against SHA-256 hashes and mark the problem solved.
