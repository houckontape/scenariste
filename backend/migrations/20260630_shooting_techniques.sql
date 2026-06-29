-- Up
-- Création de la table des techniques de prise de vue
CREATE TABLE shooting_techniques (
                                     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                     name VARCHAR(100) NOT NULL UNIQUE,
                                     slug VARCHAR(100) NOT NULL UNIQUE,
                                     description TEXT,
                                     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insertion des techniques de base
INSERT INTO shooting_techniques (name, slug, description) VALUES
                                                              ('Prise de vue réelle', 'live_action', 'Tournage avec des caméras et des acteurs réels.'),
                                                              ('Stop motion', 'stop_motion', 'Animation image par image d''objets réels.'),
                                                              ('Dessin animé 2D', '2d_animation', 'Animation traditionnelle ou numérique en deux dimensions.'),
                                                              ('Animation 3D', '3d_animation', 'Animation générée par ordinateur en trois dimensions.'),
                                                              ('Motion capture', 'motion_capture', 'Enregistrement des mouvements d''acteurs pour animer des personnages numériques.');

-- Ajout de la technique par défaut au projet
-- On ne met pas NOT NULL tout de suite pour ne pas bloquer les projets existants si besoin,
-- mais idéalement on devrait en assigner une par défaut.
ALTER TABLE projects ADD COLUMN shooting_technique_id UUID REFERENCES shooting_techniques(id) ON DELETE SET NULL;

-- Ajout de la possibilité de modifier la technique par scène
-- Nullable signifie qu'on utilise la technique du projet par défaut
ALTER TABLE scenes ADD COLUMN shooting_technique_id UUID REFERENCES shooting_techniques(id) ON DELETE SET NULL;

-- Trigger pour updated_at sur shooting_techniques
CREATE TRIGGER update_shooting_techniques_updated_at BEFORE UPDATE ON shooting_techniques FOR EACH ROW EXECUTE PROCEDURE update_computed_at_column();
