# 📊 DeFindex Monorepo

## 🔎 What's inside?

This repo includes the following packages/apps:

### 🥑 Apps and Packages
- `Contracts`: [Soroban smart contracts](https://stellar.org/soroban) for the app.
- `Dapp`: a [Next.js](https://nextjs.org/) & [Soroban React](https://soroban-react.paltalabs.io/) app to manage the indexes.
- `Docs`: a [Gitbook](https://www.gitbook.com/) app to autogenerate the docs of the apps.
- `Landing`: : a [Next.js](https://nextjs.org/) app thats shows relevant info about the project.
- `@repo/ui`: a stub React component library shared by both `web` and `docs` applications
- `@repo/eslint-config`: `eslint` configurations (includes `eslint-config-next` and `eslint-config-prettier`)
- `@repo/typescript-config`: `tsconfig.json`s used throughout the monorepo

Each package/app is crafted with 100% love by the paltalabs team🥑.

## 🐋 Development container:

First run the following code in a new terminal located in the repo folder to initalize the container:

```sh
docker compose up -d
``` 
then, once the container is up, run:
```sh
bash run.sh
# or
docker exec --tty --interactive defindex-soroban bash
```
to explore and use the container.

Once inside the container, install the project dependencies by running:
```sh
yarn
```

### ⚙️ Running apps:
 To run the development instances of the apps simply run `yarn dev`. Turbo repo will automatically run all the apps together.
 If you need to run just one app add the flag `--filter appname`. The code should look like:
 ```sh
yarn dev --filter dapp
```

### 🔨 Build & test Smart contracts:
while inside the docker container you can deploy build and test the smart contracts.
to make this happen, please move into the contracts app folder `cd apps/contracts` and
run `make build`.

## 📄 Smart contracts

To build the smart contracts simply run:

```sh
bash run
cd apps/contracts
make build
```

from the repo root folder.

### Deploy Factory contract
Before deploying any contract, you need to setup your secrets:

So, create a `.env` file in the `apps/contracts` folder with the following content:
```sh
cp apps/contracts/.env.example apps/contracts/.env
```
and fill in the values for the `ADMIN_SECRET_KEY`, `DEFINDEX_RECEIVER_SECRET_KEY` and `MAINNET_RPC_URL` variables.

To deploy the factory contract run:
```sh
bash run.sh
cd apps/contracts
yarn deploy-factory
```
### Publish addresses
Once you have deployed an instance of the factory contract. You can publish the addresses to be used by anyone on the network.
To do this run:
```sh
bash run.sh
cd apps/contracts
yarn publish-addresses <network>
```
where `<network>` is the network you are deploying to. The options are `testnet` or `mainnet`.


## 🔧 Utilities

This Turborepo has some additional tools already setup for you:

- [TypeScript](https://www.typescriptlang.org/) for static type checking
- [ESLint](https://eslint.org/) for code linting
- [Prettier](https://prettier.io) for code formatting

## 🛠️ Build

To build all apps and packages, run the following command:

```
cd my-turborepo
yarn build
```
