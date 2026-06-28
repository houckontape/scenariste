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
- **Body (JSON) :**
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
- **Body (JSON) :**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```
- **Réponse (200 OK) :**
```json
{
  "token": "votre.jwt.token",
  "token_type": "Bearer"
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
    "created_at": "timestamp",
    "updated_at": "timestamp"
  }
}
```

#### 4. Mise à jour du profil
- **URL :** `/api/auth/profile`
- **Méthode :** `POST`
- **Auth :** Requis (JWT)
- **Body (JSON) :**
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

### Routes Articles (Brainstorming Notes)

#### 1. Lister les articles
- **URL :** `/api/articles`
- **Méthode :** `GET`
- **Réponse (200 OK) :** `Array<Article>`

#### 2. Créer un article
- **URL :** `/api/articles`
- **Méthode :** `POST`
- **Auth :** Requis (JWT - Rôle `super_admin`)
- **Body (JSON) :**
```json
{
  "project_id": "uuid",
  "title": "Mon Note",
  "content": "Contenu en Markdown",
  "tags": ["tag1", "tag2"]
}
```
- **Réponse (201 Created) :** L'article créé.

---

### Divers

#### Statut du serveur
- **URL :** `/api/status`
- **Méthode :** `GET`
- **Réponse (200 OK) :** `{"status": "OK"}`