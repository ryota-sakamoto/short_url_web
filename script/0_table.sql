create database if not exists short_url;
use short_url;
create table short_url.url_list (
    id varchar(10) not null,
    password varchar(64),
    url varchar(255) not null
);
create index url_list_index on url_list(id, password);