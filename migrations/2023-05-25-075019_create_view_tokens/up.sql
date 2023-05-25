-- Your SQL goes here
CREATE TABLE view_tokens (
    id SERIAL PRIMARY KEY,
    token VARCHAR(20) NOT NULL UNIQUE,
    filepath VARCHAR(255) NOT NULL,
    expiry_date TIMESTAMP NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)