# GraphQL Sample Applications

The repository contains two sample applications that expose part of the Sakila
database via rich GraphQL API.

## Requirements

A local copy of the [Sakila dataset](https://github.com/jOOQ/sakila.git):

```
git clone https://github.com/jOOQ/sakila.git
```

Either Podman or Docker or:

* Local PostgreSQL
* Rust toolchain
* Golang toolchain

## Database setup

```bash
### Postgres container setup
export POSTGRES_USER=postgres
export POSTGRES_PASSWORD=password
export POSTGRES_DB=postgres
export POSTGRES_PORT=5432

### Run the PostgresQL container
podman run \
    --rm \
    --name postgres_sakila \
    -e POSTGRES_USER=$POSTGRES_USER \
    -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
    -e POSTGRES_DB=$POSTGRES_DB \
    -p $POSTGRES_PORT:5432 \
    -d \
    postgres \
    -N 100
    
### Create the DB schema
cat postgres-sakila-schema.sql | psql \
    -U $POSTGRES_USER \
    -d $POSTGRES_DB \
    --host 127.0.0.1 \
    --port $POSTGRES_PORT

### Import the dataset
cat postgres-sakila-insert-data-using-copy.sql | psql \
    -U $POSTGRES_USER \
    -d $POSTGRES_DB \
    --host 127.0.0.1 \
    --port $POSTGRES_PORT
```

## Application configuration

Both the Rust and the Golang application share the same configuration and
configuration options

### HTTP server configuration

```text
+------------------------------+----------+----------------------------------------------------------------------------+
|            OPTION            | DEFAULT  |                                  MEANING                                   |
+------------------------------+----------+----------------------------------------------------------------------------+
| CFG__SERVER__PORT            | 8080     | On which port the application accepts HTTP connections                     |
| CFG__SERVER__REQUEST_TIMEOUT | 10_000ms | The requests will fails with a timeout after the configured amount of time |
+------------------------------+----------+----------------------------------------------------------------------------+
```

### Database connection configuration

```text
+-------------------+-----------+----------------------------------------------------+
|      OPTION       |  DEFAULT  |                      MEANING                       |
+-------------------+-----------+----------------------------------------------------+
| CFG__DB__DB_NAME  | postgres  | Name of the database containing the Sakila dataset |
| CFG__DB__USER     | postgres  | Name of the DB user used for connecting to the DB  |
| CFG__DB__PASS     | password  | Password for the DB user for connecting to the DB  |
| CFG__DB__HOST     | 127.0.0.1 | Host address of the PG database                    |
| CFG__DB__PORT     | 5432      | Port of the PG database                            |
| CFG__DB__MAX_CONN | 16        | Maximum allowed number of connections to the DB    |
+-------------------+-----------+----------------------------------------------------+
```

### Data-loader configuration

```text
+------------------------------------+---------+------------------------------------------------------+
|               OPTION               | DEFAULT |                       MEANING                        |
+------------------------------------+---------+------------------------------------------------------+
| CFG__DATA_LOADER__DEFAULT_DELAY_MS |      10 | How much time to wait before sending a batch request |
| CFG__DATA_LOADER__MAX_BATCH_SIZE   |     100 | Maximum size of the batch requests                   |
+------------------------------------+---------+------------------------------------------------------+
```

## Building the applications

The easiest way to build the applications is to use the provided `Containerfile`:

```bash
cd ./graphql-go
podman build . -t graphql-go
```

```bash
cd ./graphql-rust
podman build . -t graphql-rust
```

## Exploring the application

Both applications provide a GraphQL playground to try and test various GQL
queries. It can be accessed at `http://host:port/playground`

### Example requests

#### Get all actors that have played in `Horror` movies

```graphql
query{
  actors(filter:{film:{category:{nameEq:"Horror"}}}){
    firstName
    lastName
  }
}
```

#### Get all information for all films that are between `(45; 50)` minutes long

```graphql
query {
  films(filter:{lengthGt:45, lengthLt:50}){
    title
    description
    categories{
      name
    }
    language{
      name
    }
    actors{
      firstName
      lastName
    }
  }
}
```

#### Get the details of all movies for a specific actor

```graphql
query {
  films(filter: { actor: { firstNameEq: "TOM", lastNameEq: "MCKELLEN" } }) {
    title
    description
    length
    categories {
      name
    }
    language {
      name
    }
  }
}
```

OR

```graphql
query {
  actors(filter: { firstNameEq: "TOM", lastNameEq: "MCKELLEN" }) {
    films {
      title
      length
      description
      categories {
        name
      }
      language {
        name
      }
    }
  }
}
```
