-- Perseus Membership System Database Schema
-- Run this file to set up the database: psql -U postgres -d membership -f schema.sql

-- Create the database (run as postgres superuser)
-- CREATE DATABASE membership;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    hashed_password VARCHAR(255) NOT NULL,
    reset_password_selector VARCHAR(255),
    reset_password_sent_at TIMESTAMP,
    reset_password_validator_hash VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    session_verifier VARCHAR(255) NOT NULL,
    otp_code_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    otp_code_encrypted VARCHAR(255) NOT NULL,
    otp_code_attempts INTEGER NOT NULL DEFAULT 0,
    otp_code_sent BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_users_reset_selector ON users(reset_password_selector);

-- Function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to automatically update updated_at
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Clean up old sessions (run periodically)
-- DELETE FROM sessions WHERE created_at < NOW() - INTERVAL '7 days';
