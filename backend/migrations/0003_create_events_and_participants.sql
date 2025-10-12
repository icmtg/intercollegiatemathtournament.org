-- Create events table to support multiple events
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    location VARCHAR(255),
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    registration_open BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_events_registration_open ON events(registration_open);
CREATE INDEX idx_events_start_date ON events(start_date DESC);

-- Create participants table with flexible JSONB field for form data
CREATE TABLE IF NOT EXISTS participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    tshirt_size VARCHAR(10) NOT NULL,
    division VARCHAR(10) NOT NULL,
    expected_graduation_year INTEGER NOT NULL,
    university VARCHAR(255) NOT NULL,
    resume_url TEXT,
    acknowledged_id_requirement BOOLEAN NOT NULL DEFAULT false,
    acknowledged_filming BOOLEAN NOT NULL DEFAULT false,
    acknowledged_team_merge BOOLEAN NOT NULL DEFAULT false,
    interested_in_financial_aid BOOLEAN NOT NULL DEFAULT false,
    -- Flexible field for future extensions or event-specific data
    additional_data JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_participants_event_id ON participants(event_id);
CREATE INDEX idx_participants_user_id ON participants(user_id);
CREATE INDEX idx_participants_email ON participants(email);
CREATE INDEX idx_participants_created_at ON participants(created_at DESC);
