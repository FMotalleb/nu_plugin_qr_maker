# nu_plugin_qr_maker

A [nushell](https://www.nushell.sh/) plugin to create qr code in terminal

## Examples

```bash
~> "https://google.com" | to qr
```

![image](https://github.com/FMotalleb/nu_plugin_qr_maker/assets/30149519/1771961a-b06b-4310-81ed-63865e8d2f8e)

## Installing

* using [nupm](https://github.com/nushell/nupm)

```bash
git clone https://github.com/FMotalleb/nu_plugin_qr_maker.git
nupm install --path nu_plugin_qr_maker -f
```

* or compile manually

```bash
git clone https://github.com/FMotalleb/nu_plugin_qr_maker.git
cd nu_plugin_qr_maker
cargo build
register target/debug/nu_plugin_qr_maker
```

* or using cargo

```bash
cargo install nu_plugin_qr_maker
register  ~/.cargo/bin/nu_plugin_qr_maker
```
