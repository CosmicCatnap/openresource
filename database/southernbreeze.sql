
/* metadata for openresource */
CREATE TABLE openresource (
    id serial PRIMARY KEY,
    name varchar(100),
    type varchar(100),
    version varchar(100),
    url varchar(100),
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
    address varchar(255),
    city varchar(60),
    state varchar(60),
    zip varchar(15)
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
    firstname varchar(60)
);

CREATE TABLE buisness_unit (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    name varchar(60)
);

CREATE TABLE payrole (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4()
);
/*
Finance
*/

CREATE TABLE invoice (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4()
);

CREATE TABLE contract (
    id serial PRIMARY KEY,
    uid uuid DEFAULT uuid_generate_v4(),
    name varchar(250),
    description text
);