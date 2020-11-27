CREATE TABLE old_tracks (
    id INTEGER PRIMARY KEY NOT NULL,
    path_ TEXT NOT NULL,
    name TEXT NOT NULL,
    artist TEXT
);

INSERT INTO old_tracks (id, path_, name, artist)
    SELECT id, path_, title, artist FROM tracks;

DROP TABLE tracks;

ALTER TABLE old_tracks RENAME TO tracks;
