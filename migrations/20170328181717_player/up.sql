CREATE TABLE player (
  id SERIAL PRIMARY KEY,
  name VARCHAR(15) NOT NULL
);

ALTER TABLE round ADD COLUMN player_id INT NOT NULL REFERENCES player ON DELETE CASCADE;