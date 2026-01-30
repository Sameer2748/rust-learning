-- Create users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);

-- Create todos table
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    tag TEXT NOT NULL
);
