# Install

There's three ways you can install figmaid
1. With the installation script on supported platforms
2. Building from source
3. Using the Docker image, documented in [figmaid in docker](./use/docker-image.md)

On *nix operating systems you can use the installation script. See the table below for supported architectures.

```sh
curl -fsSL https://raw.githubusercontent.com/jqpe/figmaid/main/install.sh | sh
```

| target                   | binary name               | install script |
| :----------------------- | ------------------------- | -------------- |
| x86_64-apple-darwin      | figmaid-macos-amd64       | ✓              |
| aarch64-apple-darwin     | figmaid-macos-aarch64     | ✓              |
| x86_64-unknown-linux-gnu | figmaid-ubuntu-amd64      | ✓              |
| x86_64-pc-windows-gnu    | figmaid-windows-amd64.exe | ⚠              |

<br/>
<br/>

> Although figmaid supports Windows/macOS you're probably better off using [Figma Agent](https://help.figma.com/hc/en-us/articles/360039956894-Access-local-fonts-on-your-computer#browser) unless you need custom directories.
> If you only intend on using installed fonts, Figma Agent will pick those up.

If you are using Windows or an unsupported architecture you need to [install Rust](https://www.rust-lang.org/tools/install) and build from source.

```sh
cargo install --git https://github.com/jqpe/figmaid
```

---

After the installation is complete you have `figmaid` ready to go!