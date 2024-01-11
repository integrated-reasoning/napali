# Napali

[![ci](https://github.com/integrated-reasoning/napali/actions/workflows/ci.yml/badge.svg)](https://github.com/integrated-reasoning/napali/actions/workflows/ci.yml)
![docs.rs](https://img.shields.io/docsrs/napali)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![codecov](https://codecov.io/github/integrated-reasoning/napali/graph/badge.svg?token=9T5TT0XE5X)](https://codecov.io/github/integrated-reasoning/napali)
![Docker Image Size (tag)](https://img.shields.io/docker/image-size/integratedreasoning/napali/latest)
[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/integrated-reasoning/napali/badge)](https://flakehub.com/flake/integrated-reasoning/napali)

## About

Napali is a TUI interface to Integrated Reasoning's hardware-accelerated solver service. The TUI provides an environment for interacting with combinatorial optimization solvers and managing local and remote optimization workloads. Furthermore, Napali aims to facilitate modern development practices in operations research workflows.

This developer preview release is intended for early adopters and has no solver related features enabled at this time.

<img alt="Napali VHS demo" src="vhs.gif" width="600" />

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

## Running with Docker

```bash
docker run -it integratedreasoning/napali:latest
```

## Roadmap

- Core Capabilities
  - Manage optimization problems in session containers
  - Configure and select from a variety of MILP/LP solvers
  - Specify hardware resources and tuning parameters per solver
  - Submit local multi-threaded and remote cloud jobs
  - Create solver ensembles with config permutations
  - Monitor runs: status, resources, metrics, logging
  - Interrupt, checkpoint and resume sessions
  - Branch session histories and compare run outcomes
- Workflow Features
  - One-click setup workflows for common scenarios
  - Quickstart templates for one-off testing
  - Template library of parameterized configurations
  - Automated notification triggers
  - Analysis charts: performance, stability etc.
  - Integrated visualization toolkit plugins
- Infrastructure Integrations
  - Git version control backend + diffing engine
  - CI/CD pipeline plugins (GitHub Actions, Airflow etc.)
  - Account and billing management
- Interface Capabilities
  - Multi-document tabbed interface
  - CLI access to features beyond core TUI
  - Contextual inline help and documentation
  - Module subsystem for community extensions
  - Scriptable actions

### Sketches

```
╦═══════════════╦═════════════════════════════════════════════════════════════════════════════╦══════╦══════╗
║ Napali        ║ miplib                                                                      ║ Warn ║ Help ║
╠═══════════════╬═════════════════════════════════════════════════════════════════════════════╬══════╬══════╣
║ Sessions >    ║ Problems:                                                                   ║ [0]  ║ F1   ║
║               ║ ▶ assign1-5-8 (MIP) - ~/data/miplib/assign1-5-8                             ║      ║      ║
║ New Session   ║                                                                             ║      ║      ║
║               ║ > Validate              Solve Options         Submit          Results       ║      ║      ║
║               ╟─────────────────────────────────────────────────────────────────────────────╢      ║      ║
║               ║ Configure Ensemble Run                                                      ║      ║      ║
║               ║                                                                             ║      ║      ║
║               ║ Name: ensemble-1                                                            ║      ║      ║
║               ║ Solvers:                                                                    ║      ║      ║
║               ║  [+] honu                                                                   ║      ║      ║
║               ║  [+] highs                                                                  ║      ║      ║
║               ║  [+] cbc                                                                    ║      ║      ║
║               ║  [+] ortools                                                                ║      ║      ║
║               ║                                                                             ║      ║      ║
║               ║ > Run ensemble                                                              ║      ║      ║
║               ╚═════════════════════════════════════════════════════════════════════════════╩══════╩══════╝
╚═══════════════╩═════════════════════════════════════════════════════════════════════════════╧══════╧══════╝

╦══════════════╦═══════════════════════════════════════════════════════════════════════════╦═══ ══╦══════╗
║ Napali       ║ miplib                                                                    ║ Warn ║ Help ║
╠══════════════╬═══════════════════════════════════════════════════════════════════════════╬══════╬══════╣
║ Sessions >   ║ Problems:                                                                 ║ [0]  ║ F1   ║
║ New Session  ║ ▶ assign1-5-8 (MIP) - ~/data/miplib/assign1-5-8                           ║      ║      ║
║              ╟───────────────────────────────────────────────────────────────────────────╢      ║      ║
║              ║ Analysis Dashboard                                                        ║      ║      ║
║              ║                                                                           ║      ║      ║
║              ║ Found Best: ███████░░░                                                    ║      ║      ║
║              ║ Path Progress: █░░░░░░░░░░░░█████░░░░░░░░░░░░███░█░░░░░░░░░░░             ║      ║      ║
║              ║ Opt Gap: █░░░░░░░░░░░░░░░░░░███░░░░░░░░░░░░░████░░░░░░░░░░░░░             ║      ║      ║
║              ║ Nodes Left: █░░░░░░░░░░░░░░░░░░░░░█████░░░░░░░░░░░░░░░░░░░░░░             ║      ║      ║
║              ╚═══════════════════════════════════════════════════════════════════════════╩══════╩══════╝
╚══════════════╩═══════════════════════════════════════════════════════════════════════════╧══════╧══════╝

╦═══════════╦═══════════════════════════════════════════════════════════════════════════════════════╦══════╦══════╗
║ Napali    ║ miplib                                                                                ║ Warn ║ Help ║
╠═══════════╬═══════════════════════════════════════════════════════════════════════════════════════╬══════╬══════╣
║ Sessions >║  model v1      ┊ 1 week ago         │Vars: 120 Const: 245 Nonzeros: 1230 Status: Opt  ║ [0]  ║  F2  ║
║ Open      ╟────┬───────────┴────────────────────┤Time: 10m Obj: $149,053 Cuts: 10 Nodes: 4,832    ║      ║      ║
║           ║ dev╎ model v2  ┊ 3 days ago         │Vars: 100 Const: 215 Nonzeros: 1050 Status: Inf  ║      ║      ║
║ Reports   ║    ╎───────────┬────────────────────┤Time: 8m Obj: NA Cuts: 5 Nodes: 342 Gap: 14%     ║      ║      ║
║           ║    ╎ model v3  ┊ 1 day ago          │Vars: 95 Const: 203 Nonzeros: 980 Status: Opt*   ║ [1]  ║      ║
║           ║    │           │                    │Time: 22m Obj: $147,261 Cuts: 23 Nodes: 8,495    ║      ║      ║
║           ║    ╰───────────┼─────> supply v3*   │Vars: 102 Const: 218 Nonzeros: 1102 Estimated    ║      ║      ║
║           ║ main ──╮       ╎ 1 hour ago         │                                                 ║      ║      ║
║           ╬════════╝       ╚════════════════════╧═════════════════════════════════════════════════╩══════╩══════║
║           ║  > Select commits to compare                                                                        ║
╚═══════════╩═════════════════════════════════════════════════════════════════════════════════════════════════════╝
```
