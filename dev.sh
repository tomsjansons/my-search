#!/bin/bash

if [ ! -f "./tailwind" ]; then
  echo "Tailwind CSS binary not found. Downloading"

  curl -L -o "./tailwind" "https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.7/tailwindcss-linux-x64"

  if [ $? -eq 0 ]; then
    chmod +x "./tailwind"
    echo "Tailwind CSS binary downloaded and made executable"
  else
    echo "Error: Failed to download Tailwind CSS binary"
    exit 1
  fi
fi

run_tailwind() {
  echo "Starting Tailwind CSS"
  ./tailwind -i ./src/base.css -o ./src/static/base.css
}

run_my_search() {
  echo "Starting my-search"
  cargo run
}

run() {
  run_tailwind && run_my_search
}

if [ $# -eq 0 ]; then
  echo "Starting watchexec..."
  watchexec --restart --watch src ./dev.sh run
elif [ "$1" = "run" ]; then
  run
else
  echo "Usage: $0 [run]"
  exit 1
fi
