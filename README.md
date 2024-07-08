# DeFindex Monorepo

## Smart contracts

To build the smart contracts simple run

```sh
yarn build --filter contracts
```

from the src directory

### development

To test and develop the smart contracts first you would need to start the docker containers by running

```sh
docker compose up -d
```

This will start a standalone stellar instance where you can deploy and test smart contracts

now you can attach into the docker container by running

```sh
bash run.sh
# or
docker exec --tty --interactive defindex-soroban bash
```

while inside the docker container you can deploy build and test the smart contracts

to build while being inside the container simply run `make build`

## What's inside?

This Turborepo includes the following packages/apps:

### Apps and Packages

- `docs`: a [Next.js](https://nextjs.org/) app
- `web`: another [Next.js](https://nextjs.org/) app
- `@repo/ui`: a stub React component library shared by both `web` and `docs` applications
- `@repo/eslint-config`: `eslint` configurations (includes `eslint-config-next` and `eslint-config-prettier`)
- `@repo/typescript-config`: `tsconfig.json`s used throughout the monorepo

Each package/app is 100% [TypeScript](https://www.typescriptlang.org/).

### Utilities

This Turborepo has some additional tools already setup for you:

- [TypeScript](https://www.typescriptlang.org/) for static type checking
- [ESLint](https://eslint.org/) for code linting
- [Prettier](https://prettier.io) for code formatting

### Build

To build all apps and packages, run the following command:

```
cd my-turborepo
pnpm build
```
