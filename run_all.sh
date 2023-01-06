#/bin/env sh

for i in $(seq -w 1 25); do
  if [ -d "$PWD/day-$i" ]; then
    cargo run --bin="day-$i" --release 2> /dev/null;
    echo "============================="
  fi
done
