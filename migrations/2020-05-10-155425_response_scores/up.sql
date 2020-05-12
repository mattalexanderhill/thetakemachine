CREATE VIEW response_scores (
  id,
  session_id,
  question_id,
  answer_id,
  x,
  y
)
AS SELECT
  ROW_NUMBER() OVER (ORDER BY r.at),
  r.session_id,
  r.question_id,
  r.answer_id,
  s.x,
  s.y
FROM
  responses as r
LEFT JOIN
  scores as s
  ON s.question_id = r.question_id
  AND s.answer_id = r.answer_id
;
