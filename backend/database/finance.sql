/* Finance */
CREATE TABLE invoice (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4()
);

CREATE TABLE contract (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    name varchar(250),
    description text
);

CREATE TABLE payment (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4()
);

CREATE TABLE expense (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    name varchar(250)
);

CREATE TABLE income (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    name varchar(250),
    description text
);