# Rivet CLI

## Installing

### macOS & Linux

```
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/rivet-gg/cli/main/install/unix.sh | sh
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
