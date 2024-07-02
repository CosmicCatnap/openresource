CREATE TABLE platform (
	id serial PRIMARY KEY, 
	name varchar (20) NOT NULL, 
	description varchar (30),
);

CREATE TABLE project (
	id serial PRIMARY KEY, 
	name varchar (20) NOT NULL, 
	description varchar (150), 
	due_date date
);

CREATE TABLE task (
	id serial PRIMARY KEY, 
	name varchar (30) NOT NULL, 
	description varchar (150), 
	due_date date
);

CREATE TABLE issue (
	id serial PRIMARY KEY, 
	name varchar (20) NOT NULL,
    type varchar (15), 
    status varchar (30), 
    priority varchar (20),
    description varchar (150), 
    due_date date
);

CREATE TABLE support (
	project_id int REFERENCES project(id), 
	issue_id int REFERENCES issue(id), 
	task_id int REFERENCES task(id)
);