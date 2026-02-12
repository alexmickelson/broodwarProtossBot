FROM nixos/nix:latest

RUN nix-channel --add https://nixos.org/channels/nixos-unstable nixpkgs && \
  nix-channel --update

RUN mkdir -p /etc/nix && \
  echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

WORKDIR /app

COPY flake.nix flake.lock /app/

ENV WINEPREFIX=/app/.wine
ENV WINEARCH=win64
ENV WINEDLLOVERRIDES="mscoree,mshtml="
ENV WINEDEBUG=-all

RUN nix develop --command bash -c "wine wineboot --init && wineserver -w"

RUN nix profile install nixpkgs#xorg.xorgserver nixpkgs#x11vnc nixpkgs#fluxbox nixpkgs#xorg.xdpyinfo nixpkgs#socat

COPY scripts /app/scripts
COPY starcraft /app/starcraft

COPY protossbot /app/protossbot

RUN nix develop --command bash -c "cd protossbot && cargo build --target x86_64-pc-windows-gnu --release"

ENV DISPLAY=:99
ENV SCREEN_RESOLUTION=640x480x24
ENV VNC_PORT=5900
ENV X_DISPLAY_NUMBER=99
ENV WEBSERVER_PORT=3333

RUN cat <<'EOF' > /start-bot.sh
#!/usr/bin/env bash

# Startup delay for sequential bot initialization
STARTUP_DELAY=${STARTUP_DELAY:-0}
if [ "$STARTUP_DELAY" -gt 0 ]; then
  echo "Waiting ${STARTUP_DELAY} seconds before starting..."
  sleep $STARTUP_DELAY
fi

# Block UDP ports to force StarCraft to use specific port
if [ -n "$UDP_BLOCKED_PORTS" ]; then
  echo "Blocking UDP ports: $UDP_BLOCKED_PORTS"
  for port in $UDP_BLOCKED_PORTS; do
    socat UDP-LISTEN:$port,fork /dev/null &
    echo "Blocked UDP port $port"
  done
  sleep 1
fi

X_DISPLAY_NUMBER=${X_DISPLAY_NUMBER:-99}
export DISPLAY=:${X_DISPLAY_NUMBER}

rm -f /tmp/.X${X_DISPLAY_NUMBER}-lock /tmp/.X11-unix/X${X_DISPLAY_NUMBER}

echo "Starting Xvfb virtual display on :${X_DISPLAY_NUMBER}..."
Xvfb :${X_DISPLAY_NUMBER} -screen 0 $SCREEN_RESOLUTION &
XVFB_PID=$!

for i in {1..10}; do
  if xdpyinfo -display :${X_DISPLAY_NUMBER} >/dev/null 2>&1; then
    echo "X server is ready"
    break
  fi
  echo "Waiting for X server... ($i/10)"
  sleep 1
done

echo "Starting Fluxbox window manager..."
fluxbox &

VNC_PORT=${VNC_PORT:-5900}
echo "Starting VNC server on port $VNC_PORT..."
x11vnc -nopw -display :${X_DISPLAY_NUMBER} -forever -shared -rfbport $VNC_PORT &

cd /app

exec nix develop --command bash -c '
  export WINEPREFIX=/app/.wine
  export WINEARCH=win64
  export WINEDLLOVERRIDES="mscoree,mshtml="
  export WINEDEBUG=-all
  export DISPLAY=:'"${X_DISPLAY_NUMBER}"'

  echo "Configuring Wine registry..."
  
  # Core registry entries
  wine REG ADD "HKEY_LOCAL_MACHINE\\SOFTWARE\\Blizzard Entertainment\\Starcraft" \
      /v InstallPath /t REG_EXPAND_SZ /d "Z:\\app\\starcraft\\" /f 2>/dev/null || true
  
  wine REG ADD "HKEY_LOCAL_MACHINE\\SOFTWARE\\Blizzard Entertainment\\Starcraft" \
      /v Program /t REG_EXPAND_SZ /d "Z:\\app\\starcraft\\StarCraft.exe" /f 2>/dev/null || true
  
  # Disable Intro
  wine REG ADD "HKEY_CURRENT_USER\\Software\\Blizzard Entertainment\\Starcraft" /v "Intro" /t REG_SZ /d "0" /f 2>/dev/null || true
  wine REG ADD "HKEY_CURRENT_USER\\Software\\Blizzard Entertainment\\Starcraft" /v "IntroX" /t REG_SZ /d "0" /f 2>/dev/null || true
  wine REG ADD "HKEY_CURRENT_USER\\Software\\Blizzard Entertainment\\Starcraft" /v "Tip" /t REG_SZ /d "0" /f 2>/dev/null || true
  
  # Chaoslauncher configuration
  wine REG ADD "HKEY_CURRENT_USER\\Software\\Chaoslauncher\\Launcher" /v "ScPath" /t REG_SZ /d "Z:\\app\\starcraft" /f 2>/dev/null || true
  wine REG ADD "HKEY_CURRENT_USER\\Software\\Chaoslauncher\\Launcher" /v "GameVersion" /t REG_SZ /d "Starcraft 1.16.1" /f 2>/dev/null || true
  wine REG ADD "HKEY_CURRENT_USER\\Software\\Chaoslauncher\\Launcher" /v "RunScOnStartup" /t REG_SZ /d "1" /f 2>/dev/null || true
  
  # Enable BWAPI 4.4.0 Injector plugin by default
  wine REG ADD "HKEY_CURRENT_USER\\Software\\Chaoslauncher\\PluginsEnabled" /v "BWAPI 4.4.0 Injector [RELEASE]" /t REG_SZ /d "1" /f 2>/dev/null || true
  # wine REG ADD "HKEY_CURRENT_USER\\Software\\Chaoslauncher\\PluginsEnabled" /v "W-MODE 1.02" /t REG_SZ /d "1" /f 2>/dev/null || true
  
  wineserver -w
  
  echo "Registry configuration complete."

  echo "Starting protossbot..."
  cd /app/protossbot
  RUST_BACKTRACE=1 wine target/x86_64-pc-windows-gnu/release/protossbot.exe &
  BOT_PID=$!
  echo "protossbot started (PID: $BOT_PID)"

  echo "Launching StarCraft with BWAPI..."
  cd /app/starcraft/BWAPI/Chaoslauncher
  wine Chaoslauncher.exe

  echo "StarCraft closed."
'
EOF

RUN chmod +x /start-bot.sh

EXPOSE 5900
EXPOSE 3000
EXPOSE 6112/udp

WORKDIR /app

CMD ["/start-bot.sh"]
