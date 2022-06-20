#!/bin/sh

set -e

platform="$(uname -sm)"

echo_error() {
	echo >&2 "$1 is not supported by the install script, \
see https://figmaid.nykanen.me/install.html on how to build from source"
}

if [ "$OS" = "Windows_NT" ]; then
	echo_error "$OS" && exit 1
fi

case $platform in
"Darwin x86_64") target="macos-amd64" ;;
"Darwin arm64") target="macos-aarch64" ;;
"Linux x86_64") target="ubuntu-amd64" ;;
*) echo_error "$platform" && exit 1 ;;
esac

dirname="$HOME/.local/bin"
exe=$dirname/figmaid
figmaid_uri=https://github.com/jqpe/figmaid/releases/latest/download/figmaid-${target}

if [ ! -d "$dirname" ]; then
	mkdir -p "$dirname"
fi

curl --fail --location --progress-bar --output "$exe" "$figmaid_uri"
chmod +x "$exe"

echo "figmaid was installed successfully to $exe"
if command -v figmaid >/dev/null; then
	echo "Run 'figmaid' to start the server or 'figmaid --help' to list all options"
else
	case $SHELL in
	/bin/zsh) shell_profile=".zshrc" ;;
	*) shell_profile=".bash_profile" ;;
	esac
	echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
	echo "  export PATH=\"$dirname:\$PATH\""
fi
