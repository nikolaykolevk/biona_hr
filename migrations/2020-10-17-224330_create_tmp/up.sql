CREATE TABLE tmp (
	id serial NOT NULL,
	tmp_date date NULL,
	"name" varchar NULL,
	CONSTRAINT tmp_pk PRIMARY KEY (id),
	CONSTRAINT tmp_un UNIQUE (id)
);