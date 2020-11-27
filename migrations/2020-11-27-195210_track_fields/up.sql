CREATE TABLE new_tracks (
    id INTEGER PRIMARY KEY NOT NULL,
    path_ TEXT NOT NULL,
    title TEXT,
    artist TEXT,
    album TEXT,
    track_num INTEGER
);

INSERT INTO new_tracks (id, path_, title, artist, album, track_num)
    SELECT id, path_, name, artist, NULL, NULL FROM tracks;

DROP TABLE tracks;

ALTER TABLE new_tracks RENAME TO tracks;
