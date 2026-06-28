import { Routes } from '@angular/router';

export const routes: Routes = [
    {
        path: '',
        redirectTo: 'home',
        pathMatch: 'full'
    },
    {
        path: 'home',
        loadChildren: () => import('./feature/home/home.routes').then(m => m.HOME_ROUTES)
    },
    {
        path: 'auth',
        // On charge le fichier complet de routes de la feature Auth à la demande
        loadChildren: () => import('./feature/auth/auth.routes').then(m => m.AUTH_ROUTES)
    },
    {
        path: '**',
        redirectTo: 'home' // Redirection générique pour le moment
    }
];