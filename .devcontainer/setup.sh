apt-get update
apt-get upgrade -y
apt-get install -y nala

nala install -y curl git jq sudo zsh vim gnupg2 openssl build-essential

## Install common components
rustup component add rustfmt
rustup component add clippy

cargo install cargo-nextest
cargo install cargo-update
cargo install cargo-expand
cargo install cargo-readme
cargo install cargo-watch
cargo install cargo-edit

## Setup and install oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
cp -R /root/.oh-my-zsh /home/$USERNAME
cp /root/.zshrc /home/$USERNAME
sed -i -e "s/\/root\/.oh-my-zsh/\/home\/$USERNAME\/.oh-my-zsh/g" /home/$USERNAME/.zshrc
chown -R $USER_UID:$USER_GID /home/$USERNAME/.oh-my-zsh /home/$USERNAME/.zshrc