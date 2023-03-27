-- Your SQL goes here
alter table users
add constraint unique_names UNIQUE (name);

alter table entries
add constraint unique_terms UNIQUE(terms);
