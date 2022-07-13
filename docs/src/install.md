# Install

figmaid currently only supports Linux. Previous versions included precompiled binaries for macOS (arm64 and x64) as well as Windows. For the documentation of the last macOS/Windows version switch to [tag v0.9.0](https://github.com/jqpe/figmaid/tree/v0.9.0/docs). If you want to use custom directories and thus need figmaid on macOS/Windows, you can still use the Docker image and continue to get updates. 

There's three ways you can install figmaid
1. With the installation script on Linux
   
   ```sh
   curl -fsSL https://raw.githubusercontent.com/jqpe/figmaid/main/install.sh | sh
   ```

2. Building from source
3. Using the Docker image, documented in [figmaid in docker](./use/docker-image.md)

> You can use figmaid on Windows/macOS via Docker, but you're probably better off using [FigmaAgent](https://help.figma.com/hc/en-us/articles/360039956894-Access-local-fonts-on-your-computer#browser) unless you need custom directories.
> If you only intend on using installed fonts, FigmaAgent will pick those up.

To build from source [install Rust](https://www.rust-lang.org/tools/install). For a specific tag you can use the `--tag` argument, e.g. for Windows and macOS you're limited to <= v0.9.0.

```sh
cargo install --git https://github.com/jqpe/figmaid
```
