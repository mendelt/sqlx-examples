
CREATE TABLE example_table (
    id bigint generated always as identity primary key,
    name varchar not null,
    value varchar not null
);

INSERT INTO example_table (name, value) VALUES
    ('name1', 'some value'),
    ('name2', 'some other value');

CREATE TABLE json_table (
    id bigint generated always as identity primary key,
    name varchar not null,

    json_stuff jsonb not null
);
