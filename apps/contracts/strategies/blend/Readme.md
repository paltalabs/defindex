# Blend Strategy
- Gets initialized with the blend pool address, the underlying asset that will be used to deposit on the pools and the BLND reward tokens
- On every deposit it will claim and reinvest the BLND reward tokens
- On deposit and withdraw returns the underlying balance of the caller (DeFindex Vault?) so they can track investment returns