CREATE TABLE contact ( 
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(60),
    last_name VARCHAR(60),
    organization_name VARCHAR(60),
    title VARCHAR(60),
    phone VARCHAR(15),
    email VARCHAR(60),
    address VARCHAR(120),
    city VARCHAR(60),
    region VARCHAR(60),
    postal VARCHAR(15),
    country VARCHAR(60)
);

CREATE TABLE customer ( 
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(60),
    last_name VARCHAR(60),
    organization_name VARCHAR(60),
    contact_id INTEGER REFERENCES contact(id)
);

CREATE TABLE product (
    id SERIAL PRIMARY KEY,
    name VARCHAR(60),
    type VARCHAR(30),
    catagory VARCHAR(30),
    description TEXT
);

CREATE TABLE location (
    id SERIAL PRIMARY KEY,
    name VARCHAR(60),
    region VARCHAR(30),
    type VARCHAR(20)
);

CREATE TABLE inventory (
    id SERIAL PRIMARY KEY,
    product_id INTEGER REFERENCES product(id),
    location_id INTEGER REFERENCES location(id),
    quantity INTEGER,
    price DECIMAL(15,2)
);

CREATE TABLE transaction (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES customer(id),
    product_id INTEGER REFERENCES product(id),
    price DECIMAL(15,2)
);

CREATE TABLE organization (
    id SERIAL PRIMARY KEY,
    name VARCHAR(60),
    description TEXT,
    catagory VARCHAR(60)
);

CREATE TABLE business_unit (
    id SERIAL PRIMARY KEY,
    name VARCHAR(60),
    description TEXT
);

CREATE TABLE employee (
    id SERIAL PRIMARY KEY,
    firstname VARCHAR(60),
    lastname VARCHAR(60),
    title VARCHAR(30),
    birthdate DATE,
    hiredate DATE,
    business_unit INTEGER REFERENCES business_unit(id),
    address VARCHAR(60),
    phone VARCHAR(15),
    email VARCHAR(60),
    city VARCHAR(30),
    state VARCHAR(30),
    zipcode VARCHAR(15),
    country VARCHAR(30)
);

CREATE TABLE payrole (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER REFERENCES employee(id),
    business_unit_id INTEGER REFERENCES business_unit(id),
    location_id INTEGER REFERENCES location(id),
    type VARCHAR(60),
    status VARCHAR(30)
);

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES customer(id),
    product_id INTEGER REFERENCES product(id),
    quantity INTEGER,
    price DECIMAL(10,2),
    status VARCHAR(30),
    order_date DATE
);

CREATE TABLE shipment  (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id),
    address VARCHAR(60),
    shipmnet_date DATE,
    arrival_date DATE,
    city VARCHAR(30),
    region VARCHAR(30),
    postal VARCHAR(15),
    country VARCHAR(30)
);