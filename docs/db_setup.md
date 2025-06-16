# How to set up a PostgreSQL database for testing

```
docker run --rm -it --name biocrs_test \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=bio1rs \
  -e POSTGRES_DB=bioc \
  -p 5432:5432 postgres
```



# set env file:

```bash
echo DATABASE_URL=postgres://postgres:bio1rs@localhost/bioc > .env

```