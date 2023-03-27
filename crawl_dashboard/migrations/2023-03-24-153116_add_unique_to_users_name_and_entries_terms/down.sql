-- This file should undo anything in `up.sql`

alter table users
drop constraint unique_names ;

alter table entries
drop constraint unique_terms ;
