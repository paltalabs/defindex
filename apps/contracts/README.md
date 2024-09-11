## DeFindex Contracts

First, run the development server:

```bash
yarn build
```

### Building contracts inside the container
To build the contracts inside the container you can do 
```bash
make build
```
and to test them run

```bash 
make test
```
if you want to test or build each contract separatedly you can do the same inside the contract directory

### Tests on typescript

to deploy the factory you can use 

```bash 
yarn deploy-factory <network>
```

to test the factory to deploy a DeFindex Vault

```bash
yarn test <network>
```

