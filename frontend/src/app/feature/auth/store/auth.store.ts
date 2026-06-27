import { inject, Injectable, signal, computed } from '@angular/core';
import { AuthApiService } from '../services/auth-api.service';
import { RegisterInput } from '../models/auth.model';
import { catchError, of } from 'rxjs';

// 1. Définition de la structure de l'état de notre domaine Auth
interface AuthState {
    user: any | null; // Sera enrichi lors de la partie Login / JWT
    isLoading: boolean;
    error: string | null;
    registerSuccess: boolean;
}

@Injectable({
    providedIn: 'root' // Le store est un singleton global pour le domaine auth
})
export class AuthStore {
    private readonly authApiService = inject(AuthApiService);

    // 2. L'état initial (Privé pour empêcher les modifications directes depuis l'extérieur)
    private readonly state = signal<AuthState>({
        user: null,
        isLoading: false,
        error: null,
        registerSuccess: false
    });

    // 3. Les Selectors (Exposés en lecture seule via computed ou accès direct)
    readonly isLoading = computed(() => this.state().isLoading);
    readonly error = computed(() => this.state().error);
    readonly registerSuccess = computed(() => this.state().registerSuccess);

    // 4. Les Actions / Logique métier
    register(input: RegisterInput): void {
        // On passe le statut en chargement et on reset les erreurs précédentes
        this.state.update(s => ({ ...s, isLoading: true, error: null, registerSuccess: false }));

        this.authApiService.register(input).subscribe({
            next: (response) => {
                // Succès de l'inscription côté Rust (201 Created)
                this.state.update(s => ({
                    ...s,
                    isLoading: false,
                    registerSuccess: true
                }));
            },
            error: (err) => {
                // Gestion de l'erreur (ex: 409 Conflict de notre Rust si l'email existe déjà)
                let errorMessage = "Une erreur serveur est survenue.";

                if (err.status === 409) {
                    errorMessage = "Cette adresse email est déjà utilisée.";
                } else if (err.status === 400) {
                    errorMessage = "Données invalides (mot de passe trop court).";
                }

                this.state.update(s => ({
                    ...s,
                    isLoading: false,
                    error: errorMessage
                }));
            }
        });
    }

    // Permet de reset le statut après avoir affiché un message par exemple
    resetStatus(): void {
        this.state.update(s => ({ ...s, error: null, registerSuccess: false }));
    }
}