-- Add migration script here
-- Add role column to users and set default to 'user'
ALTER TABLE users
ADD COLUMN IF NOT EXISTS role TEXT DEFAULT 'user';

-- Ensure existing rows have a role
UPDATE users SET role = 'user' WHERE role IS NULL;

-- (Optional) You may add an admin user here manually or with a separate seed migration.
