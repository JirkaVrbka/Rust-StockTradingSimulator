FROM ubuntu:21.04

RUN apt update
RUN apt install -y libpq-dev

COPY ./target/release /server

ENV DATABASE_URL=postgres://rust:KFQLs8a8Vs4Bs9s@pv287-stockss.postgres.database.azure.com/postgres?sslmode=require
EXPOSE 8081

CMD ["/server/server"] 
