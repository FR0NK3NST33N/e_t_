-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id INT PRIMARY KEY,
  email VARCHAR(250) UNIQUE NOT NULL,
  username VARCHAR(250) UNIQUE NOT NULL,
  password VARCHAR(250) NOT NULL
);

INSERT INTO users(id, email, username, password) VALUES (1, 'admin@et.com', 'admin', 'password');