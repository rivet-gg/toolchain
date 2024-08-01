# Used for shipping an optimized game server for production.

FROM ghcr.io/epicgames/unreal-engine:dev-5.2.1 as builder

# Copy source code
COPY --chown=ue4:ue4 . /project

# Clean & build dedicated server for Linux
RUN rm -rf /project/Build /project/Saved /project/Intermediate /project/Binaries && \
	/home/ue4/UnrealEngine/Engine/Build/BatchFiles/RunUAT.sh BuildCookRun \
		-Server -NoClient -ServerConfig=Shipping \
		'-Project=/project/__UPROJECT_PATH__' \
		-NoDebugInfo -UTF8Output -AllMaps -NoP4 -Build -Cook -Stage -Pak -Package -Archive \
		-ArchiveDirectory=/project/Packaged \
		-Platform=Linux

# Copy files from builder
FROM debian:11-slim

# Install expect package for 'unbuffer' utility, `xdg-user-dir` to fix Unreal error
RUN apt-get update && apt-get upgrade -y && apt install -y expect xdg-user-dirs

# Create user
RUN useradd nonroot
USER nonroot

# Copy packaged game server
COPY --from=builder --chown=nonroot:nonroot /project/Packaged/LinuxServer /home/nonroot/project

# Expose the game server port
EXPOSE 7777/udp

# Run the game server
ENTRYPOINT ["unbuffer", "/home/nonroot/project/__GAME_MODULE__/Binaries/Linux/__GAME_MODULE__Server-Linux-Shipping", "__GAME_MODULE__", "-server"]
