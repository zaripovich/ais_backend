-- Your SQL goes here
CREATE TABLE products(
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL UNIQUE,
    price INT NOT NULL
);