## Debian/Ubuntu

1. Install Rust if you don´t have installed yet:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh        
2. Clone the repository

    ```bash
    git clone https://github.com/rubendd/RustPix.git
3. Navigate to the project directory:
   ```
   cd rustpix 
4. Build the project:
    ```
    cargo build --release
5. Add RustPix to your PATH:
    ```
    echo 'export PATH="$PATH:$PWD/target/release"' >> ~/.bashrc

    source ~/.bashrc
 6. Run RustPix:
    ``` 
    rustpix <input_image> <output_image> [options]
    ```


