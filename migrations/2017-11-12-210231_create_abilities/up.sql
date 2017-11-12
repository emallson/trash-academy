CREATE TABLE abilities (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    base_description TEXT NOT NULL,
    mob INTEGER NOT NULL,
    FOREIGN KEY(mob) REFERENCES mobs(id)
);
