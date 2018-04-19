CREATE TABLE reviews
(
    product_id INT NOT NULL,
    reviewer VARCHAR(255) NOT NULL,
    review VARCHAR(255) NOT NULL,
    PRIMARY KEY (product_id, reviewer)
);
INSERT INTO reviews
    (product_id, reviewer, review)
VALUES
    (0, 'Reviewer1', 'An extremely entertaining play by Shakespeare. The slapstick humour is refreshing!');
INSERT INTO reviews
    (product_id, reviewer, review)
VALUES
    (0, 'Reviewer2', 'Absolutely fun and entertaining. The play lacks thematic depth when compared to other plays by Shakespeare.');
