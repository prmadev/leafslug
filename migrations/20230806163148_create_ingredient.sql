create table ingredients (
    id serial not null,
    primary key (id),
    name text not null
);


create table ingredient_relations (
    id serial not null,
    primary key (id),

    original_id integer not null,
    foreign key (original_id) references ingredients (id),

    substitute_id integer not null,
    foreign key (substitute_id) references ingredients (id)
);
