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
For more information about dapp, please refer to the [dapp README](apps/dapp/README.md).    

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
yarn deploy-factory <network>
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

## Dart Package development

First, get the specific dart container up by running:
```sh
docker compose up -d dart
```
The genereal docker compose won't launch the dart service.

then, once the container is up, run:
```sh
docker exec -it dart-defindex bash
```
Then you can launch the example app by running:
```sh
cd example
flutter run -d web-server --web-port 8080 --web-hostname 0.0.0.0
```
This will launch the example app on the port 8080 of your localhost. so you can go to
`http://localhost:8080` to see the app.

If you are getting the folowing error:
```
[+] Running 0/0
 ⠋ Container dart-defindex  Starting     0.0s 
Error response from daemon: network 9b51654ecff85fb4a6176438b85fc52f19e12f994427193e5c35bad48c9c7917 not found
```
You need to do 
```
docker rm dart-defindex
```

### Publish dart package
This is published on https://pub.dev/packages/defindex_sdk

Once everything is ok, you can run the following command to check the package:
```sh
dart pub publish --dry-run
```

once, there is no issues you can run the following command to publish the package:
```sh
dart pub publish
```
It will prompt a link to login:
```sh
Please login with your Google account: http://localhost:<port>/?code...
```
Then, you will need to open a terminal connected to the container and run the following command:
```sh
curl http://localhost:33791/?code...
```
This will log you in and you can publish the package.

## TypeScript SDK Development

See the [TypeScript SDK README](./packages/defindex-sdk/README.md) for more information.

You can use the defindex-soroban container to develop the SDK.
```sh
bash run.sh --nb
```
Then, move to the folder `packages/defindex-sdk`.

### Publish TypeScript SDK
Inside the container, on the `defindex-sdk` folder, run:
```sh
# Login to npm
npm login

# Install dependencies
yarn install

# Build the package
yarn build

# Publish to npm
npm publish --access public
```

## 📄 Whitepaper

To generate a pdf version of the whitepaper, you need to install mdbook:
```sh
cargo install mdbook
# Install mdbook-pdf
cargo install mdbook-pdf

# Install mdbook-katex
cargo install mdbook-katex
```

 Then, run the following command:
```sh
cd apps/docs/10-whitepaper
mdbook build

```