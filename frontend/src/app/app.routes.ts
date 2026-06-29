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
            { 
                path: 'projects', 
                children: [
                    { path: '', redirectTo: '/dashboard/workspace', pathMatch: 'full' },
                    { 
                        path: 'create', 
                        loadComponent: () => import('./feature/project/pages/project-create/project-create.component').then(m => m.ProjectCreateComponent) 
                    },
                    {
                        path: ':id',
                        children: [
                            { path: '', redirectTo: 'notes', pathMatch: 'full' },
                            { 
                                path: 'notes', 
                                loadComponent: () => import('./feature/project/pages/project-notes/project-notes.component').then(m => m.ProjectNotesComponent) 
                            },
                            { 
                                path: 'characters', 
                                loadComponent: () => import('./feature/project/pages/project-characters/project-characters.component').then(m => m.ProjectCharactersComponent) 
                            },
                            { 
                                path: 'structure', 
                                loadComponent: () => import('./feature/project/pages/project-structure/project-structure.component').then(m => m.ProjectStructureComponent) 
                            }
                        ]
                    }
                ]
            },
            { path: 'profile', component: ProfileComponent },
            // Les autres routes enfants (projects, brainstorming) pourront être ajoutées ici
        ]
    },
    {
        path: '**',
        redirectTo: 'home'
    }
];