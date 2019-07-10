-- Your SQL goes here
CREATE TABLE people(
  id INT(11) PRIMARY KEY AUTO_INCREMENT,
  first_name VARCHAR(60) NOT NULL,
  last_name VARCHAR(60) NOT NULL,
  age INT(11) NOT NULL,
  profession VARCHAR(60) NOT NULL,
  salary INT(11) NOT NULL
)