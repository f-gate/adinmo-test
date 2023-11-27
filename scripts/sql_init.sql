-- Create database
-- CREATE DATABASE IF NOT EXISTS adinmo_test_db;

-- Create user and grant privileges
CREATE USER 'adinmo'%'@' IDENTIFIED BY 'Password123!';
GRANT ALL PRIVILEGES ON '*' TO 'adinmo'@'%';
FLUSH PRIVILEGES;