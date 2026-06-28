import { inject, Injectable, signal, computed } from '@angular/core';
import { AuthApiService } from '../services/auth-api.service';
import { LoginInput, RegisterInput, User } from '../models/auth.model';
import { Router } from '@angular/router';

// 1. Définition de la structure de l'état de notre domaine Auth
interface AuthState {
    user: User | null;
    token: string | null;
    isLoading: boolean;
    error: string | null;
    registerSuccess: boolean;
}

@Injectable({
    providedIn: 'root' // Le store est un singleton global pour le domaine auth
})
export class AuthStore {
    private readonly authApiService = inject(AuthApiService);
    private readonly router = inject(Router);

    private readonly TOKEN_KEY = 'rustover_token';
    private readonly USER_KEY = 'rustover_user';

    // 2. L'état initial
    private readonly state = signal<AuthState>({
        user: this.getStoredUser(),
        token: localStorage.getItem(this.TOKEN_KEY),
        isLoading: false,
        error: null,
        registerSuccess: false
    });

    // 3. Les Selectors
    readonly currentUser = computed(() => this.state().user);
    readonly isAuthenticated = computed(() => !!this.state().token);
    readonly isLoading = computed(() => this.state().isLoading);
    readonly error = computed(() => this.state().error);
    readonly registerSuccess = computed(() => this.state().registerSuccess);

    // 4. Les Actions
    login(input: LoginInput): void {
        this.state.update(s => ({ ...s, isLoading: true, error: null }));

        this.authApiService.login(input).subscribe({
            next: (response) => {
                this.saveSession(response.token, response.user);
                this.state.update(s => ({
                    ...s,
                    user: response.user,
                    token: response.token,
                    isLoading: false
                }));
                this.router.navigate(['/dashboard']);
            },
            error: (err) => {
                let errorMessage = "Identifiants invalides.";
                if (err.status === 0) errorMessage = "Le serveur ne répond pas.";
                
                this.state.update(s => ({
                    ...s,
                    isLoading: false,
                    error: errorMessage
                }));
            }
        });
    }

    register(input: RegisterInput): void {
        this.state.update(s => ({ ...s, isLoading: true, error: null, registerSuccess: false }));

        this.authApiService.register(input).subscribe({
            next: () => {
                this.state.update(s => ({
                    ...s,
                    isLoading: false,
                    registerSuccess: true
                }));
            },
            error: (err) => {
                let errorMessage = "Une erreur serveur est survenue.";
                if (err.status === 409) errorMessage = "Cette adresse email est déjà utilisée.";
                else if (err.status === 400) errorMessage = "Données invalides.";

                this.state.update(s => ({ ...s, isLoading: false, error: errorMessage }));
            }
        });
    }

    logout(): void {
        localStorage.removeItem(this.TOKEN_KEY);
        localStorage.removeItem(this.USER_KEY);
        this.state.update(s => ({ ...s, user: null, token: null }));
        this.router.navigate(['/auth/login']);
    }

    private saveSession(token: string, user: User): void {
        localStorage.setItem(this.TOKEN_KEY, token);
        localStorage.setItem(this.USER_KEY, JSON.stringify(user));
    }

    private getStoredUser(): User | null {
        const userJson = localStorage.getItem(this.USER_KEY);
        return userJson ? JSON.parse(userJson) : null;
    }

    resetStatus(): void {
        this.state.update(s => ({ ...s, error: null, registerSuccess: false }));
    }
}