import { Routes } from '@angular/router';

export const AUTH_ROUTES: Routes = [
    {
        path: '',
        redirectTo: 'register',
        pathMatch: 'full'
    },
    {
        path: 'register',
        // Chargement différé du composant de la page d'inscription
        loadComponent: () => import('./pages/register-page/register-page.component')
            .then(m => m.RegisterPageComponent)
    }
    // Nous pourrons ajouter la route 'login' ici plus tard !
];