CREATE SEQUENCE user_id_seq;
CREATE TABLE users (
    id           integer NOT NULL DEFAULT nextval('user_id_seq') CONSTRAINT user_key PRIMARY KEY,
    email        text NOT NULL,
    pass_hash    text NOT NULL,
    first_name   text NOT NULL,
    last_name    text NOT NULL,
    section      text NOT NULL,
    phone_number varchar(10) NOT NULL,
    UNIQUE(email)
);
ALTER SEQUENCE user_id_seq OWNED BY users.id;
INSERT INTO users (email, pass_hash, first_name, last_name, section, phone_number)
VALUES ('sammohr97@gmail.com', '4bd9c5e227a71a432f67599c4d9a05bc4f192d3f98892fe6fdf62c267c873cf4', 'Sam', 'Mohr', 'Tenor 2', '4047751184');

CREATE SEQUENCE event_id_seq;
CREATE TABLE events (
    id               integer NOT NULL DEFAULT nextval('event_id_seq') CONSTRAINT event_key PRIMARY KEY,
    title            text NOT NULL,
    location         text NOT NULL,
    category         text NOT NULL,
    description      text,
    start_time       integer NOT NULL,
    end_time         integer NOT NULL,
    performance_time integer
);
ALTER SEQUENCE event_id_seq OWNED BY events.id;

CREATE SEQUENCE attendance_id_seq;
CREATE TABLE attendances (
    id            integer NOT NULL DEFAULT nextval('attendance_id_seq') CONSTRAINT attendance_key PRIMARY KEY,
    event_id      integer NOT NULL REFERENCES events(id),
    user_id       integer NOT NULL REFERENCES users(id),
    should_attend boolean NOT NULL DEFAULT 'f',
    did_attend    boolean NOT NULL DEFAULT 'f',
    minutes_late  integer NOT NULL DEFAULT 0,
    confirmed     boolean NOT NULL DEFAULT 'f'
);
ALTER SEQUENCE attendance_id_seq OWNED BY attendances.id;

CREATE SEQUENCE carpool_id_seq;
CREATE TABLE carpools (
    id        integer NOT NULL DEFAULT nextval('carpool_id_seq') CONSTRAINT carpool_key PRIMARY KEY,
    event_id  integer NOT NULL REFERENCES events(id),
    user_id   integer NOT NULL REFERENCES users(id),
    is_driver boolean NOT NULL DEFAULT 'f',
    driver_id integer
);
ALTER SEQUENCE carpool_id_seq OWNED BY carpools.id;

CREATE SEQUENCE song_id_seq;
CREATE TABLE songs (
    id            integer NOT NULL DEFAULT nextval('song_id_seq') CONSTRAINT song_key PRIMARY KEY,
    name          text NOT NULL,
    description   text,
    key           varchar(2) NOT NULL DEFAULT 'c',
    starting_pitch  varchar(2) NOT NULL DEFAULT 'c',
    this_semester boolean NOT NULL DEFAULT 'f'
);
ALTER SEQUENCE song_id_seq OWNED BY songs.id;

CREATE SEQUENCE file_id_seq;
CREATE TABLE files (
    id        integer NOT NULL DEFAULT nextval('file_id_seq') CONSTRAINT file_key PRIMARY KEY,
    song_id   integer NOT NULL REFERENCES songs(id),
    path text NOT NULL,
    name      text NOT NULL,
    is_sheet  boolean NOT NULL DEFAULT 'f'

);
ALTER SEQUENCE file_id_seq OWNED BY files.id;

CREATE SEQUENCE link_id_seq;
CREATE TABLE links (
    id             integer NOT NULL DEFAULT nextval('link_id_seq') CONSTRAINT link_key PRIMARY KEY,
    song_id        integer NOT NULL REFERENCES songs(id),
    link           text NOT NULL,
    name           text NOT NULL,
    is_performance boolean NOT NULL DEFAULT 'f'
);
ALTER SEQUENCE link_id_seq OWNED BY links.id;
