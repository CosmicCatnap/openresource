CREATE TABLE resource (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    capacity INTEGER,
    available_from TIMESTAMP,
    available_to TIMESTAMP
);

CREATE TABLE schedule (
    id SERIAL PRIMARY KEY,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    resource_id INTEGER NOT NULL REFERENCES resources(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE event (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    recurring BOOLEAN DEFAULT FALSE,
    recurring_interval INTERVAL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE event_schedule (
    event_id INTEGER NOT NULL REFERENCES events(id),
    schedule_id INTEGER NOT NULL REFERENCES schedules(id),
    PRIMARY KEY (event_id, schedule_id)
);
