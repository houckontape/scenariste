import { Routes } from '@angular/router';

export const routes: Routes = [
    {
        path: '',
        redirectTo: 'auth/register',
        pathMatch: 'full'
    },
    {
        path: 'auth',
        // On charge le fichier complet de routes de la feature Auth à la demande
        loadChildren: () => import('./feature/auth/auth.routes').then(m => m.AUTH_ROUTES)
    },
    {
        path: '**',
        redirectTo: 'auth/register' // Redirection générique pour le moment
    }
];