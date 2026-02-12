FROM nixos/nix:latest

RUN nix-channel --add https://nixos.org/channels/nixos-unstable nixpkgs && \
  nix-channel --update

RUN mkdir -p /etc/nix && \
  echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

WORKDIR /app

COPY flake.nix flake.lock /app/

ENV DISPLAY=:99
ENV SCREEN_RESOLUTION=640x480x24
ENV WINEPREFIX=/app/.wine
ENV WINEARCH=win64
ENV WINEDLLOVERRIDES="mscoree,mshtml="
ENV WINEDEBUG=-all

RUN nix develop --command bash -c "wine wineboot --init && wineserver -w"

RUN nix profile install nixpkgs#xorg.xorgserver nixpkgs#x11vnc nixpkgs#fluxbox nixpkgs#xorg.xdpyinfo

COPY protossbot /app/protossbot

RUN nix develop --command bash -c "cd protossbot && cargo build --target x86_64-pc-windows-gnu --release"

COPY starcraft /app/starcraft


RUN cat <<'EOF' > /start-bot.sh
#!/usr/bin/env bash

rm -f /tmp/.X99-lock /tmp/.X11-unix/X99

echo "Starting Xvfb virtual display..."
Xvfb :99 -screen 0 $SCREEN_RESOLUTION &
XVFB_PID=$!

for i in {1..10}; do
  if xdpyinfo -display :99 >/dev/null 2>&1; then
    echo "X server is ready"
    break
  fi
  echo "Waiting for X server... ($i/10)"
  sleep 1
done

echo "Starting Fluxbox window manager..."
fluxbox &

echo "Starting VNC server on port 5900..."
x11vnc -nopw -display :99 -forever -shared -rfbport 5900 &

cd /app

export DISPLAY=:99

exec nix develop --command bash -c '
  export WINEPREFIX=/app/.wine
  export WINEARCH=win64
  export WINEDLLOVERRIDES="mscoree,mshtml="
  export WINEDEBUG=-all
  export DISPLAY=:99

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
