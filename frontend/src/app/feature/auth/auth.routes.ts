import { Routes } from '@angular/router';

export const AUTH_ROUTES: Routes = [
    {
        path: '',
        redirectTo: 'login',
        pathMatch: 'full'
    },
    {
        path: 'login',
        loadComponent: () => import('./pages/login-page/login-page.component')
            .then(m => m.LoginPageComponent)
    },
    {
        path: 'register',
        // Chargement différé du composant de la page d'inscription
        loadComponent: () => import('./pages/register-page/register-page.component')
            .then(m => m.RegisterPageComponent)
    }
];