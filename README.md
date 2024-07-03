# ChirpStack open-source LoRaWAN(R) Network Server (but it can do TS-LoRa)

## How to compile and run the debug build?

I assume that you are running a clean installation of the Debian 13. The build might work on other systems, but I have not tested that.
Here are the steps that you need to undertake to achieve the debug build optimized for the build speed, not for performance and reliablity:

1) Open the bash shell either by using the GUI or by clicking Ctrl + Alt + T.
2) Install git to able to conveniently downliad the source code of this program (I assume that you are logged in as a non-root user who can do sudo):
```bash
   sudo apt update
   sudo apt install git -y
```
3) Move to the Documents folder (it is optional, everything is fine as long as your shell session is located in one of the subdirectiories of the HOME folder)
```bash
   cd ~/Documents]
```
4) Download this project's (i.e. ts-lora chirpstack) source code from Github:
```bash
   git clone https://github.com/ts-lora/ts-lora-chirpstack.git
```
5) Move to the project's directory:
```bash
   cd ts-lora-chirpstack
```
6) Install curl to be able to download Nix:
```bash
   sudo apt install curl -y
```
7) Install xz-util to be able to unpack Nix:
```bash
   sudo apt install xz-utils -y
```
8) Install Nix:
```bash
  sh <(curl -L https://nixos.org/nix/install) --no-daemon
  . /home/bob/.nix-profile/etc/profile.d/nix.sh
```
9) Restart the shell session:
```bash
   exec $SHELL
```
10) Install docker engine through its convenience script:
```bash
   curl -fsSL https://get.docker.com -o install-docker.sh
   sudo sh install-docker.sh
```
11) Allow to run the docker engine commands as a non-root user:
```bash
   sudo groupadd docker
   sudo usermod -aG docker $USER
   newgrp docker
```
12) Start the development shell:
```bash
   nix-shell
```
13) Build the Chirpstack UI:
```bash
    $ make build-ui
```
14) Run the required services for the Chirpstack
```bash
    $ docker compose up -d
```
15) Run the Chirpstack tests (optional):
 ```bash
    $ make test
```
16) Install the Chirpstack's dependencies:
```bash
    $ make dev-dependencies
```
17) Do the debug build:
```bash
    $ cd chirpstack
    $ make debug-amd64
```
18) Run chirpstack
```bash
    $ cd chirpstack
```
```bash
    $ cargo run -- --config-dir configuration
```
## License

ChirpStack Network Server is distributed under the MIT license. See also
[LICENSE](https://github.com/brocaar/chirpstack/blob/master/LICENSE).
