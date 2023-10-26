-- Your SQL goes here
CREATE TABLE products(
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL UNIQUE,
    price INT NOT NULL
);


INSERT INTO products VALUES(DEFAULT,'Пиво',250);
INSERT INTO products VALUES(DEFAULT,'Гренки',100);
INSERT INTO products VALUES(DEFAULT,'Колбаса баварская',500);
INSERT INTO products VALUES(DEFAULT,'Шашлык',1200);
INSERT INTO products VALUES(DEFAULT,'Роллы',1000);
INSERT INTO products VALUES(DEFAULT,'Креветки',3000);
INSERT INTO products VALUES(DEFAULT,'Вода',50);