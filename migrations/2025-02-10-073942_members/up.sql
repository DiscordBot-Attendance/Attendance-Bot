CREATE TABLE members (
    id SERIAL PRIMARY KEY,
    team_id INTEGER REFERENCES teams(id) ON DELETE CASCADE,
    discord_id VARCHAR(50) NOT NULL,
    position VARCHAR(50),
    join_date DATE DEFAULT CURRENT_DATE
);

