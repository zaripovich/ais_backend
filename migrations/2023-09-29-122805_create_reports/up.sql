-- Your SQL goes here
CREATE TABLE reports(
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    role INT NOT NULL,
    FOREIGN KEY(role) REFERENCES roles(id)
);
