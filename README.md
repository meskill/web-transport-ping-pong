# WebTransport Ping-Pong App

## Description

Create a client-server ping-pong application with the following specs:

![client-server](assets/client-server.png)

Essential Requirements:
- Preferably the Client should be written in RUST, but feel free to use any other language
you are comfortable with or believe is best for this operation.
- The Server should be written in Python
- Communication protocol should be in WebTransport
- Provide some unit test coverage for both sides to demonstrate your skill in unit testing

Desirable Requirements:
- Make the communication channel secure or suggest what security measures you would
implement given more time.
- Provide a plan for Kubernetes deployment
- Provide a plan/design for an auto-recovery mechanism for both sides (in case of a
temporary connection failure). Feel free to implement that if you have enough time.
- Provide integration tests
- Can you think of a way for the client to auto-discover the server without the need to point
it to the exact server endpoint?

## Solution

Prerequisites:
- `rust 1.70`

### Run example

1. Go to `crates/server` and execute `cargo run` to start server
2. Go to `crates/client` and execute `cargo run` to run client

### Run Tests

Execute `cargo test` from repository root.

### Security concerns

The WebTransport protocol comes with the security in mind - [specification](https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3/#name-security-considerations-20). So is the implementation in this repo with the help of `wtransport` crate:

- it requires using tls or similar security protocols. To showcase this the current repo uses self-signed certificates that should be replaced with valid certificate from certificate authority
- it prevents from establishing connection and sharing sensitive information through errors in case of the server is not supporting WebTransport protocol

Additional considerations to improve security:
- WebTransport protocol requires the client to provide the origin value from which connection was initiated (yet not implemented by used crate). That value might be used on server to handle only requests only from certain origins
- protocol may use client certificates (yet not implemented by used crate) to provide additional authentication capabilities
- protocol may use URI to describe server endpoints (yet not implemented by sed crate) to add additional validation in browsers through CSP
- depending on the use of communication and whether it should be available in public internet we can restrict server and client inside private network that won't be available from the internet.

### Auto-discover mechanism

Current, implementation requires the client to provide ip address of the server in order to connect.

To simplify management for the ip address we might use DNS-based solution to resolve url of the server to different ips. Depending on the requirements it could be global internet DNS or private DNS such as Kubernetes DNS.

### Kubernetes deployment

1. Use `client.Dockerfile`, `server.Dockerfile` to build images for client and server respectively
2. Use `client.yaml`, `server.yaml` to push deploys to kubernetes cluster
3. Additionally, apply changes to k8s configs to increase number of replicas or specify required resources for the deploys
