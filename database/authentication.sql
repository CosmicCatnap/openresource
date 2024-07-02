CREATE TABLE role (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    name varchar(128),
    privileges json
);

CREATE TABLE user (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    username varchar(32),
    password varchar(50),
    role references user_role(id),
    email varchar(64),
    first_name varchar(32),
    last_name varchar(32),
    metadata json
);

CREATE TABLE user_role (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    user_id int,
    role_id int
);
