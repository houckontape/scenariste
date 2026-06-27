import { Component, OnInit, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms'; // <-- Import indispensable pour les formulaires

@Component({
    selector: 'app-root',
    standalone: true,
    imports: [CommonModule, FormsModule], // <-- On ajoute FormsModule ici
    template: `
        <div style="text-align:center; margin-top: 50px; font-family: sans-serif;">
            <h1>Mon SaaS Angular + Rust 🦀</h1>

            <div *ngIf="apiData" style="margin-bottom: 30px; color: green;">
                <p>Serveur Rust v{{ apiData.version }} : En ligne 🟢</p>
            </div>

            <div style="padding: 20px; border: 1px solid #ccc; display: inline-block; border-radius: 8px; text-align: left;">
                <h3>Créer un compte SaaS</h3>

                <div style="margin-bottom: 10px;">
                    <label>Email : </label><br>
                    <input type="email" [(ngModel)]="email" style="width: 250px; padding: 5px;">
                </div>

                <div style="margin-bottom: 15px;">
                    <label>Nom de l'entreprise : </label><br>
                    <input type="text" [(ngModel)]="entreprise" style="width: 250px; padding: 5px;">
                </div>

                <button (click)="creerCompte()" style="padding: 7px 15px; background-color: #df5b24; color: white; border: none; border-radius: 4px; cursor: pointer;">
                    S'inscrire sur le SaaS
                </button>

                <div *ngIf="serverFeedback" style="margin-top: 15px; font-weight: bold; color: #2e7d32;">
                    {{ serverFeedback }}
                </div>
            </div>
        </div>
    `
})
export class AppComponent implements OnInit {
    private http = inject(HttpClient);

    apiData: any;
    email: string = '';
    entreprise: string = '';
    serverFeedback: string = '';

    ngOnInit() {
        this.http.get('http://127.0.0.1:3000/api/status')
            .subscribe(data => this.apiData = data);
    }

    creerCompte() {
        const payload = {
            email: this.email,
            entreprise: this.entreprise
        };

        // On envoie les données en POST au serveur Rust
        this.http.post('http://127.0.0.1:3000/api/register', payload)
            .subscribe({
                next: (res: any) => {
                    this.serverFeedback = res.message;
                },
                error: (err) => {
                    console.error(err);
                    this.serverFeedback = "Erreur lors de l'inscription.";
                }
            });
    }
}