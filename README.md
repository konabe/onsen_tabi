# onsen_tabi

## what's this

温泉を起点とした旅の記録をしてみたいと思ったので作りました。

## システム構成

vercel <-> render.com WEB SERVICE <-> MySQL [AWS]

// TODO: AWSに移行

- [frontend source](https://github.com/konabe/onsen-tabi-web)

## マイグレーション

```
diesel migration generate {name}
diesel migration run
```