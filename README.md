# One-End Encryption (OEE): Stronger than End-to-End Encryption

Today everybody uses end-to-end encryption, but it's not secure enough
for you! Powerful state-sponsored attackers have the ability to break
end-to-end encryption. **One-end encryption** (OEE) solves the problem, by
sending ciphertexts literally impossible to decrypt, even by advanced
cyber criminals and the NSA.

Specifically:

* OEE is **secure against hostile endpoints**, while end-to-end encryption
  is only secure against hostile networks

* OEE is **post-quantum**, quantum-safe, quantum-resilient 

* OEE is secure even if **P=NP**, even alien technologies won't break OEE

* OEE is secure against **anthropic computing** attacks

* OEE is **post-Snowden** and **NSA-proof**

* OEE is **memory-efficient**, with terabytes of data encrypted into a
  32-byte ciphertext

* OEE software is safe, because it's coded in **Rust**

* OEE is **gluten-free** and **non-GMO** (and 99% vegan)

* OEE is **provably secure** 

* OEE is fully **constant-time**: the runtime of OEE doesn't depend on
  the input length 
  
* OEE is **Theresa May approved**

* OEE guarantees total **forward secrecy**, **backward secrecy**, and
  **frontward secrecy**, meaning that your messages are safe against
  future, past, and present attacks

* OEE has **no backdoor** (really) and **no front door either**, so the
  FBI can't use that. There are also no windows, skylights, cat flaps,
  gates, or other ingress points for human-sized lifeforms or lifeforms
  trained by humans to retrieve cryptographic keys with warrants

* OEE **passed security audits** with flying colors

To try OEE, use our reference implementation:

```
cargo run file-to-encrypt
```

To use the in-place feature (be aware, this is only to be used by the very-braves):

```
cargo run -- -i file-to-encrypt
```
