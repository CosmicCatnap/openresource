FROM postgres:bookworm as build

RUN apt-get update && apt-get install postgresql-contrib
COPY ./dbinit.sh /docker-entrypoint-initdb.d/init-user-db.sh
RUN chmod +x /docker-entrypoint-initdb.d/init-user-db.sh

FROM postgres:bookworm as final