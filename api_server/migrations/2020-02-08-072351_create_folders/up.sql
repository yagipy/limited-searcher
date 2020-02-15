CREATE TABLE folders (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL,
    name VARCHAR NOT NULL,
    status BOOLEAN NOT NULL DEFAULT 't',
    CONSTRAINT fk_user_id
        FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE RESTRICT ON UPDATE RESTRICT
)