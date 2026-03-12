-- 1. Create Enums if they don't exist
DO $$ BEGIN
    CREATE TYPE website_status AS ENUM ('up', 'down', 'unknown');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE monitor_type AS ENUM ('URL', 'HEARTBEAT');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 2. Users Table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Websites (The Monitors)
CREATE TABLE IF NOT EXISTS websites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    period INTEGER NOT NULL DEFAULT 60,
    monitor_type monitor_type NOT NULL DEFAULT 'URL',
    paused BOOLEAN DEFAULT FALSE,
    last_heartbeat TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 4. Website Ticks (The History Logs)
CREATE TABLE IF NOT EXISTS website_ticks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    website_id UUID NOT NULL REFERENCES websites(id) ON DELETE CASCADE,
    response_time_ms INTEGER NOT NULL,
    status website_status NOT NULL,
    message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
