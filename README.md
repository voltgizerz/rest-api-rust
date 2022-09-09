# REST API Rust
Simple REST APIs.
# BUILD WITH
- Rocket

# HOW TO RUN PROJECT ?
- Import database pokemon.sql at folder config
- run this project using <b>cargo run</b>

# GET
Routes:
- (hello) GET /
- (ping) GET /ping
- (loops) GET /loops
- (user) GET /user/<name>/<age>
- (hello_param) GET /hello/<name>/<age>

- Response hello_param:

```json 
{
    "active": true,
    "username": "burhan",
    "email": "burhan@gggmail.com",
    "age": 12,
    "duration": "Millis: 0 ms"
}
```