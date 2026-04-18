# zk-auth-totp

A password-authenticated, zero-knowledge login system implemented in Rust. The client proves knowledge of its long-term secret key using a Schnorr proof (via Fiat-Shamir transform) without transmitting the password or the key itself. Session freshness is enforced by binding each proof to an ephemeral Diffie-Hellman exchange and a TOTP derived from the resulting shared secret. The server stores only the user's public key; it holds no password hash and no secret material.

---

## Architecture

The project is a Cargo workspace with three crates:

| Crate | Role |
|-------|------|
| `shared` | Cryptographic primitives and message types used by both client and server |
| `client` | CLI application: handles registration, key storage, and login proof generation |
| `server` | TCP server: manages user records and verifies login proofs |

**client** stores the user's secret key `x` encrypted under an Argon2-derived key. The plaintext of `x` never leaves the client.

**server** stores only the user's long-term public key `Y = g^x`. No password, no secret key.

---

## Design Rationale

- **Password never leaves the client.** It is used only to derive a local AES-256-GCM key for encrypting `x` at rest; it is never sent over the network.
- **Server holds no secret.** Storing only `Y = g^x` means a full server compromise exposes no material that can directly authenticate as a user.
- **Proof-based authentication.** Login is a zero-knowledge Schnorr proof of discrete log. The verifier learns nothing about `x` beyond that the prover holds it.
- **Session-bound proof.** The Fiat-Shamir challenge binds the proof to the specific DH exchange (via `A`, `B`) and the current time window (via TOTP and server nonce), preventing replay across sessions.

---

## Protocol Flow

### Registration

1. Client generates a long-term keypair: `x` (scalar), `Y = g^x` (Ristretto point).
2. Client derives an encryption key from the user's password via Argon2id.
3. Client encrypts `x` with AES-256-GCM and stores the ciphertext locally.
4. Client sends `(username, Y)` to the server.
5. Server stores `(username, Y)`.

### Login

```
Client                                          Server
  |                                               |
  |-- LoginStartRequest: (username, A=g^a) -----> |
  |                                               |  generate b, B=g^b, nonce
  |<-- LoginStartResponse: (B, nonce) ----------- |
  |                                               |
  |  K = a·B = g^(ab)                             |  K = b·A = g^(ab)
  |  TOTP = HMAC-SHA256(K, floor(t/60))           |  TOTP = HMAC-SHA256(K, floor(t/60))
  |                                               |
  |  e = SHA-512("ZK_AUTH_TOTP"||t||Y||A||B||TOTP||nonce) |
  |  s = r + e·x                                  |
  |                                               |
  |-- LoginProofRequest: SchnorrProof(t, s) -----> |
  |                                               |  verify: g^s == t · Y^e
  |<-- LoginResult: Success / Failure ------------ |
```

1. Client sends its ephemeral DH public key `A = g^a` alongside the username.
2. Server generates its own ephemeral keypair `(b, B)` and a random 16-byte nonce, then replies with `(B, nonce)`.
3. Both sides independently derive the shared key `K = g^(ab)`.
4. Both sides derive a session TOTP from `K` using HMAC-SHA256 over the current 60-second time window.
5. Client decrypts its stored `x` using the password-derived key, then generates a Schnorr proof:
   - Pick random `r`; compute commitment `t = g^r`
   - Compute Fiat-Shamir challenge: `e = SHA-512("ZK_AUTH_TOTP" || t || Y || A || B || TOTP || nonce)`
   - Compute response: `s = r + e·x`
6. Server recomputes the same TOTP, reconstructs `e`, and verifies: `g^s == t · Y^e`.

---

## Cryptographic Components

| Component | Algorithm | Purpose |
|-----------|-----------|---------|
| Group | Ristretto255 (curve25519-dalek) | Prime-order group for all elliptic curve operations |
| Key exchange | Ephemeral Diffie-Hellman | Establish per-session shared secret |
| Session OTP | HMAC-SHA256 over DH shared key | Bind proof to the current session and time window |
| ZK proof | Schnorr with Fiat-Shamir transform | Prove knowledge of `x` without revealing it; challenge hashed with domain separator `"ZK_AUTH_TOTP"` |
| Hash | SHA-512 | Fiat-Shamir challenge: `e = SHA-512("ZK_AUTH_TOTP" \|\| t \|\| Y \|\| A \|\| B \|\| TOTP \|\| nonce)` |
| Local encryption | AES-256-GCM | Protect secret key `x` at rest on the client |
| Key derivation | Argon2id (19 MB, 2 iterations) | Derive AES key from user password |

---

## Build & Run

**Prerequisites:** Rust toolchain (edition 2021), `cargo`.

```bash
# Build all crates
cargo build --release

# Run the server
cargo run -p server --release

# Run the client
cargo run -p client --release -- register
cargo run -p client --release -- login
```

The server listens on `127.0.0.1:7878`. Both binaries must be run from the workspace root so that the relative data paths (`server/data/`, `client/data/`) resolve correctly.

---

## Example Usage

**Register a new user:**

```
$ cargo run -p client --release -- register
Enter Username: alice
Enter Password:
User 'alice' successfully registered
```

**Login:**

```
$ cargo run -p client --release -- login
Enter Username: alice
Enter Password:
Authentication successful
```

**Wrong password:**

```
$ cargo run -p client --release -- login
Enter Username: alice
Enter Password:
Authentication failed
```

---

## Project Structure

```
zk-auth-totp/
├── Cargo.toml                  # Workspace definition
├── shared/
│   └── src/
│       ├── lib.rs
│       ├── messages.rs         # Client/server message types (serde)
│       └── crypto/
│           ├── mod.rs
│           ├── schnorr.rs      # Proof generation and verification
│           ├── dh.rs           # Ephemeral DH keypair and key derivation
│           ├── totp.rs         # HMAC-SHA256 based TOTP
│           ├── aes.rs          # AES-256-GCM encrypt/decrypt
│           └── kdf.rs          # Argon2id password-to-key derivation
├── server/
│   └── src/
│       ├── main.rs
│       ├── network.rs          # TCP listener, per-connection session state
│       ├── auth.rs             # handle_register, handle_login_start, handle_login_proof
│       └── storage.rs          # UserRecord persistence (JSON files)
└── client/
    └── src/
        ├── main.rs
        ├── network.rs          # TCP client, message serialization
        ├── auth.rs             # register(), login() flows
        └── storage.rs          # LocalUserRecord persistence (JSON files)
```

---

## Security Properties

- **Password never transmitted.** The password is used only locally to derive the AES key that decrypts `x`.
- **Server stores no secrets.** The server holds only the public key `Y = g^x` and cannot recover `x` or the password.
- **Zero-knowledge proof.** The Schnorr proof reveals nothing about `x` beyond the fact that the prover knows it.
- **Session binding.** The Fiat-Shamir challenge incorporates both DH public keys, the TOTP, and the server nonce, preventing replay and cross-session substitution attacks.
- **Forward secrecy (per session).** Ephemeral DH keys are generated fresh for every login; compromise of long-term keys does not retroactively expose past sessions.
- **TOTP freshness.** TOTP is derived from the ephemeral shared key and a 60-second time window, bounding proof validity to the current session window.
- **Local key protection.** The secret key `x` is stored encrypted under AES-256-GCM. Decryption failure (wrong password or tampered ciphertext) surfaces as an authentication error, not a panic.

---

## Limitations

- **No TLS.** The TCP connection is unauthenticated and unencrypted at the transport layer. The cryptographic proof prevents impersonation, but traffic is visible to a network observer.
- **No replay protection beyond the session.** The nonce prevents replay within the same DH exchange; a separate nonce blacklist would be needed to prevent cross-session replay if the same ephemeral keys were reused (they are not, but this is not enforced by the protocol).
- **Clock skew.** TOTP verification uses a single 60-second window with no tolerance for clock skew between client and server. A difference of even a few seconds straddling a window boundary will cause legitimate logins to fail.
- **File-based storage.** User records are stored as plain JSON files with no locking, indexing, or access control beyond the filesystem.
- **Single-threaded server per connection.** Each client connection spawns a dedicated OS thread; this does not scale to large numbers of concurrent clients.
- **No account enumeration protection.** The server returns distinct error messages for "user does not exist" vs. authentication failure.

---

## Comparison to Traditional Authentication

| Aspect | Traditional | This system |
|--------|-------------|-------------|
| What is sent to server | Password or password-derived hash | Nothing secret — only a ZK proof |
| What server stores | Password hash (e.g. bcrypt, Argon2) | Public key `Y = g^x` only |
| Authentication mechanism | Verify a secret | Verify a zero-knowledge proof of knowledge of `x` |
| Session binding | Tokens or TLS | Cryptographically enforced via DH shared key and TOTP |

---

## Dependencies

| Crate | Version | Usage |
|-------|---------|-------|
| `curve25519-dalek` | 4 | Ristretto255 group operations |
| `aes-gcm` | 0.10 | AES-256-GCM encryption |
| `argon2` | 0.5 | Argon2id key derivation |
| `hmac` | 0.12 | HMAC-SHA256 for TOTP |
| `sha2` | 0.10 | SHA-512 for Fiat-Shamir hash |
| `rand_core` | 0.6 | Cryptographic random number generation |
| `serde` / `serde_json` | 1 | Message serialization |
| `rpassword` | 7 | Terminal password input without echo |
