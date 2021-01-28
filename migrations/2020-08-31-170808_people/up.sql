CREATE TABLE people (
	id serial NOT NULL,
	first_name varchar(40) NOT NULL,
	last_name varchar(40) NOT NULL,
	is_employee bool NOT NULL,
	notes varchar(500) NULL,
	CONSTRAINT pk_people PRIMARY KEY (id)
);