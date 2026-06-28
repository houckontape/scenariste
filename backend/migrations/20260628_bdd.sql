-- Up
-- 1. ENUMS & EXTENSIONS
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_role AS ENUM ('free_user', 'premium_user', 'support', 'admin', 'super_admin');
CREATE TYPE project_type AS ENUM ('movie', 'series', 'animation', 'novel');
CREATE TYPE member_role AS ENUM ('owner', 'write', 'read');

-- 2. TABLE USERS (Authentification)
CREATE TABLE users (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       email VARCHAR(255) UNIQUE NOT NULL,
                       password_hash VARCHAR(255) NOT NULL,
                       role user_role NOT NULL DEFAULT 'free_user',
                       is_active BOOLEAN NOT NULL DEFAULT TRUE,
                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. TABLE PROFILES (Informations Personnelles)
CREATE TABLE profiles (
                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                          user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
                          first_name VARCHAR(100),
                          last_name VARCHAR(100),
                          avatar_url VARCHAR(512),
                          bio TEXT,
                          address TEXT,
                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. TABLE PROJECTS (L'œuvre globale - Style GitHub Repo)
CREATE TABLE projects (
                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                          title VARCHAR(255) NOT NULL,
                          slug VARCHAR(255) NOT NULL, -- Pour des URLs propres style github.com/user/mon-film
                          description TEXT,
                          p_type project_type NOT NULL,
                          is_private BOOLEAN NOT NULL DEFAULT TRUE,
                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          CONSTRAINT unique_user_project_slug UNIQUE (id, slug) -- Clé unique combinée
);

-- 5. TABLE PIVOT : MEMBRES DU PROJET (Collaborateurs - Style GitHub Teams)
CREATE TABLE project_members (
                                 project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                                 user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                                 role member_role NOT NULL DEFAULT 'read',
                                 invited_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                 joined_at TIMESTAMPTZ,
                                 PRIMARY KEY (project_id, user_id)
);

-- 6. TABLE BRAINSTORMING (Notes & Idées - Style GitHub Issues/Wiki)
CREATE TABLE brainstorming_notes (
                                     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                     project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                                     author_id UUID NOT NULL REFERENCES users(id) ON DELETE SET NULL,
                                     title VARCHAR(255) NOT NULL,
                                     content TEXT NOT NULL, -- Support du Markdown textuel
                                     tags VARCHAR(50)[], -- Array PostgreSQL pour filtrer par #idée, #acte1, #dialogue
                                     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 7. TABLE NARRATIVE : SYNOPSIS & TRAITEMENT
CREATE TABLE project_synopses (
                                  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                  project_id UUID NOT NULL UNIQUE REFERENCES projects(id) ON DELETE CASCADE,
                                  logline VARCHAR(512), -- L'idée en une phrase accrocheuse
                                  summary_short TEXT,   -- Synopsis court (1 page)
                                  summary_long TEXT,    -- Traitement complet (Plusieurs pages)
                                  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 8. TABLE NARRATIVE : BIBLE DES PERSONNAGES (Celtx style)
CREATE TABLE character_bible (
                                 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                 project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                                 full_name VARCHAR(255) NOT NULL,
                                 character_role VARCHAR(100) DEFAULT 'protagonist', -- protagonist, antagonist, mentor, etc.
                                 avatar_url VARCHAR(512),
                                 description TEXT, -- Description globale
    -- Le champ ci-dessous utilise JSONB pour stocker la psychologie, les désirs, les faiblesses
    -- de manière hyper flexible sans figer le schéma si on ajoute des critères.
                                 attributes JSONB NOT NULL DEFAULT '{}'::jsonb,
                                 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                 updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 9. TRIGGERS POUR LA GESTION AUTOMATIQUE DE UPDATED_AT
CREATE OR REPLACE FUNCTION update_computed_at_column()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();
CREATE TRIGGER update_profiles_updated_at BEFORE UPDATE ON profiles FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();
CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON projects FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();
CREATE TRIGGER update_brainstorming_notes_updated_at BEFORE UPDATE ON brainstorming_notes FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();
CREATE TRIGGER update_project_synopses_updated_at BEFORE UPDATE ON project_synopses FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();
CREATE TRIGGER update_character_bible_updated_at BEFORE UPDATE ON character_bible FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();

-- Up

-- 1. ENUMS POUR LE SCÉNARIO
CREATE TYPE scene_time_of_day AS ENUM ('DAY', 'NIGHT', 'MORNING', 'EVENING', 'DAWN', 'DUSK');
CREATE TYPE scene_setting AS ENUM ('INT', 'EXT', 'INT_EXT'); -- Intérieur / Extérieur / Mixte

-- 2. TABLE DES ACTES (Pour structurer le récit : Acte I, Acte II, Acte III...)
CREATE TABLE project_acts (
                              id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                              project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                              title VARCHAR(255) NOT NULL, -- Ex: "L'Appel à l'aventure"
                              position INT NOT NULL, -- Pour trier et réordonner les actes (1, 2, 3...)
                              description TEXT,
                              created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                              updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                              CONSTRAINT unique_project_act_position UNIQUE (project_id, position)
);

-- 3. TABLE DES SCÈNES (Le cœur du traitement de texte style Fade In)
CREATE TABLE scenes (
                        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                        act_id UUID NOT NULL REFERENCES project_acts(id) ON DELETE CASCADE,
                        project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE, -- Doublé pour optimiser les requêtes par projet
                        position INT NOT NULL, -- Ordre de la scène dans l'acte/projet
                        setting scene_setting NOT NULL DEFAULT 'INT', -- INT. ou EXT.
                        location VARCHAR(255) NOT NULL, -- Ex: "CAFE DE LA GARE"
                        time_of_day scene_time_of_day NOT NULL DEFAULT 'DAY', -- JAV / NUIT
                        content TEXT NOT NULL DEFAULT '', -- Contenu brut du script (Format Fountain ou JSON structuré pour l'éditeur Angular)
                        note TEXT, -- Note de travail spécifique à la scène
                        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. GESTION DES SCHÉMAS ET ARCS NARRATIFS (Style Excalidraw / Miro)
CREATE TABLE project_canvases (
                                  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                  project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                                  title VARCHAR(255) NOT NULL DEFAULT 'Arc Narratif Principal',
                                  description TEXT,
    -- Excalidraw génère un JSON contenant la liste des éléments graphiques (rectangles, flèches, textes).
    -- Le type JSONB de Postgres est parfait pour stocker et requêter cette structure à haute vitesse.
                                  elements JSONB NOT NULL DEFAULT '[]'::jsonb,
    -- Permet de stocker l'état de la caméra Excalidraw (zoom, scrollX, scrollY)
                                  app_state JSONB NOT NULL DEFAULT '{}'::jsonb,
                                  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 5. INDEXES POUR ASSURER DES PERFORMANCES ENTERPRISE
CREATE INDEX idx_scenes_project ON scenes(project_id);
CREATE INDEX idx_scenes_act ON scenes(act_id);
CREATE INDEX idx_project_canvases_project ON project_canvases(project_id);

-- 6. TRIGGERS POUR UPDATED_AT
CREATE TRIGGER update_project_acts_updated_at BEFORE UPDATE ON project_acts FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();
CREATE TRIGGER update_scenes_updated_at BEFORE UPDATE ON scenes FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();
CREATE TRIGGER update_project_canvases_updated_at BEFORE UPDATE ON project_canvases FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();