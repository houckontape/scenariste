-- Migration : Création de la table users
CREATE TABLE IF NOT EXISTS users (
                                     id SERIAL PRIMARY KEY,
                                     email VARCHAR(255) NOT NULL UNIQUE,
                                     company VARCHAR(255) NOT NULL,
                                     password_hash VARCHAR(255) NOT NULL, -- Ajout du champ sécurisé
                                     created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
                             );