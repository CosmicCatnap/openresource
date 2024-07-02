-- performs batch processing, scheduled actions, and event handeling
CREATE TABLE worker (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    priority INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    enabled BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE worker_status (
    id SERIAL PRIMARY KEY,
    worker_id INTEGER NOT NULL REFERENCES jobs(id),
    status VARCHAR(255) NOT NULL CHECK(status IN ('QUEUED', 'RUNNING', 'COMPLETED', 'FAILED')),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create the queues table
CREATE TABLE worker_queue (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE worker_log (
    id SERIAL PRIMARY KEY,
    worker_id INTEGER NOT NULL REFERENCES jobs(id),
    message TEXT
);
