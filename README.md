# Rivet CLI

## Installing

### Docker

#### macOS & Linux & Windows (PowerShell)

```
docker run -v "$(pwd):/app" ghcr.io/rivet-gg/cli
```

#### Windows (Command Line)

```
docker run -v "%cd%:/app" ghcr.io/rivet-gg/cli
```

### Binary (Linux only)

```
curl https://github.com/rivet-gg/cli/releases/download/0.0.13/rivet_0.0.13_linux_x86_64.tar.gz -L -o rivet.tar.gz
tar xf rivet.tar.gz
mv ./rivet /usr/local/bin/rivet
```

