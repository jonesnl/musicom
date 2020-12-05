CREATE TABLE new_tracks (
    id INTEGER PRIMARY KEY NOT NULL,
    path_ TEXT UNIQUE NOT NULL,
    title TEXT,
    artist TEXT,
    album TEXT,
    track_num INTEGER
);

INSERT INTO new_tracks (path_, title, artist, album, track_num)
    SELECT DISTINCT path_, title, artist, album, track_num FROM tracks;

DROP TABLE tracks;

ALTER TABLE new_tracks RENAME TO tracks;
