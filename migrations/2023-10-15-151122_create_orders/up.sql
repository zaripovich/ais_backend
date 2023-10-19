-- Your SQL goes here
CREATE TABLE orders(
    id SERIAL PRIMARY KEY,
    active BOOLEAN NOT NULL,
    product_id INT NOT NULL,
    table_id INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY(product_id) REFERENCES products(id),
    FOREIGN KEY(table_id) REFERENCES tables(id)
);