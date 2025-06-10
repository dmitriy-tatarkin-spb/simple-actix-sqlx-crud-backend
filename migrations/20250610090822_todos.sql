-- Add migration script here
CREATE TABLE todos (
    id bigserial primary key,
    description text NOT NULL,
    done boolean DEFAULT false NOT NULL
);
