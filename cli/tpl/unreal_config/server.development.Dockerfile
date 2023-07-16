# Used for testing the game server locally.
#
# The Rivet CLI will mount the project to `/project`.

FROM ghcr.io/epicgames/unreal-engine:dev-5.2.1

CMD echo '=== Building ===' && \
	/home/ue4/UnrealEngine/Engine/Build/BatchFiles/RunUAT.sh BuildCookRun \
		-Server -NoClient -ServerConfig=Development \
		-Project=/project/__UPROJECT_PATH__ \
		-UTF8Output -NoDebugInfo -AllMaps -NoP4 -Build -Cook -Stage -Pak -Package -Archive \
		-ArchiveDirectory=/project/Build/LinuxServer/Packaged \
		-Platform=Linux \
	&& \
	echo '=== Running ===' \
	/project/Build/LinuxServer/Packaged/__GAME_MODULE__.sh -server -log
