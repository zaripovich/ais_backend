-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role INT NOT NULL,
    FOREIGN KEY(role) REFERENCES roles(id)
);