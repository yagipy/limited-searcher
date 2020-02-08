CREATE TABLE urls (
    id SERIAL PRIMARY KEY,
    folder_id SERIAL NOT NULL,
    url VARCHAR NOT NULL,
    status BOOLEAN NOT NULL DEFAULT 't',
    CONSTRAINT fk_folder_id
        FOREIGN KEY (folder_id)
        REFERENCES folders (id)
        ON DELETE RESTRICT ON UPDATE RESTRICT
)