#Setup Rust
Following guide to install and configure rust toolchain at https://www.rust-lang.org/tools/install

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
export PATH=$PATH:~/.cargo/bin
rustc --version
```

# Prepare for STM32F1 development

Install the following:
 - Arm compiler and linker and debugger
 - OpenOCD
 - ST-Link USB Driver

 ```
 sudo dnf install arm-none-eabi-gcc-cs arm-none-eabi-binutils-cs openocd stlink
 ```

Install gdb on Fedora

```
sudo dnf copr enable sailer/axide
sudo dnf install arm-none-eabi-gdb-arm
```

Configure rust toolchain: 
```
rustup target add thumbv7m-none-eabi
```

# Setup IDE (vscode)
 - Install VSCode
 ```
 sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
 sudo sh -c 'echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" > /etc/yum.repos.d/vscode.repo'
 sudo dnf check-update
 sudo dnf install code
 ```
 - Install Plugins in VSCode
   - Better TOML (bungcip)
   - C/C++ (Microsoft)
   - Native Debug (WebFreak)
   - Rust (kalitaalexey)
   - Rust (rls) (rust-lang)


# Setup project
 - follow guide at: https://github.com/stm32-rs/stm32f1xx-hal (Setting up your project)

## compile it (command line tools)
```
cargo build
```

## flash it
```
openocd -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
cargo run

# in gdb
continue
```


