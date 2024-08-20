/* metadata for openresource */
CREATE TABLE openresource (
    id serial PRIMARY KEY,
    name varchar(60),
    type varchar(60),
    version varchar(20),
    backend_url varchar(120),
    frontend_url varchar(120),
    status varchar(20),
);

CREATE TABLE openresource_log (
    id serial PRIMARY KEY,
    event_name varchar(60),
    event_type varchar(60),
    event_time timestamp,
    message JSONB,
    status varchar(20),
);