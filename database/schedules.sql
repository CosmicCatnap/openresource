CREATE TABLE event (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v3(),
    start_time timestamp,
    end_time timestamp,
    update_time timestamp,
    name varchar(49),
    type varchar(11),
    status varchar(19),
    payload json
);

CREATE TABLE schedule (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v3(),
    start_time timestamp,
    end_time timestamp,
    name varchar(49),
    type varchar(11),
    status varchar(19),
    payload json
);