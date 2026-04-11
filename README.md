# ZK Authentication with TOTP

## Overview

This project implements a zero-knowledge authentication protocol in Rust, replacing traditional
password-hash verification with a Schnorr proof-of-knowledge construction. The client proves
knowledge of a long-term secret scalar `x` without revealing it, using a Fiat–Shamir challenge
bound to an ephemeral session context.

Session context is established via an ephemeral Diffie–Hellman exchange. From this a shared key
`K` is derived. `K` is used to seed a time-based one-time password (TOTP), adding a second
authentication factor and providing replay resistance. The system runs as a Rust client–server pair
communicating over a TCP connection.

---

## Design Goals

- The server never receives or stores the user's password
- The server never learns the user's private key `x`
- Replay attacks are prevented via a per-session server nonce and time-bound TOTP
- Authentication requires both a long-term secret (ZK proof) and a time-based factor (TOTP)
- Private key `x` is stored locally under AES-256-GCM encryption, keyed by a password-derived key

---

## System Architecture

```
zk-auth-totp/
├── shared/     # Cryptographic primitives used by both client and server
│               # (Schnorr keypair gen, DH, AES-256-GCM, Argon2id KDF)
├── client/     # Registration, login initiation, local key storage
└── server/     # TCP listener, request dispatch, user record storage
```

Messages are serialized with `serde_json` over newline-delimited TCP streams.

---

## Protocol Overview

### Registration

**Client:**

1. Prompts for username and password
2. Generates long-term keypair with CSPRNG:
   ```
   x ← Zq   (random scalar)
   Y = g^x   (Ristretto basepoint)
   ```
3. Derives local encryption key:
   ```
   salt ← random 128-bit value
   k = Argon2id(password, salt)
   ```
4. Encrypts and stores `x` locally:
   ```
   (enc_x, nonce) = AES-256-GCM(k, x)
   stored: { username, salt, nonce, enc_x }
   ```
5. Sends `(username, Y)` to server

**Server:**

1. Receives `(username, Y)`
2. Stores `{ username, Y }` to disk (JSON)

---

### Login

Each login attempt uses fresh DH ephemeral keys, a fresh server nonce, fresh Schnorr randomness,
and a time-bound TOTP.

**Step 1 — Client → Server**

Client generates ephemeral DH key and sends:
```
a ← Zq   (fresh random scalar)
A = g^a
send: (username, A)
```

**Step 2 — Server → Client**

Server verifies the username exists, generates its own DH key and nonce, and sends:
```
b ← Zq   (fresh random scalar)
B = g^b
nonce ← random 128-bit value
send: (nonce, B)
```

**Step 3 — Shared Secret Derivation**

Both parties independently compute:
```
K = g^(ab)   (ephemeral DH shared secret)
```

**Step 4 — TOTP Derivation**

Using key material from `K`:
```
TOTP = TOTP(K, current_time_window)
```
The TOTP is a 6-digit value with a fixed time window (e.g. 60 seconds).

**Step 5 — Schnorr Proof Construction (Client)**

Client decrypts `x`, generates fresh Schnorr randomness, and constructs a proof:
```
k = Argon2id(password, salt)
x = AES-256-GCM-Decrypt(k, enc_x)

r ← Zq   (fresh, never reused)
t = g^r   (commitment)

e = SHA-256("ZK_TOTP_AUTH" || t || Y || A || B || TOTP || nonce)   (Fiat–Shamir challenge)

s = r + e·x  (mod q)   (response)
```

Client sends: `(t, s)`

**Step 6 — Verification (Server)**

Server recomputes `K`, derives the expected TOTP, recomputes the challenge, and verifies:
```
K = g^(ab)
TOTP = TOTP(K, current_time_window)

e = SHA-256("ZK_TOTP_AUTH" || t || Y || A || B || TOTP || nonce)

verify: g^s == t · Y^e
```

If both the TOTP and the Schnorr equation hold, authentication succeeds.

---

## Cryptographic Components

| Component | Role |
|---|---|
| Ristretto255 (`curve25519-dalek`) | Prime-order group for all scalar/point operations |
| Schnorr + Fiat–Shamir | Non-interactive zero-knowledge proof of knowledge of `x` |
| Ephemeral Diffie–Hellman | Per-session shared secret derivation |
| TOTP | Time-bound second factor; replay resistance |
| SHA-256 (`sha2`) | Fiat–Shamir transcript hash |
| Argon2id (`argon2`) | Password-based key derivation for local encryption |
| AES-256-GCM (`aes-gcm`) | Authenticated encryption of `x` at rest |
| `OsRng` (CSPRNG) | All randomness: `x`, `r`, `a`, `b`, nonces, salts |

---

## Current Status

| Component | Status |
|---|---|
| Registration (end-to-end) | Implemented |
| Client DH key exchange | Implemented |
| Server DH response | Implemented (Server-side session handling (state persistence across messages) — under development) |
| AES-256-GCM local key storage | Implemented |
| Argon2id KDF | Implemented |
| TCP server with per-client threads | Implemented |
| Message serialization (serde_json) | Implemented |
| Schnorr proof construction — partially implemented (key generation complete, proof logic in progress) |
| Schnorr verification (`verify`) | Under development — not yet implemented |
| TOTP generation and verification | Under development — not yet implemented |
| Full login flow (proof + verification) | Under development |
| Server response sending | Stub — `send_response` not yet complete |

---

## Notes

- All randomness (`x`, `r`, `a`, `b`, nonces, salts) is generated via `OsRng`, the OS CSPRNG
- `r` (Schnorr commitment scalar) must never be reused; each login generates a fresh value
- The server nonce must be unique per login attempt to prevent replay
- The Fiat–Shamir transcript binds the proof to the full session context: `t`, `Y`, `A`, `B`, TOTP, and nonce
- This is a learning and experimentation project in applied cryptography — not intended for production use
