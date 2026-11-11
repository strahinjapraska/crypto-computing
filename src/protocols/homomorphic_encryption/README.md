## Testing
The test initialize the parties and goes through all the possible combinations of blood types. To initialize Alice, key pair for the FHEncryption is generated first using the proposed security parameters.
For each combination of blood types, Alice sends her inputs to Bob through a common communication channel, Bob then evaluates the blood compatibility function using his inputs as constants (to not increase the noise), then Bob sends the result to Alice, who decrypts it and returns it. The decrypted result is compared to the output of the standalone boolean function.

### To run the test
From the root of the repository run:
```
cargo test --package crypto-computing --lib -- protocols::homomorphic_encryption::protocol::tests::test --nocapture
```

## Security parameters discussion
