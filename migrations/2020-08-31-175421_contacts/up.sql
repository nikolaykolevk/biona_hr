CREATE TABLE contacts (
	id serial NOT NULL,
	person_id int4 NULL,
	company_id int4 NULL,
	line1 varchar(50) NULL,
	line2 varchar(50) NULL,
	line3 varchar(50) NULL,
	notes varchar(500) NULL,
	CONSTRAINT pk_contacts PRIMARY KEY (id),
	CONSTRAINT fk_contacts_companies FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE SET NULL,
	CONSTRAINT fk_contacts_people FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE SET NULL,
	CONSTRAINT chk_null CHECK (person_id IS NOT NULL OR company_id IS NOT NULL)
);