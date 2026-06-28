import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { AuthResponse, LoginInput, RegisterInput, RegisterResponse } from '../models/auth.model';

@Injectable({
    providedIn: 'root' // Disponible dans toute l'application (Tree-shakable)
})
export class AuthApiService {
    // Remplacement de l'ancien constructeur par la fonction inject() (Standard Angular moderne)
    private readonly http = inject(HttpClient);
    private readonly apiUrl = 'http://localhost:3000/api/auth';

    register(input: RegisterInput): Observable<RegisterResponse> {
        return this.http.post<RegisterResponse>(`${this.apiUrl}/register`, input);
    }

    login(input: LoginInput): Observable<AuthResponse> {
        return this.http.post<AuthResponse>(`${this.apiUrl}/login`, input);
    }
}