# lidoom

[![crates.io](https://img.shields.io/crates/v/lidoom)](https://crates.io/crates/lidoom)
[![Build](https://github.com/fwcd/lidoom/actions/workflows/build.yml/badge.svg)](https://github.com/fwcd/lidoom/actions/workflows/build.yml)

DOOM port for Project Lighthouse.

## Usage

Make sure to have the following environment variables set or in a `.env` file in the working directory:

```sh
LIGHTHOUSE_USER=<your user>
LIGHTHOUSE_TOKEN=<your token>
```

Additionally, make sure that your working directory contains a DOOM WAD (e.g. `DOOM1.WAD`), which you need to obtain externally, then run

```sh
lidoom
```

> If you're building the repository, use `cargo run` instead.
