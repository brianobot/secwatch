# SecWatch (Currently in Development: Installation Steps would not work as of yet)

This is a command-line tool for checking for common security vunerabilty in your backend REST APIs.

The Vunerablities covered are:
- Rate Limiting
- DDOS Protection
- Security Headers
- CORS Misconfiguration
- Trusted Domain Misconfiguration
- Sensitive Data Exposure (.env file exposure)


## Installation
To install secwatch you need to have Rust and Cargo installed on your system.
You can install secwatch with the following command

```bash
cargo install secwatch
```

## Basic Usage 

Secwatch provides several commands to control the security check you wish to watch

### `init`
```bash
secwatch init
```

### `run`
```bash
secwatch run 
```


## Maintainer
- Brian Obot <brianobot9@gmail.com>