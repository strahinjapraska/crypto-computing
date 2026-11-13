## Testing
The test initialize the parties and goes through all the possible combinations of blood types. To initialize Alice, key pair for the FHEncryption is generated first using the proposed security parameters.
For each combination of blood types, Alice sends her inputs to Bob through a common communication channel, Bob then evaluates the blood compatibility function using his inputs as constants (to not increase the noise), then Bob sends the result to Alice, who decrypts it and returns it. The decrypted result is compared to the output of the standalone boolean function.

### To run the test
From the root of the repository run:
```
cargo test --package crypto-computing --lib -- protocols::homomorphic_encryption::protocol::tests::test --nocapture
```

## Security parameters discussion
We are aiming for 60 bits security. The security against brute force attacks for finding approximate-GCD (and get the secret key) depends on the size of `r`s, so their size is set to 60 bits. From that, we deduce the size of `p` such that the noise does not become a problem. Each multiplication doubles the bit-length of the noise. In our implementation, we have 3 multiplications, and thus the noise is aplified by 8 (double of that when considering all the constants). For it not to be an issue, the size of `p` should be roughly 17 times larger than `r`. So `p` is set to be 
a prime of 1024 bits, which is larger than 17 times size of `r`. For security against bruteforcing through the public key components to get the plaintext from a ciphertext,v`n` is chosen to be 128, and the sample size |S| is chosen to be 16. There are more than 9 * 10^19 ways to choose 16 samples out of 128, which is a number of bit size 66, more than the required 60 bits of security. Finally, `q` is chosen to be double the bit size of `p`, however this will not prevent against LLL attacks. In the end, we have:
```
|p| = 1024
|q| = 2048
n = 128
|S| = 16
|r| = 60
```
The chosen parameters can be seen in `setup_parties()` at `src/protocols/homomorphic_encryption/protocol.rs`.