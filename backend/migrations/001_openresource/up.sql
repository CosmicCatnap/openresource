/* metadata for openresource */
CREATE TABLE openresource (
    id SERIAL PRIMARY KEY,
    name VARCHAR(60),
    type VARCHAR(60),
    version VARCHAR(20),
    backend_url VARCHAR(120),
    frontend_url VARCHAR(120),
    status VARCHAR(20)
);

CREATE TABLE openresource_log (
    id SERIAL PRIMARY KEY,
    event_name VARCHAR(60),
    event_type VARCHAR(60),
    event_time TIMESTAMP,
    status VARCHAR(20)
);