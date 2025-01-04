# Quick Start

> Note: Binary releases are not yet available.

## Build dakia from source

- [Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Clone the repository
  ```txt
  https://github.com/ats1999/dakia.git
  ```
- Navigate to the project root
  ```txt
  cd dakia/dakia
  ```
- Build the project
  ```txt
  cargo build --release
  ```

## Running the Binary

Once the build is successful, execute the binary:

```sh
./target/release/dakia
```

On a successful run, dakia will display its version in the following format:

```txt
_______
\  ___ `'.                    .          .--.
 ' |--.\  \                 .'|          |__|
 | |    \  '              .'  |          .--.
 | |     |  '     __     <    |          |  |     __
 | |     |  |  .:--.'.    |   | ____     |  |  .:--.'.
 | |     ' .' / |   \ |   |   | \ .'     |  | / |   \ |
 | |___.' /'  `" __ | |   |   |/  .      |  | `" __ | |
/_______.'/    .'.''| |   |    /\  \     |__|  .'.''| |
\_______|/    / /   | |_  |   |  \  \         / /   | |_
              \ \._,\ '/  '    \  \  \        \ \._,\ '/
               `--'  `"  '------'  '---'       `--'  `"

   ___        _        ___
  / _ \      / |      / _ \
 | | | |     | |     | | | |
 | |_| |  _  | |  _  | |_| |
  \___/  (_) |_| (_)  \___/

```

## Config

`Dakia` uses a configuration directory instead of a single file. Refer to the [CLI Documentation](./cli.md) for instructions on specifying the Dakia directory path.

The configuration directory should contain a file at:

```txt
<dakia-directory>/config/dakia.yaml
```

Dakia directory must have a config file located inside `dakia-directory/config/dakia.yaml`. You can find an example configuration file here: [Sample Config](./config.sample.yaml)

We support MongoDB like query syntax for filtering routes, which you can find in the sample config.

```yaml
routers:
  - upstream: payment
    filter:
      $or:
        ds.req.path:
          $starts_with: /pay
        ds.req.query.type:
          $in:
            - pay
            - simple-pay
            - fake-pay
  - upstream: search
    filter:
      $or:
        ds.req.query.search:
          $exists: true
        ctx.user.authenticated: true
      ds.req.method: POST
```

> Documentation on parsing and applying filters to routes along with other options in config will be available soon!
