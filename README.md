# TEST
cargo test --test schema  -- test_create --nocapture
cargo expand --test schema  -- test_schema

## DATABASE

- example
`surreal start --log trace --user root --pass root --bind 0.0.0.0:8080 file://path/to/mydatabase`

- run 
`surreal start --auth --user root --pass root --bind 127.0.0.1:8000 file:unco.db`

- cli
`surreal sql --endpoint http://localhost:8000 --username root --password root --namespace test --database test --pretty`