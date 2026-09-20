ALTER TABLE experiences
    ADD CONSTRAINT experiences_user_practice_unique UNIQUE (user_id, practice_id);
