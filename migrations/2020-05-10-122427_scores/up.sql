CREATE TABLE scores (
  id          SERIAL  PRIMARY KEY,
  question_id INTEGER NOT NULL REFERENCES questions(id),
  answer_id   INTEGER NOT NULL REFERENCES answers(id),
  x           INTEGER NOT NULL,
  y           INTEGER NOT NULL
);
