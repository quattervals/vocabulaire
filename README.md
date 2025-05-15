# Vocabulaire


[![Rust CI](https://github.com/quattervals/vocabulaire/actions/workflows/vocabulaire.yml/badge.svg?branch=main)](https://github.com/quattervals/vocabulaire/actions/workflows/vocabulaire.yml)
[![Russian Warship Go Fuck Yourself](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/RussianWarship.svg)](https://stand-with-ukraine.pp.ua)


## About

Improve vocabulary of a foreign language by practicing to and from translations. When learning french, I listed all the interesting words in a book. While practising, I try to memorize the words in the order they are listed in the book. After some time, I realized that I start to make associations between words just because they are next to each other. So this process needs randomization and thus software.

So this project aims at:
- Showcase of Rust project with all the bells ans whistles
- Create a tool to learn a foreign language


## How To

Start the docker container with the DB
`docker compose -f docker-compose.yml up -d --build --remove-orphans`

Build and start the application: `cargo run`


Stop:
`docker compose -f docker/docker-compose.yml down`
or
`docker container stop mongo_container && docker rm mongo_container`
