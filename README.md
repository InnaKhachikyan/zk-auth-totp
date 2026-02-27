# ZK Authentication Augmented with TOTP

## Overview

This project will implement a zero-knowledge authentication protocol in Rust based on Schnorr proofs, replacing traditional hash(password + salt) verification with a proof-of-knowledge construction. The protocol will be augmented with time-based one-time passwords (TOTP) to provide time-bound security and replay resistance.

Authentication will succeed only if the client:

1. Proves knowledge of a long-term secret without revealing it
2. Provides a valid time-based one-time password derived from a shared session key

The system will consist of a Rust-based client and server communicating over a TCP connection.

## System Components

- Shared Library
- Server
- Client

## Shared Library 

The shared library will contain code that both the client and server will use. It will hold the functionality that both components need. Both the client and server will need to perform the same cryptographic operations (like Diffie-Hellman, time-based OTP etc.). Instead of duplicating this code in both the client and server, it will be put in the library that both can depend on.

## Server

The server will handle authentication and user management. It will store user records (username, public key), generate unique session ID-s, accept new registrations, verify Schnorr proofs, listen for incoming connections.

## Client

The client will manage user secrets and perform authentication. It will generate secret scalar x during registration, compute public key, store the secret and load it on client startup, generate a new keypair during registration, send registration request with username and public key, handle registration response, derive shared session key via DH, generate TOTP, compute Schnorr proof, sned auth messages, prompt for username, display authentication results.


## Registration

During registration, the client generates a long-term secret scalar x and computes the corresponding public key Y = g^x over the Ristretto group. The public key is transmitted to the server and stored under the associated username. The client retains x locally, and the server stores only the public key, ensuring that no password or secret material is ever transmitted or persisted server-side.

## Login

During login, the client initiates an ephemeral Diffie–Hellman key exchange with the server to derive a fresh session key K. Using this key, the client generates a time-based one-time password (TOTP) and produces a Schnorr zero-knowledge proof of knowledge of its long-term secret. The proof is bound to the session identifier, server nonce, and OTP via transcript hashing, ensuring replay resistance and time-bound authentication. The server verifies both the TOTP and the Schnorr proof before granting access.
