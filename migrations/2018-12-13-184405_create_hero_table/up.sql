-- Your SQL goes here
CREATE TABLE hero (
    id INT AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    identity VARCHAR(255) NOT NULL,
    hometown VARCHAR(255) NOT NULL,
    age INT(8) NOT NULL,
    PRIMARY KEY (id)
);