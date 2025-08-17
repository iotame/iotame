For now (until proper environment management is implemented):

- Start Postgres with TimescaleDB enabled:  
  `docker run -d --name iotame -p 5432:5432 -e POSTGRES_PASSWORD=iotame timescale/timescaledb-ha:pg17`
- Connect via CLI:  
  `psql -d "postgres://postgres:iotame@localhost/postgres"`
