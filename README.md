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

This system implements a Zero-Knowledge Authentication Protocol augmented with TOTP and ephemeral Diffie–Hellman key exchange.

The design goals are:
- The server never learns the user's password;
- The server never learns the user's private key x;
- Replay attacks are prevented;
- Both a long-term secret and a time-based one-time factor are required.

The protocol consists of 2 phases:
- Registration
- Login

## Registration

#### Client-Side
1. User provides:
- username
- password

2. Client generates:
- Random private key x from the Zq with CSPRNG
- Computes public key: Y = g^x

3. Client derives encryption key: 
- k = KDF(password, local_salt)

4. Client encrypts private key:
- enc_x = ENC_k(x)

5. Client stores locally:
- username
- local_salt
- enc_x

6. Client sends: (username, Y)

#### Server-Side

1. Server stores: (username, Y)

## Login

Each login attempt uses:
- Fresh Diffie-Hellman keys
- Fresh server nonce
- Fresh Schnorr randomness
- Time-based TOTP

#### Step 1: Client -> Server

Client sends:
- username
- A = g^a (DH public key)

where: 
- a is freshly generated
- A is ephemeral

#### Step 2: Server -> Client

Server sends:
- server_nonce
- B = g^b (DH public key)

where:
-server_nonce is freshly generated 256-bit number
- b is freshly generated
- B is ephemeral

#### Step 3: Shared Secret Derivation

Both parties compute: K = g^(ab)
This is the ephemeral Diffie-Hellman shared secret.

#### Step 4: TOTP Generation

Using key material derived from K:
- TOTP = TOTP(K, current_time)

The TOTP:
- is 6 digits
- expires every fixed-time window (e.g. 60 seconds)

#### Step 5: Schnorr Proof Construction (Client)

User enters password.
Client:
1. Derives: k = KDF(password, local_salt);
2. Decrypts: x = DEC_k(enc_x)
3. Generates fresh random: r from Zq
4. Computes commitment: t = g^r
5. Computes Fiat-Shamir challenge:
- e = H("ZK_TOTP_AUTH"||t||y||A||B||TOTP||server_nonce)
6. Computes response: 
- s = r + ex (mod q)

#### Step 6: Client -> Server

Client sends:
- t
- s

#### Step 7: Verification (Server)

Server:
1. Computes shared secret: K = g^(ab);
2. Computes expected TOTP;
3. Recomputes:
- e = H("ZK_TOTP_AUTH"||t||y||A||B||TOTP||server_nonce)
4. Verifies Schnorr equation:
- g^s =? t * y^e

If valid -> authentication successful.

NOTES:
- All randomness will be generated using a cryptographically secure RNG;
- r must never be reused;
- server_nonce must be unique per login attempt;
- Hash function will be SHA-256
- KDF will be Argon2

