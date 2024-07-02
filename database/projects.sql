CREATE TABLE platform (
	id serial PRIMARY KEY, 
	name VARCHAR(20) NOT NULL, 
	description VARCHAR(30),
);

CREATE TABLE task (
	id serial PRIMARY KEY, 
	name VARCHAR(30) NOT NULL, 
	description VARCHAR(150), 
	due_date date
);

CREATE TABLE support (
	project_id int REFERENCES project(id), 
	issue_id int REFERENCES issue(id), 
	task_id int REFERENCES task(id)
);

CREATE TABLE project (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
	platform_id int REFERENCES platform(id),
    description TEXT,
    start_date DATE,
    end_date DATE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE issue_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    icon_url VARCHAR(255)
);

CREATE TABLE issue (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id),
    issue_type_id INTEGER NOT NULL REFERENCES issue_types(id),
    summary VARCHAR(255) NOT NULL,
    description TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    status VARCHAR(255) NOT NULL CHECK(status IN ('OPEN', 'IN_PROGRESS', 'RESOLVED', 'CLOSED')),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE label (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    color VARCHAR(7)
);

CREATE TABLE component (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id),
    name VARCHAR(255) NOT NULL,
    description TEXT
);

CREATE TABLE attachment (
    id SERIAL PRIMARY KEY,
    issue_id INTEGER NOT NULL REFERENCES issues(id),
    file_path VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);