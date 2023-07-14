## Starting mysql container
1. Create and start a docker container
`docker-compose up -d`

2. Install sqlx-cli (if not installed)
`cargo install sqlx-cli`

3. Migrate/setup the mysql database
`sqlx migrate run`
