# TukTak

Rust app using SSH/gRPC for remote command execution on large networks of machine.

# Libraries

- [getopts] for command line parsing
  https://docs.rs/getopts/latest/getopts/
- [openssh] for SSH connections and requests
  https://docs.rs/openssh/latest/openssh/

# Usage

## Options

- -o, --out, FILENAME, "Output file for commands results"
- -p, --proto, SSH/GRPC, "Protocol to use (ssh or grpc)"
- -r, --recap, FILENAME, "Generates recap of remote executions & performances"
- -k, --command, COMMAND,"Command to execute on remote machine"
- -K, --commands, FILENAME ,"File containing commands to execute on remote machine"
- -c, --connexion, SSH_FORMATED_INFOS, "Connexion information for remote machine"
- -C, --connexions, FILENAME, "File containing connexion information for remote machine"

## Flags

- --thr, "Threaded execution"
- --async, "Asynchronous execution" (Default)
- --deploy, "Auto-deployment active"
- --no-deploy, "Auto-deployment inactive" (Default)

- -h, --help, "Print help menu"
- --Khelp, "Print commands required file formatting"
- --Chelp, "Print connexions required file formatting"
- -v, --version, "Print version"
- -d, --default, "Print default configuration"

## Resulting checks

- -h, -v, -d, --Khelp, --Chelp execute and exit instantly

- --thr and --async are mutually exclusive
- --deploy and --no-deploy are mutually exclusive
- -k and -K are mutually exclusive
- -c and -C are mutually exclusive
- --proto GRPC and --deploy are mutually exclusive
