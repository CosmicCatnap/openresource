CREATE TABLE report (
	id serial PRIMARY KEY, 
	title varchar(250), 
	description text, 
	date_created date DEFAULT NOW()
);

CREATE TABLE report_template (
	id serial PRIMARY KEY,
    title varchar(100),
    description text,
    date_created date DEFAULT NOW(),
    date_updated date,
    data json
);
