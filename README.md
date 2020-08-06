# dartbot

[![Build Status](https://travis-ci.org/whentze/dartbot.svg?branch=master)](https://travis-ci.org/whentze/dartbot)
[![Coverage Status](https://coveralls.io/repos/github/whentze/dartbot/badge.svg?branch=master)](https://coveralls.io/github/whentze/dartbot?branch=master)


Ein Telegram-Bot der erfolgreiches Dartspiel mit 1 Woche Auszeit belohnt

## Wie kann ich ihn benutzen?

1. `export BOT_TOKEN=123456789:abcdefgh`
2. `cargo run`

oder 

1. `docker build -t dartbot:latest .`
2. `docker run -e BOT_TOKEN=123456789:abcdefgh dartbot:latest`

oder (ohne Docker build)

1. `docker run -e BOT_TOKEN=123456789:abcdefgh docker.registry.github.com/whentze/dartbot/dartbot:latest`

