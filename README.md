# TokenGator Program Library

> The Easiest Way to Manage Dynamic NFT Collections on Solana.

## Repositories

TokenGator is currently split into two repositories, one for the platform (API/SDK/Web UI) and one for the Anchor program.

- [TokenGator Platform](https://github.com/pubkeyapp/tokengator)
- [TokenGator Program Library](https://github.com/pubkeyapp/tokengator-program-library) (this repository)

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/en/) (v20 or higher)
- [PNPM](https://pnpm.io/) (v8 or higher)
- [Git](https://git-scm.com/)

### Installation

1. Clone the repository:

```sh
git clone https://github.com/pubkeyapp/tokengator-starter my-app
cd my-app
pnpm install
```

### Development

Start the API app:

```shell
pnpm dev:api
```

Start the web app:

```sh
pnpm dev:web
```

### Build

Build the API app:

```sh
pnpm build:api
```

Build the web app:

```sh
pnpm build:wev
```

### Lint

```sh

pnpm lint
```

### Test

```sh
pnpm test
```

## Add Anchor

This project is compatible with the generators from [create-solana-dapp](https://npm.im/create-solana-dapp).

You can use it to generate an Anchor application:

```shell
pnpm add -D @solana-developers/preset-anchor
pnpm nx generate @solana-developers/preset-anchor:application anchor --dry-run
```

With this base set up, you can now add Anchor programs and tests to your project.

```shell
pnpm nx generate @solana-developers/preset-anchor:template --projectName anchor --directory anchor --template counter counter --dry-run
```

## License

MIT
