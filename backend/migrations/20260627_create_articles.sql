CREATE TABLE IF NOT EXISTS articles (
                                        id SERIAL PRIMARY KEY,
                                        title VARCHAR(255) NOT NULL,
                                        slug VARCHAR(255) UNIQUE NOT NULL, -- Pour des URLs propres côté Angular (ex: /blog/mon-premier-article)
                                        content TEXT NOT NULL,              -- Contenu brut en Markdown
                                        author_name VARCHAR(100) NOT NULL,
                                        published_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);