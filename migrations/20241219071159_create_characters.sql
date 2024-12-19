-- Add migration script here
CREATE TABLE `characters` (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    actor_name VARCHAR(255) NOT NULL
);