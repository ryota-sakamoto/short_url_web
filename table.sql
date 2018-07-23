create database short_url;
use short_url;
create table url_list (
    id varchar(10) not null,
    url varchar(255) not null
);
create index url_list_index on url_list(id);