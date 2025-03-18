# x86_os

A minimal OS for the x86_64 architecture.

## Installation

Clone this Repo via
```$ git clone https://github.com/lmeller-git/x86_os.git && cd x86_os```

Install nightly rust
```$ rustup install nightly```

add target architecture to rustup
```$ rustup target add x86_64-unknown-none```  

Add rustup components
```$ rustup component add rust-src llvm-tools-preview --toolchain nightly```

Install necessary dependancies (Ubuntu)  
```$ cargo install bootimage && sudo apt-get update && sudo apt-get install -y quemu-system-x86```

## Usage

run bootimage (in dir x86_os)  
```$ cargo bootimage```

run the os in quemu
```cargo run --release```
