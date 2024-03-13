# onsen_tabi
[![codecov](https://codecov.io/gh/konabe/onsen_tabi/graph/badge.svg?token=WRRRJTB2BE)](https://codecov.io/gh/konabe/onsen_tabi)

## what's this

温泉を起点とした旅の記録をしてみたいと思ったので作りました。

## システム構成

vercel <-> Route53 <-> ALB <-> EC2 <-> RDS for MySQL

- [frontend source](https://github.com/konabe/onsen-tabi-web)

## マイグレーション

```
diesel migration generate {name}
diesel migration run
```

## Docker

```
docker build --no-cache --tag app-hello-rocket:latest .
docker run --rm --env DATABASE_URL="" --env JWT_SECRET_KEY="" --publish 8000:8000 --name app-local app-hello-rocket:latest

```

## テストカバレッジ

![sunburst](https://codecov.io/gh/konabe/onsen_tabi/graphs/sunburst.svg?token=WRRRJTB2BE)