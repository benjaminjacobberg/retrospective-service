# retrospective-service

## Summary

TODO

## Run

### Start Infrastructure

- Navigate to the compose file: `cd ./deployment/`
- Start up the container: `docker-compose up -d`

### Start Web Service

- cargo run

### Login

Try out the endpoint through the API gateway:

- Navigate your browser to http://localhost:9080/api/v1/
- Login
  - username: `user`
  - password: `password`
- You should then be redirected and see the email address in text `user@example.com`

Try hitting the endpoint directly:

- Navigate your browser to http://localhost:8000/api/v1/
- You should then see the text `no info`
