# One-End Encryption (OEE): Stronger than End-to-End Encryption

Today everybody uses end-to-end encryption, but it's not secure enough
for you! Powerful state-sponsored attackers have the ability to break
end-to-end encryption. **One-end encryption** (OEE) solves the problem, by
sending ciphertexts literally impossible to decrypt, even by advanced
cyber criminals and the NSA.

Specifically:

* OEE is **secure against hostile endpoints**, while end-to-end encryption
  is only secure against hostile networks

* OEE guarantees total **forward secrecy**, **backward secrecy**, and **frontward
  secrecy**, meaning that your messages are safe against future, past, and
  present attacks

* OEE is **post-quantum**, quantum-safe, quantum-resilient 

* OEE is secure even if **P=NP**, a.k.a. informationally secure (even
  alien technology won't break OEE)

* OEE is **post-Snowden** and **NSA-proof**

* OEE is **memory-efficient**, with terabytes of data encrypted into a
  32-byte ciphertext

* OEE software is safe, because it's coded in **Rust**

* OEE is **gluten-free** and **non-GMO**


To try OEE, use our reference implementation:

```
cargo run file-to-encrypt
```


