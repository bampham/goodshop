CREATE TABLE ShoppingList (
    list_id INT AUTO_INCREMENT PRIMARY KEY,
    list_name VARCHAR(64) NOT NULL,
    time_stamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
);

CREATE TABLE Product (
    product_id INT AUTO_INCREMENT PRIMARY KEY,
    list_id INT NOT NULL,
    product_name VARCHAR(64) NOT NULL,
    quantity INT NOT NULL,
    time_stamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (list_id) REFERENCES ShoppingList(list_id)
);

