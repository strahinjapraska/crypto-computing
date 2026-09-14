## Testing
The test goes through all the possible combinations of blood types.
For each combination, it creates two parties (Alice and Bob), a dealer and a protocol instance. After each run of the protocol, it displays the the intermediary states (their shares) of each party, and fails if the final output is different from the output of 
the standalone boolean function.

### To run the test
From the root of the repository run:
```
cargo test --package crypto-computing --lib -- protocols::bedoza::bedoza::test::test_bedoza_protocol --nocapture
```
