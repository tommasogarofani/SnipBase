/* Active: 1792953316422@@127.0.0.1@5434@snipbase */
CREATE TABLE IF NOT EXISTS snippet
(   id             BIGSERIAL      PRIMARY KEY
   ,title          VARCHAR(255)   NOT NULL
   ,description    TEXT
   ,content        TEXT           NOT NULL
   ,language       VARCHAR(64)    NOT NULL
   ,public         BOOLEAN        DEFAULT FALSE
   ,created_at     TIMESTAMP      WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tag
(   id       BIGSERIAL      PRIMARY KEY
   ,name     VARCHAR(255)   UNIQUE NOT NULL
   ,color    VARCHAR(7)     DEFAULT '#6366f1'
);

CREATE TABLE IF NOT EXISTS snippet_tag
(   snippet_id    BIGSERIAL   REFERENCES snippet(id) ON DELETE CASCADE
   ,tag_id        BIGSERIAL   REFERENCES tag(id) ON DELETE CASCADE
   ,PRIMARY KEY (snippet_id, tag_id)
);