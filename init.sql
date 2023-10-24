CREATE TABLE movie (
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(50),
    description TEXT,
    created_at timestamp DEFAULT current_timestamp,
    updated_at timestamp
);