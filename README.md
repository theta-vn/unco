# TEST

## DATABASE
- run 
`surreal start --auth --user root --pass root --bind 127.0.0.1:8000 file:unco.db`

## TEST UNCO

cargo test --test schema  -- test_create --nocapture

cargo test --test relate  -- test_create --nocapture

cargo test --test conn  -- test_conn --nocapture

## EXPAND UNCO
cargo expand --test schema


