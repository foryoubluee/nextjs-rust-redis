# Next.js + Tailwind CSS Example

This example shows how to use [Tailwind CSS](https://tailwindcss.com/) [(v3.2)](https://tailwindcss.com/blog/tailwindcss-v3-2) with Next.js. It follows the steps outlined in the official [Tailwind docs](https://tailwindcss.com/docs/guides/nextjs).

## Deploy your own

Deploy the example using [Vercel](https://vercel.com?utm_source=github&utm_medium=readme&utm_campaign=next-example) or preview live with [StackBlitz](https://stackblitz.com/github/vercel/next.js/tree/canary/examples/with-tailwindcss)

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/git/external?repository-url=https://github.com/vercel/next.js/tree/canary/examples/with-tailwindcss&project-name=with-tailwindcss&repository-name=with-tailwindcss)

## How to use

Execute [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app) with [npm](https://docs.npmjs.com/cli/init), [Yarn](https://yarnpkg.com/lang/en/docs/cli/create/), or [pnpm](https://pnpm.io) to bootstrap the example:

```bash
npx create-next-app --example with-tailwindcss with-tailwindcss-app
```

```bash
yarn create next-app --example with-tailwindcss with-tailwindcss-app
```

```bash
pnpm create next-app --example with-tailwindcss with-tailwindcss-app
```

Deploy it to the cloud with [Vercel](https://vercel.com/new?utm_source=github&utm_medium=readme&utm_campaign=next-example) ([Documentation](https://nextjs.org/docs/deployment)).

# REDIS CONFIGURATION

Make sure you have Redis installed on your machine

## [If anybody doesn't have RedisJSON installed]

Step 1 : brew services stop redis

Step 2 : Go to https://github.com/RedisJSON/RedisJSON

Step 3 : Choose a folder to download & Install the following piece of code

Step 4 : Clone The RedisJSON Repository
```bash git clone https://github.com/RedisJSON/RedisJSON.git ```

Step 5 : Release it
```bash cargo build --release (Make sure you have rust installed or run this command - curl https://sh.rustup.rs -sSf | sh )```

Step 6 : Start Redis server with --loadmodule args
```bash redis-server --loadmodule ./your_release_directory_from_installed_RedisJSON/librejson.dylib```

## Example Redis JSON.SET command

```bash
redis-cli
```

```bash
JSON.SET movie $ '[{ "title": "Hello world", "description": "blablablablalbalbalb", "ratings": 8.0 },{ "title": "Toy story", "description": "toy story anjay mabar broh", "ratings": 8.9 },{ "title": "Alladin", "description": "Alladin arap apa india sih dia?", "ratings": 8.7 }]'
```
