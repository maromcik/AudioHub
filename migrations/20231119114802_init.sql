CREATE TABLE IF NOT EXISTS "User"
(
    id              bigserial PRIMARY KEY,
    ---------------------------------------------
    username        text UNIQUE NOT NULL,
    email           text UNIQUE NOT NULL,
    name            text        NOT NULL,
    surname         text        NOT NULL,
    bio             text        NOT NULL,
    profile_picture text        NOT NULL,
    password_hash   text        NOT NULL,
    password_salt   text        NOT NULL,
    created_at      timestamptz NOT NULL DEFAULT now(),
    edited_at       timestamptz NOT NULL DEFAULT now(),
    deleted_at      timestamptz
);


CREATE TABLE IF NOT EXISTS "Publisher"
(
    id           bigserial PRIMARY KEY,
    ---------------------------------------------
    name         text        NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz
);


CREATE TABLE IF NOT EXISTS "Genre"
(
    id           bigserial PRIMARY KEY,
    ---------------------------------------------
    name         text        NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz
);



CREATE TABLE IF NOT EXISTS "Audiobook"
(
    id         bigserial PRIMARY KEY,
    ---------------------------------------------
    publisher_id        bigserial       NOT NULL,
    genre_id            bigserial       NOT NULL,
    author_id           bigserial       NOT NULL,
    name                text            NOT NULL,
    price_dollars       int             NOT NULL,
    price_cents         int             NOT NULL,
    length              interval        NOT NULL,
    file_path           text            NOT NULL,
    stream_count        bigint          NOT NULL,
    overall_rating      smallint        NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz,

    FOREIGN KEY (publisher_id)      REFERENCES "Publisher" (id),
    FOREIGN KEY (genre_id)          REFERENCES "Genre" (id),
    FOREIGN KEY (author_id)      REFERENCES "User" (id)
);

CREATE TABLE IF NOT EXISTS "Chapter"
(
    id           bigserial PRIMARY KEY,
    ---------------------------------------------
    name                text            NOT NULL,
    audiobook_id        bigserial       NOT NULL,
    length              interval        NOT NULL,
    sequential_number   int             NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz,

    FOREIGN KEY (audiobook_id)      REFERENCES "Audiobook" (id)
);

CREATE TABLE IF NOT EXISTS "Rating"
(
    id         bigserial PRIMARY KEY,
    ---------------------------------------------
    user_id         bigserial        NOT NULL,
    audiobook_id    bigserial        NOT NULL,
    rating          smallint    NOT NULL,
    review          text,
    created_at      timestamptz NOT NULL DEFAULT now(),
    edited_at       timestamptz NOT NULL DEFAULT now(),
    deleted_at      timestamptz,

    FOREIGN KEY (user_id)       REFERENCES "User" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Audiobook" (id)
);

CREATE TABLE IF NOT EXISTS "Bookmark"
(
    user_id         bigserial        NOT NULL,
    audiobook_id    bigserial        NOT NULL,

    PRIMARY KEY (user_id, audiobook_id),
    FOREIGN KEY (user_id)       REFERENCES "User" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Audiobook" (id)
);


CREATE TABLE IF NOT EXISTS "Active_Audiobook"
(
    user_id                         bigserial        NOT NULL,
    audiobook_id                    bigserial        NOT NULL,
    playback_chapter_id             bigserial,
    playback_position_in_chapter    interval        DEFAULT make_interval(0,0,0,0,0,0,0),

    PRIMARY KEY (user_id, audiobook_id, playback_chapter_id),
    FOREIGN KEY (user_id)       REFERENCES "User" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Audiobook" (id),
    FOREIGN KEY (audiobook_id)  REFERENCES "Chapter" (id)
);