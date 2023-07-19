CREATE TABLE tags(
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL
);

CREATE TABLE folders (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  parent_id INT,
  FOREIGN KEY(parent_id) REFERENCES folders(id)
);

CREATE TABLE notes (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  content JSON NOT NULL,
  parent_id INT
);

CREATE TABLE note_tags (
  note_id INT,
  tag_id INT,
  PRIMARY KEY(note_id, tag_id),
  FOREIGN KEY(tag_id) REFERENCES tags(id),
  FOREIGN KEY(note_id) REFERENCES notes(id)
);
