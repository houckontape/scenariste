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