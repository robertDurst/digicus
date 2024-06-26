# Technical Architecture

**Table of Contents**
- [Technical Architecture](#technical-architecture)
  - [Digicus Programming Language](#digicus-programming-language)
    - [Overall Architecture](#overall-architecture)
    - [Digit](#digit)
      - [Digicus Textual Representation Format Specification](#digicus-textual-representation-format-specification)
      - [The Instruction List](#the-instruction-list)
      - [Compilation Examples](#compilation-examples)
        - [Hello World](#hello-world)
  - [Digicus IDE](#digicus-ide)
    - [How to Use It?](#how-to-use-it)

***

## [Digicus Programming Language](#Digicus-Programming-Language)

Heavily influenced by [Scratch](https://scratch.mit.edu/), Digicus is a novel, block-based, visual smart contract programming language consisting of a predefined set of *blocks* which users piece together to define Soroban smart contracts. 

### [Initial Architecture - Stellar's Soroban](#Overall-Architecture)

![overall architecture](./technical_architecture_artifacts/overall_architecture.png)

Digicus is accessible via a browser based IDE (React frontend with Rails backend) that serves smart contracts as defined in the DTR (Digicus Textual Representation) format. It comes with a Rust to DTR (Rust crate) and DTR to Rust compiler (Ruby gem) such that it can then leverage the Rust toolchain to compile to WASM and submit the contract to the Stellar network. Over time we plan to not only support other programming language SDKs but also eventually WASM directly.

In compiler terms, think of Digicus like the frontend language, DTR and Rust as the intermediate representations, and then WASM as the backend language (interpreted and executed by Stellar core nodes running the Soroban VM).

***

### [Digit](#Digit)

The Digicus compiler (*digit*) is a Ruby gem which provides transcompilation from block-based contracts to and from various blockchain programming languages/SDKs. For starters, we'll be supporting the SDF maintained Rust SDK. 

Digit accomplishes this via the following pipelines (where `.dtr` stands for `digicus textural representation`):

1. **[To Contract]**: block-based contract --> `.dtr` file --> `.rs`
2. **[From Contract]**: `.rs` --> `.dtr` --> block-based contract

#### [Digicus Textual Representation Format Specification](#Digicus-Textual-Representation-Format-Specification)

Each `.dtr` file contains the definition for a single contract. Each contract consists of:

* a contract name (`CONTRACT_NAME`)
* a set of functions (the interface)
* state, or data accessible to the contract

Thus, we have three main sections:

1. [Contract]: where the name of the contract is specified
2. [State]: where we define the type and initial data for each variable
3. [Function]: where we define each method (name, input, output, body)

If the `State` or `Function` section is omitted, it will be assumed to be non-existent.

**Overall structure**:
```
[Contract]: CONTRACT_NAME

[State]:
  * STATE_DEFINITION
  * STATE_DEFINITION
  ...
  * STATE_DEFINITION

[Functions]:
  * FUNCTION_DEFINITION
  * FUNCTION_DEFINITION
  ...
  * FUNCTION_DEFINITION
```

**STATE_DEFINITION**:
```
  [STATE_NAME]:
    * Type: TYPE_NAME
    * Initial Value: VALUE
```

Where `TYPE_NAME` can be any valid Rust (Soroban subset) type. (TBD) do we want a 1-to-1 mapping of Rust types to .dtr types? Yes for MVP, but maybe as we implement this it will be (a) easier to generalize and/or (b) clear that new users need not care about this and so we can make *smart* decisions for them. Eventually we may rethink this when we expand to other frontend targets.

**FUNCTION_DEFINITION**:
```
  [FUNCTION_NAME]:
    * Input:
      {
        INPUT_NAME: TYPE_NAME,
        ...
        INPUT_NAME: TYPE_NAME
      }
    * Output: TYPE_NAME
    * Instructions:
      {
        { instruction: INSTRUCTION_NAME, input: (VALUE_NAME: VALUE,..., VALUE_NAME: VALUE), assign: ASSIGN_NAME },
        ...
        { instruction: INSTRUCTION_NAME, input: (VALUE_NAME: VALUE,..., VALUE_NAME: VALUE), assign: ASSIGN_NAME }
      }
```

Note:
* the input section is optional
* the output section is optional
* output may be at most one value
* an instruction need not have an input nor an output
* an `ASSIGN_NAME` is a local variable which may be referenced by following instructions
* `INSTRUCTION_NAME` is the name of a subset (TBD) of supported rust expressions

#### [The Instruction List](#the-instruction-list)

We will provide a standard library of instructions. Much of this will be determined from looking at:
* [contract examples](https://github.com/stellar/soroban-examples)
* [How to Build an SDK Docs](https://developers.stellar.org/docs/tools/sdks/build-your-own)
* [Rust SDK](https://github.com/stellar/rs-soroban-sdk/tree/main)
* [AssemblyScript SDK](https://github.com/Soneso/as-soroban-sdk)

**Instruction List (WIP):**
* AddSymbols
* Return
* FetchFromStorage
* SaveToStorage
* Log
* ... etc.

#### [Compilation Examples](#a-compilation-example)

##### [Hello World](#hello-world)

Taken from the [SDF Soroban example repository here](https://github.com/stellar/soroban-examples/tree/main/hello_world).

**hello_world.rs**

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}
```

**hello_world.dtr**

```dtr
[Contract]: HelloContract

[Functions]:
  * [hello]
      * Input:
        { 
          to: Symbol 
        }
      * Output: Symbol
      * Instructions:
        {
          { instruction: AddSymbols, input: ("Hello", to), assign: HelloToResult }
          { instruction: Return, input: HelloToResult }
        }
```

***

**Digicus IDE**

![digicus ide hello world](./technical_architecture_artifacts/digicus-ide-mockup.png)

***

## [Digicus IDE](#digicus-ide)

The Digicus IDE is the recommended text editor for creating and visualizing Soroban Visual block contracts. It is a React frontend with a Ruby on Rails backend. Visualization is achieved by transforming `.dtr` files into colorful, draggable, editable "Lego-like" blocks. Once contracts are loaded (or in the case of creation, initialized), the IDE presents a host of tools to aid in development:

1. drag and drop interface with real-time error detection on block compatibility
2. a pseudo language server which provides real time feedback on common mistakes, security vulnerabilities, etc.
3. simulation
4. testing
5. auto-complete

The Digicus IDE is powered by the **DTR Render Engine**. The DTR Render Engine consumes DTR formatted files and interprets them to display the block-based contract representation that users can CRUD in the Digicus IDE. Since Digicus consumes DTR files, it can provide compilation to and from *any* Soroban SDK which has DTR compilation support.

### [How to Use It?](#how-to-use-it)

For the initial 1.0 release of Digicus, we will host the IDE on our website (*link TBD*). By hosting the IDE, we lower the barriers to entry and make Digicus as easy to use as possible. However, once folks evolve beyond the learning phase and begin to write more business critical smart contracts, we imagine they will wish to devlop these contracts locally, *owning* the entire development pipeline. Thus, after version 1.0, we will also work to release and support:

1. self-hosting the IDE
2. a desktop version

***