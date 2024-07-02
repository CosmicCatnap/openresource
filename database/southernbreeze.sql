
/* metadata for openresource */
CREATE TABLE openresource (
    id serial PRIMARY KEY,
    name varchar(100),
    type varchar(100),
    version varchar(100),
    backend_url varchar(100),
    frontend_url varchar(100),
    status varchar(10),
);
/* Sales */
create table customer ( 
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    individual bool,
    first_name varchar(255),
    last_name varchar(255),
    organization_name varchar(255),
    contact_id references contact (id),
    address varchar(255),
    city varchar(60),
    region varchar(60),
    postal varchar(15),
    country varchar(15)
);

create table contact ( 
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    first_name varchar(255),
    last_name varchar(255),
    organization_name varchar(255),
    title varchar(255),
    phone varchar(15),
    email varchar(100),
    address varchar(255),
    city varchar(60),
    region varchar(60),
    postal varchar(15),
    country varchar(15)
);

CREATE TABLE product (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    name varchar(255),
    description text,
    catagory varchar(60),
    metadata json
);

CREATE TABLE inventory (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    product_id int references product(id),
    location varchar(255),
    quantity int,
    price decimal(15,2)
);

CREATE TABLE sale (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    customer_id int references customer(id)
);

CREATE TABLE sale_product (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    sale_id int references sale(id),
    product_id int references product(id)
);

CREATE TABLE sale_log (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    sale_id int references sale(id),
    date timestamp
);

CREATE TABLE transaction (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    customer_id int references customer(id),
    product_id int references product(id),
    price decimal(15,2)
);

CREATE TABLE transaction_log (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    transaction_id int references transaction(id)
);

/*
Buisness Structure
*/
CREATE TABLE organization (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    name varchar(255),
    description text,
    catagory varchar(60),
    metadata json
);

CREATE TABLE employee (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    firstname varchar(60),
    lastname varchar(60),
    title varchar(30),
    birthdate date,
    hiredate date,
    department int references department(id),
    organizational_unit int references buisness_unit(id),
    address varchar(255),
    phone varchar(15),
    email varchar(255),
    city varchar(30),
    state varchar(30),
    zipcode varchar(15),
    country varchar(30),
    metadata json
);

CREATE TABLE buisness_unit (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    name varchar(60),
    description text,
    metadata json
);

CREATE TABLE payrole (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    employee_id int references employee(id),
    buisness_unit_id int references buisness_unit(id),
    type varchar(60),
    status varchar(30)
);

CREATE TABLE order (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    customer_id int references customer(id),
    product_id int references product(id),
    shipment_id int references shipment(id),
    quantity int,
    price decimal(10,2),
    status varchar(30),
    order_date date
);

CREATE TABLE shipment  (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    order_id int references order(id),
    customer_id int references customer(id),
    address varchar(255),
    shipmnet_date date,
    arrival_date date,
    city varchar(30),
    region varchar(30),
    postal varchar(15),
    country varchar(30)
);