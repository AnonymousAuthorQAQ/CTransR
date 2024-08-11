# CTransR: Automated Code Translation from C/C++ to Rust

## Overview

CTransR is an automated approach for translating code from C/C++ to Rust. The method combines self-prompt engineering with a genetic algorithm to guide a large language model (LLM) in generating Rust code from given C/C++ instances. The translation process is designed to maintain the same I/O functionality as the original C/C++ code.

## Installation and Dependencies
```bash
httpx               # For making asynchronous HTTP requests
zhipuai             # For interacting with the ZhipuAI API
requests            # For making HTTP requests and handling exceptions
openai              # For interacting with the OpenAI API
```

- build
```bash
pip install -r requirements.txt
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run

- Add Your API Keys:
    
    Locate the `CTransR/api` directory in your project.
    Add your API keys in the appropriate files within this directory. Each API key should be placed in the corresponding file, depending on which service it is for (e.g., ZhipuAI, OpenAI, etc.).
- Install Python Dependencies
    ```
    pip install -r requirements.txt
    ```
- Install Rust and Cargo
    1. Install Rust and Cargo:

        If Rust and Cargo are not already installed, you can install them using rustup:
        ```bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        ```
    2. Verify the installation:

        After installation, verify that Rust and Cargo are correctly installed:
        ```bash
        rustc --version
        cargo --version
        ```
- Run the Genetic Algorithm with Self-prompt
    ```bash
    cd CTransR/ga
    # we run CTransR with GPT4o mini as an example. 
    ./miniga.sh 
    ```

## Dataset EvalC2R
EvalC2R is a benchmark dataset specifically constructed to evaluate the effectiveness of automated code translation from C/C++ to Rust. The dataset includes a variety of C/C++ code instances drawn from real-world scenarios, particularly focusing on the challenging aspects of code translation。




