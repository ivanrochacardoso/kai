# LINUX 
sudo apt update
sudo apt install curl build-essential pkg-config libx11-dev libxtst-dev libxcb1-dev libxcb-util0-dev libxfixes-dev libxrandr-dev libxinerama-dev libxi-dev
sudo apt-get install libxdo-dev libxtst-dev libxrandr-dev libx11-dev libxi-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
   
cargo new kai
cd kai
cargo run


para compilar release;

cargo build --release

Se quiser um binário portável que rode até sem dependências dinâmicas, pode compilar com musl:

sudo apt install musl-tools
cargo build --release --target x86_64-unknown-linux-musl

O executável fica totalmente estático.
