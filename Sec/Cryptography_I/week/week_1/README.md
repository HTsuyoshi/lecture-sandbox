# Section 1

## Where Cryptography is used?

### The core of cryptography

Secure key establishment:

- Alice knows she is talking with Bob and Bob knows he is talking with Alice

Secure communication:

- Confidentiality and Integrity

    - Digital Signatures (2 half of the course)
    - Anonymous communication (Mix net)
    - Anonymous digital cash (Digital coin)

Secure comunication:

- Web traffic: HTTPS (SSL, TLS)
    - Secure Sockets Layer
        - 1 Handshake Protocol: Estabilish shared secret key using public-key cryptography
        - 2 Record Layer: Transmit data using shared secret key

- Wireless traffic: 802.11iWPA2, GSM, Bluetooth

Encrypting files on disk: EFS, TrueCrypt

- Alice communicates with alice tomorrow

Content protection (DVD, Blue-ray): CSS, AACS

User autentication

## Symmetric encryption

Building block: Symmetric encryption

E, D: cipher
m, c: plaintext, ciphertext
k: secret key (128 bits)

Encryption algorithm is publicly known;

- Never use a proprietary cipher

## Use cases

### Single used key (NONCE)

- Key is only used to encrypt one message
    - Encrypted email: new key generated for every email

### Multi use key (Many time key)

- Key used to encrypt multiple messages
    - Encrypted files: same key used to encrypt many files
- Need more machinery than for one-time key

## Important

- A tremendous tool
- The basis for many security mechanisms

- Isn't the solution to all security problems
- Reliable unless implemented and used properly
- Something you should try to invent yourself
    - many many examples of broken ad-hoc designs

## Crypto Magic

### Privately outsourcing computation



### Zero Knowledge


## Cryptography steps

Three steps:

- Precisely specify threat model
- Propose a construction
- Prova that breaking construction under threat mode will solve and underlying hard problem

## Books

### History of Cryptography

- The Code Breakers (1996) by David Kahn

### Discrete Probability

- http://en.wikibooks.org/High_School_Mathematics_Extensions/Discrete_Probability
