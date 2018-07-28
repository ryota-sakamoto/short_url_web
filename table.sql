create database short_url;
use short_url;
create table url_list (
    id varchar(10) not null,
    password varchar(32),
    url varchar(255) not null
);
create index url_list_index on url_list(id);