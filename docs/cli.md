# Dakia CLI

### `--dp`

- **Description**: Path to Dakia's local directory for storing configuration, interceptors, filters, extensions, and runtime data.
- **Type**: `String`
- **Example**: `--dp "/path/to/dakia"`

---

### `--watch` / `-w`

- **Description**: Watch for changes in configuration files, interceptors, filters, and extensions, and automatically apply updates.
- **Type**: `bool`
- **Example**: `--watch`

---

### `--reload`

- **Description**: Reload configuration files and update runtime settings. This may trigger a graceful restart if required.
- **Type**: `bool`
- **Example**: `--reload`

---

### `--test` / `-t`

- **Description**: Test the server configuration without starting the application.
- **Type**: `bool`
- **Example**: `--test`

---

### `--version` / `-v`

- **Description**: Display the current version of the API Gateway and exit.
- **Type**: `bool`
- **Example**: `--version`

---

### `--verbose`

- **Description**: Enable verbose logging for more detailed output. Useful for debugging and monitoring.
- **Type**: `bool`
- **Example**: `--verbose`

---

### `--debug`

- **Description**: Enable debug mode to output additional debugging information. Use this to troubleshoot issues during development or runtime.
- **Type**: `bool`
- **Example**: `--debug`

---

### `--upgrade` / `-u`

- **Description**: Enable the server to attempt an upgrade from a running older server. This feature is supported only on Linux platforms.
- **Type**: `bool`
- **Example**: `--upgrade`

## Example Usage

1. **Start Dakia with a specific configuration directory and watch mode enabled:**
   ```bash
   dakia --dp "/path/to/dakia" --watch
   ```
1. **Print dakia version:**
   ```bash
   dakia --version
   dakia -v
   ```
   **Output**

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
