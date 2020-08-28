CREATE TABLE example_table (
    id bigint generated always as identity primary key,
    name varchar not null,

    json_stuff jsonb not null
)
