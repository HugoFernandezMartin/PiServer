DROP TABLE IF EXISTS usuario;

CREATE TABLE usuario (
    nombre text PRIMARY KEY,
    password text not null,
    rol text not null CHECK(rol in('admin', 'user'))
);

