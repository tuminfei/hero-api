-- Your SQL goes here
CREATE TABLE login_history
(
    id INT(11) PRIMARY KEY AUTO_INCREMENT,
    user_id INT(11) NOT NULL REFERENCES users(id),
    login_timestamp TIMESTAMP NOT NULL
);