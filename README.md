# Rivet CLI

## Installing

### macOS & Linux

```
curl -fsSL https://raw.githubusercontent.com/rivet-gg/cli/main/install/unix.sh | sh
curl -fsSL https://github.com/rivet-gg/cli/raw/dc673a469ddfad222dbd60a96648312f05701e8b/install/unix.sh | sh
```

### Docker

> Commands such as `rivet build push` will not work when running using Docker.

#### macOS & Linux & Windows (PowerShell)

```
docker run -v "$(pwd):/app" ghcr.io/rivet-gg/cli
```

#### Windows (cmd)

```
docker run -v "%cd%:/app" ghcr.io/rivet-gg/cli
```

