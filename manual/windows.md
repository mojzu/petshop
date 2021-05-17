# Windows Setup

Install dependencies

-   [WSL2](https://docs.microsoft.com/en-us/windows/wsl/install-win10)
-   [Docker Desktop](https://www.docker.com/products/docker-desktop)
-   [Docker Desktop WSL2 Backend](https://docs.docker.com/docker-for-windows/wsl/)

To check dependencies are installed

```shell
wsl --help
docker --version
docker-compose --version
```

Install and start Debian distribution, install dependencies

-   [Rust](https://www.rust-lang.org/)

```shell
sudo apt-get update \
    && sudo apt-get install git curl libssl-dev pkg-config build-essential \
    && sudo apt-get upgrade
cargo install --force cargo-make
```

To check dependencies are installed

```shell
docker --version
docker-compose --version
cargo --version
cargo make --version
```

Clone git repository into `~/` directory, instead of the Windows filesystem in `/mnt`

To save a copy of the distribution, exit terminals and run

```shell
wsl --shutdown
wsl --export ${Distro} ${FileName}.tar.gz
```

To import a saved copy of a distribution

```shell
wsl --import ${Distro} ${InstallLocation} ${FileName}.tar.gz
wsl --list -v
```

Setup default user in imported distribution

```shell
wsl -d ${Distro}
echo -e "[user]\ndefault=${UserName}" >> /etc/wsl.conf
logout
```

Connect to distribution using [WSL Remote](https://code.visualstudio.com/docs/remote/wsl) in [VSCode](https://code.visualstudio.com/), open cloned directory
