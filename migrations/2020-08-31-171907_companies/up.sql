CREATE TABLE sys_company_type (
	id serial NOT NULL,
	"name" varchar(500) NOT NULL,
	CONSTRAINT sys_company_type_pk PRIMARY KEY (id)
);

CREATE TABLE companies (
	id serial NOT NULL,
	"name" varchar(500) NOT NULL,
	"type_id" int4 NOT NULL,
	is_partner bool NOT NULL,
	notes varchar(500) NULL,
	CONSTRAINT companies_pk PRIMARY KEY (id),
	CONSTRAINT companies_fk FOREIGN KEY (type_id) REFERENCES sys_company_type(id) ON DELETE SET NULL
);