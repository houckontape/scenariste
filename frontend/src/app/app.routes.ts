import { Routes } from '@angular/router';
import { authGuard } from './core/guards/auth.guard';
import { DashboardLayoutComponent } from './feature/dashboard/layouts/dashboard-layout/dashboard-layout.component';
import { WorkspaceHomeComponent } from './feature/dashboard/pages/workspace-home/workspace-home.component';
import { ProfileComponent } from './feature/dashboard/pages/profile/profile.component';

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
        loadChildren: () => import('./feature/auth/auth.routes').then(m => m.AUTH_ROUTES)
    },
    {
        path: 'dashboard',
        component: DashboardLayoutComponent,
        canActivate: [authGuard],
        children: [
            { path: '', redirectTo: 'workspace', pathMatch: 'full' },
            { path: 'workspace', component: WorkspaceHomeComponent },
            { path: 'profile', component: ProfileComponent },
            // Les autres routes enfants (projects, brainstorming) pourront être ajoutées ici
        ]
    },
    {
        path: '**',
        redirectTo: 'home'
    }
];