## Process de la mise en place de l environement de dev 

### Démarrer la base de données sous docker
depuis le terminal dans le dossier racine  
```bash
docker compose up -d
``` 
### Arreter la base de données et vider les volummes
```bash
docker compose down -v
```
### Démarer le serveur API   
depuis le dossier backend  
```bash
cargo run
``` 

### Démarer le serveur fronetnd

```bash
npm start
```

## Documentation de l'API

L'API est accessible sur `http://localhost:3000`.

### Authentification

Certaines routes nécessitent un token JWT passé dans le header `Authorization`.

**Format :** `Authorization: Bearer <votre_token>`

---

### Routes Authentification

#### 1. Inscription
- **URL :** `/api/auth/register`
- **Méthode :** `POST`
- **Corps (JSON) :**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```
- **Réponse (201 Created) :**
```json
{
  "message": "Utilisateur et profil créés avec succès !",
  "user_id": "uuid-v4"
}
```

#### 2. Connexion
- **URL :** `/api/auth/login`
- **Méthode :** `POST`
- **Corps (JSON) :**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```
- **Réponse (200 OK) :**
```json
{
  "token": "eyJhbG...",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "role": "free_user"
  }
}
```

#### 3. Utilisateur actuel (Me)
- **URL :** `/api/auth/me`
- **Méthode :** `GET`
- **Auth :** Requis (JWT)
- **Réponse (200 OK) :**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "role": "free_user",
  "is_active": true,
  "profile": {
    "id": "uuid",
    "user_id": "uuid",
    "first_name": "Jean",
    "last_name": "Dupont",
    "avatar_url": null,
    "bio": "Ma bio",
    "address": null,
    "created_at": "2026-06-29T10:00:00Z",
    "updated_at": "2026-06-29T10:00:00Z"
  }
}
```

#### 4. Mise à jour du profil
- **URL :** `/api/auth/profile`
- **Méthode :** `POST`
- **Auth :** Requis (JWT)
- **Corps (JSON) :**
```json
{
  "first_name": "Jean",
  "last_name": "Dupont",
  "avatar_url": "http://...",
  "bio": "Ma bio",
  "address": "123 rue Rust"
}
```
- **Réponse (200 OK) :** Profil complet mis à jour.

---

### Routes Projets

#### 1. Lister les projets
- **URL :** `/api/projects`
- **Méthode :** `GET`
- **Auth :** Requis (JWT)
- **Réponse (200 OK) :** `Array<ProjectWithRole>`
```json
[
  {
    "id": "uuid",
    "title": "Mon Film",
    "slug": "mon-film",
    "description": "...",
    "p_type": "movie",
    "is_private": true,
    "created_at": "...",
    "updated_at": "...",
    "user_role": "owner"
  }
]
```

#### 2. Créer un projet
- **URL :** `/api/projects`
- **Méthode :** `POST`
- **Auth :** Requis (JWT)
- **Corps (JSON) :**
```json
{
  "title": "Mon nouveau projet",
  "description": "Une super histoire",
  "p_type": "movie",
  "is_private": true
}
```
- **Réponse (201 Created) :** Le projet créé.

#### 3. Voir / Modifier / Supprimer un projet
- **URL :** `/api/projects/:id`
- **Méthodes :** `GET`, `PATCH`, `DELETE`
- **Auth :** Requis (JWT)
- **Corps (PATCH) :** Tous les champs sont optionnels.
```json
{
  "title": "Nouveau titre",
  "description": "Nouvelle description",
  "p_type": "series",
  "is_private": false
}
```

---

### Routes Actes

#### 1. Lister les actes d'un projet
- **URL :** `/api/projects/:id/acts`
- **Méthode :** `GET`
- **Auth :** Requis (JWT)

#### 2. Créer un acte
- **URL :** `/api/acts`
- **Méthode :** `POST`
- **Auth :** Requis (JWT)
- **Corps (JSON) :**
```json
{
  "project_id": "uuid",
  "title": "Acte I",
  "position": 1,
  "description": "L'introduction"
}
```

#### 3. Voir / Modifier / Supprimer un acte
- **URL :** `/api/acts/:id`
- **Méthodes :** `GET`, `PATCH`, `DELETE`
- **Corps (PATCH) :** Tous les champs sont optionnels.

---

### Routes Scènes

#### 1. Lister les scènes d'un projet
- **URL :** `/api/projects/:id/scenes`
- **Méthode :** `GET`
- **Auth :** Requis (JWT)

#### 2. Créer une scène
- **URL :** `/api/scenes`
- **Méthode :** `POST`
- **Auth :** Requis (JWT)
- **Corps (JSON) :**
```json
{
  "act_id": "uuid",
  "project_id": "uuid",
  "position": 1,
  "setting": "INT",
  "location": "SALON",
  "time_of_day": "DAY",
  "content": "Le texte de la scène",
  "note": "Ma note"
}
```

#### 3. Voir / Modifier / Supprimer une scène
- **URL :** `/api/scenes/:id`
- **Méthodes :** `GET`, `PATCH`, `DELETE`
- **Corps (PATCH) :** Tous les champs sont optionnels.

---

### Routes Brainstorming (Notes)

#### 1. Lister les notes d'un projet
- **URL :** `/api/projects/:id/notes`
- **Méthode :** `GET`
- **Auth :** Requis (JWT)

#### 2. Créer une note
- **URL :** `/api/notes`
- **Méthode :** `POST`
- **Auth :** Requis (JWT)
- **Corps (JSON) :**
```json
{
  "project_id": "uuid",
  "title": "Idée géniale",
  "content": "Du contenu **Markdown**",
  "tags": ["idée", "personnage"]
}
```

#### 3. Voir / Modifier / Supprimer une note
- **URL :** `/api/notes/:id`
- **Méthodes :** `GET`, `PATCH`, `DELETE`

---

### Routes Membres du projet

#### 1. Lister les membres
- **URL :** `/api/projects/:id/members`
- **Méthode :** `GET`
- **Auth :** Requis (JWT)

#### 2. Ajouter un membre
- **URL :** `/api/projects/:id/members`
- **Méthode :** `POST`
- **Auth :** Requis (JWT)
- **Corps (JSON) :**
```json
{
  "email": "collaborateur@example.com",
  "role": "write"
}
```

#### 3. Modifier le rôle d'un membre
- **URL :** `/api/projects/:id/members/:user_id`
- **Méthode :** `PATCH`
- **Auth :** Requis (JWT)
- **Corps (JSON) :** `{"role": "read"}`

#### 4. Retirer un membre
- **URL :** `/api/projects/:id/members/:user_id`
- **Méthode :** `DELETE`
- **Auth :** Requis (JWT)

---

### Routes Synopsis

#### 1. Voir le synopsis
- **URL :** `/api/projects/:id/synopsis`
- **Méthode :** `GET`
- **Auth :** Requis (JWT)

#### 2. Créer ou Mettre à jour le synopsis (Upsert)
- **URL :** `/api/projects/:id/synopsis`
- **Méthode :** `POST`
- **Auth :** Requis (JWT)
- **Corps (JSON) :**
```json
{
  "logline": "Un homme redécouvre le monde.",
  "summary_short": "Synopsis court...",
  "summary_long": "Synopsis détaillé..."
}
```

#### 3. Supprimer le synopsis
- **URL :** `/api/projects/:id/synopsis`
- **Méthode :** `DELETE`
- **Auth :** Requis (JWT)

---

### Routes Métadonnées

#### 1. Scene Settings (Enums)
- **URL :** `/api/metadata/scene-settings`
- **Méthode :** `GET`
- **Réponse :** `["INT", "EXT", "INT_EXT"]`

#### 2. Scene Times of Day (Enums)
- **URL :** `/api/metadata/scene-times-of-day`
- **Méthode :** `GET`
- **Réponse :** `["DAY", "NIGHT", "MORNING", "EVENING", "DAWN", "DUSK"]`

---

### Divers

#### Statut du serveur
- **URL :** `/api/status`
- **Méthode :** `GET`
- **Réponse (200 OK) :** `{"status": "OK"}`