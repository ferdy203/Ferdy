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

<!-- ![dakia logo](https://github.com/user-attachments/assets/7877c4bb-4358-4297-9213-e29d81550f99) -->

![dakia logo (1)](https://github.com/user-attachments/assets/44a908dd-a79c-4045-9e3e-b3125a5efdc5)


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

<a href="https://www.vecteezy.com/free-vector/indian-postman">Indian Postman Vectors by Vecteezy</a>

## TODO

    upstream backend {
        # Define backend servers
        server backend1.example.com;
        server backend2.example.com;
    }

- allow options to specify backend seperatly not inside location block
- once backends are specified, then users can specify which backend to use from location block
- if no path specified, then atleast / path must be specified
- apart from just path in the location block, users can specify, header, etc as well
- allow a plugin for users to return path (dynamic linking)
