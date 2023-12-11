# Napali

[![ci](https://github.com/integrated-reasoning/napali/actions/workflows/ci.yml/badge.svg)](https://github.com/integrated-reasoning/napali/actions/workflows/ci.yml)
![docs.rs](https://img.shields.io/docsrs/napali)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![codecov](https://codecov.io/github/integrated-reasoning/napali/graph/badge.svg?token=9T5TT0XE5X)](https://codecov.io/github/integrated-reasoning/napali)
[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/integrated-reasoning/napali/badge)](https://flakehub.com/flake/integrated-reasoning/napali)

## About

Napali is a TUI interface to Integrated Reasoning's hardware-accelerated solver service. The TUI provides an environment for interacting with combinatorial optimization solvers and managing local and remote optimization workloads. Furthermore, Napali aims to facilitate modern development practices in operations research workflows.

This developer preview release is intended for early adopters and has no solver related features enabled at this time.

## Usage as a flake

Add napali to your `flake.nix`:

```nix
{
  inputs.napali.url = "https://flakehub.com/f/integrated-reasoning/napali/*.tar.gz";

  outputs = { self, napali }: {
    # Use in your outputs
  };
}

```
