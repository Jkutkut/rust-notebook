CREATE TABLE CATEGORY (
    ID INT AUTO_INCREMENT,
    CAT_NAME VARCHAR(42) NOT NULL UNIQUE,

    PRIMARY KEY (ID)
);

CREATE TABLE TAG (
    ID INT AUTO_INCREMENT,
    TAG_NAME VARCHAR(42) NOT NULL UNIQUE,

    PRIMARY KEY (ID),
    FOREIGN KEY (CATEGORY_ID) REFERENCES CATEGORY(ID)
);

CREATE TABLE NOTES (
    ID INT AUTO_INCREMENT,
    NOTE_NAME VARCHAR(42) NOT NULL,
    NOTE_DESC VARCHAR(420) NOT NULL,
    
    CATEGORY_ID INT NOT NULL,
    TAG_ID INT NOT NULL,

    PRIMARY KEY (ID)
);

INSERT INTO CATEGORY (CAT_NAME) VALUES ('UNCATAGORIZED');
INSERT INTO TAG (TAG_NAME) VALUES ('UNTAGGED');
