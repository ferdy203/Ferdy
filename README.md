<!--
```text
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
```
-->

<!-- canva logo url -> https://www.canva.com/design/DAGZAdY1d9c/YCHWZRD78H5j0CAWaaF6gw/edit -->

![dakia logo](https://github.com/user-attachments/assets/7877c4bb-4358-4297-9213-e29d81550f99)

A next generation multi language programmable proxy & API gateway

## Plan

- Create an api gateway with all features that we want to support
- Make gateway as a package
- Move features from core code to extensions as a seperate crete

## Features

- Nginx features
- Support of customization in other languages
- Extensions/Plugins based archetecture
  - They can be written in any language
  - Allow popular extensions in static linking
  - Allow users to install plugins at run time or start time (Dynamic Linking)
- TCP/UDP/SMTP etc proxy
- gRPC proxy
- WebSocket proxy
- Static content serving

## POC project

- HTTP proxy
- load balancing
- virtual hosts
- SSL/TLS
- Caching
- Authentication

====================================================

- Archetecture
- router_config
  - a router_config can have multiple server_config
    - a server_config can have listen on multiple ports
    - a server can process request for multiple hosts
