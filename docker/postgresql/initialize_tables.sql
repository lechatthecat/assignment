-- Create restaurant tables table
DROP TABLE IF EXISTS restaurant_tables;
CREATE TABLE restaurant_tables (
  id INTEGER primary key generated always as identity,
  note VARCHAR(500) DEFAULT NULL
);

-- Adds 10 tables
INSERT INTO restaurant_tables (note) VALUES (null);
INSERT INTO restaurant_tables (note) VALUES (null);
INSERT INTO restaurant_tables (note) VALUES (null);
INSERT INTO restaurant_tables (note) VALUES (null);
INSERT INTO restaurant_tables (note) VALUES (null);
INSERT INTO restaurant_tables (note) VALUES (null);
INSERT INTO restaurant_tables (note) VALUES (null);
INSERT INTO restaurant_tables (note) VALUES (null);
INSERT INTO restaurant_tables (note) VALUES (null);
INSERT INTO restaurant_tables (note) VALUES (null);

-- Create users table
DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id INTEGER primary key generated always as identity,
  name VARCHAR(50) NOT NULL,
  password VARCHAR(61) NOT null,
  UNIQUE(name)
);

-- add users
INSERT INTO public.users ("name","password") VALUES
	 ('test_user1','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user2','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user3','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user4','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user5','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i');
