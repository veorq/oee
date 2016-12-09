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

* OEE is **gluten-free** and **non-GMO** (and 99% vegan)

* OEE protects from **cyber APT**

* OEE is **provably secure** and **formally verified** by cryptographers
  with PhDs

* OEE is **constant-time** and **leakage-resilient**

* OEE is **asymptotically maximally memory-hard**

* OEE has **optimal rigidity**

* OEE has **no backdoor** (really) and **no front door either**, so the FBI can't use that. 
  There are also no windows, skylights, cat flaps, gates or other ingress points for human-sized 
  lifeforms or lifeforms trained by humans to retrieve cryptographic keys with warrants.

* OEE has **no less than 5 (five) rachets**. It can self-heal its self healing properties 
  and contains a built in copy of Whitfield Diffie and Martin Hellman in case we ever need to 
  reinvent key exchange.

* OEE is based on **really-really-really-twisted edwards curves**. Have you ever left ethernet cables 
  in a big pile for a year and then tried to untangle them? That's how twisted this curve is. 
  This is the kind of twist-security DJB dreams about.

* OEE **is upset its author did not base it on lattices** as RLWE is the new hotness.

* OEE **is meta-data free** by design. Who can reconstruct your social graph when you only 
encrypt at one end?

* OEE supports **In-Place** ciphering for the **Very-Braves**

To try OEE, use our reference implementation:

```
cargo run file-to-encrypt
```

To use the in-place feature (be aware, this is only to be used by the very-braves):

```
cargo run -- -i file-to-encrypt
```
