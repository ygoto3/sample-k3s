#!/bin/sh

DATABASE_URL=postgres://root:devpassword@127.0.0.1/db sea-orm-cli generate entity -o ./src/entity
