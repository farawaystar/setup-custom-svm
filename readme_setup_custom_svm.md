Note: Follow [this link for instructions](https://github.com/farawaystar/agave-monitor/) to setup Github Actions

# Solana SVM Customization Toolkit


A toolkit for creating customized Solana SVM environments through specfic package cloning and dependency management

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Workflow](#workflow)
- [Repository Structure](#repository-structure)
- [Troubleshooting](#troubleshooting)
- [Acknowledgments](#contributing)
- [List of Packages that can be isolated](#List-of-Packages-that-can-be-isolated)


## Features
- Lightweight cloning of Solana agave config files
- Dependency Extraction
- Single command build of package specific/partial SVM 

## Installation
```bash
git clone https://github.com/farawaystar/agave-monitor.git
cd agave-monitor
chmod +x setup-custom-svm.sh
```

## Usage 
### Option 1 - single run, recommended

(Recommended) For one-click (single run) instruction just run:
```bash
./setup-custom-svm.sh <PACKAGE_NAME>
# example: ./setup-custom-svm.sh solana-core
```
be sure to checkout the [list of available packages]() that can be compiled independently.

### Option 2 - run instructions manually
Do this only in case of errors. Open the file ```setup-custom-svm.sh``` and run each instructions manually. Worst case, you will know where exactly the error occurred.

### Workflow
```
╔═══════════════════════════════╗
║ 1.   Clone Cargo.toml files   ║
║    from agave in agave-clone/ ║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║ 2.    Build this repo         ║
║       (agave-monitor/)        ║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║ 3.    Extract Dependencies    ║
║         .json from 1          ║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║ 4.     Generate Sparse        ║
║        Checkout Command       ║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║ 5.  generate new Cargo.toml   ║
║                               ║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║ 6.   Clone agave repo in      ║
║   agave-sparse/ (with no blob)║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║ 7.  Apply sparse checkout     ║
║    on chosen Paths from 4     ║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║ 8.  Replace Cargo.toml with   ║
║         new one from 5        ║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║ 9.  Build agave-sparse with   ║
║      given Package as input   ║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║10.         Cleanup            ║
║       Temporary Files         ║
╚═══════════════╤═══════════════╝
                │
                ▽
╔═══════════════════════════════╗
║             ENJOY             ║
╚═══════════════════════════════╝
```

## Repository Structure
```
├── agave-monitor       # SVM customiization kit. run this from here
├── agave-clone         # temporary light clone repo of Cargo.toml files
└── agave-sparse        # final output repo with built package 
```

## Troubleshooting
use Option 2 if you run into errors.

## Acknowledgments
[Turbin3](https://turbin3.com) SVM cohort: @janinedotgm , @onaboat , sam, merdus

---

## List-of-Packages-that-can-be-isolated

```
- account-decoder
- solana-core
- solana-msg
- solana-svm
- solana-account
- solana-account-info
- solana-address-lookup-table-interface
- solana-atomic-u64
- solana-big-mod-exp
- solana-bincode
- solana-blake3-hasher
- solana-bn254
- solana-borsh
- solana-client-traits
- solana-clock
- solana-cluster-type
- solana-commitment-config
- solana-compute-budget
- solana-compute-budget-interface
- solana-config-program
- solana-cpi
- solana-decode-error
- solana-define-syscall
- solana-derivation-path
- solana-ed25519-program
- solana-epoch-rewards
- solana-epoch-schedule
- solana-feature-set
- solana-fee-calculator
- solana-fee-structure
- solana-hash
- solana-instruction
- solana-instructions-sysvar
- solana-last-restart-slot
- solana-measure

...testing ongoing for others
```

---