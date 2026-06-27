import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { AuthResponse, RegisterInput } from '../models/auth.model';

@Injectable({
    providedIn: 'root' // Disponible dans toute l'application (Tree-shakable)
})
export class AuthApiService {
    // Remplacement de l'ancien constructeur par la fonction inject() (Standard Angular moderne)
    private readonly http = inject(HttpClient);
    private readonly apiUrl = 'http://localhost:3000/api/auth';

    register(input: RegisterInput): Observable<AuthResponse> {
        return this.http.post<AuthResponse>(`${this.apiUrl}/register`, input);
    }
}