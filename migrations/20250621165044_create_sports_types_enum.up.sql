-- Create sports type enum
CREATE TYPE sport_type AS ENUM (
    'basketball',
    'table_tennis',
    'volleyball',
    'badminton',
    'tennis',
    'football',
    'cricket',
    'chess',
    'esports'
);

-- Create team composition enum (for categories)
CREATE TYPE team_composition AS ENUM (
    'singles',
    'doubles',
    'mixed_doubles',
    'team'
);

-- Create tournament format enum
CREATE TYPE tournament_format AS ENUM (
    'elimination',
    'double_elimination',
    'round_robin',
    'league',
    'swiss',
    'groups_and_knockout'
);

-- Create tournament status enum
CREATE TYPE tournament_status AS ENUM (
    'draft',
    'upcoming',
    'registration_open',
    'registration_closed',
    'in_progress',
    'completed',
    'cancelled'
);