import { inject, Injectable, signal, computed } from '@angular/core';
import { ProjectApiService } from '../services/project-api.service';
import { CreateProjectInput, ProjectWithRole, ShootingTechnique } from '../models/project.model';
import { Router } from '@angular/router';

interface ProjectState {
    projects: ProjectWithRole[];
    shootingTechniques: ShootingTechnique[];
    isLoading: boolean;
    error: string | null;
}

@Injectable({
    providedIn: 'root'
})
export class ProjectStore {
    private readonly projectApiService = inject(ProjectApiService);
    private readonly router = inject(Router);

    private readonly state = signal<ProjectState>({
        projects: [],
        shootingTechniques: [],
        isLoading: false,
        error: null
    });

    readonly projects = computed(() => this.state().projects);
    readonly shootingTechniques = computed(() => this.state().shootingTechniques);
    readonly shootingTechniquesMap = computed(() => {
        const map = new Map<string, ShootingTechnique>();
        this.state().shootingTechniques.forEach(t => map.set(t.id, t));
        return map;
    });
    readonly projectCount = computed(() => this.state().projects.length);
    readonly isLoading = computed(() => this.state().isLoading);
    readonly error = computed(() => this.state().error);
    readonly lastModifiedProject = computed(() => {
        const projects = this.state().projects;
        if (projects.length === 0) return null;
        return projects.reduce((prev, current) => 
            (new Date(prev.updated_at) > new Date(current.updated_at)) ? prev : current
        );
    });

    loadProjects(): void {
        this.state.update(s => ({ ...s, isLoading: true, error: null }));
        this.projectApiService.getProjects().subscribe({
            next: (projects) => {
                this.state.update(s => ({ ...s, projects, isLoading: false }));
            },
            error: (err) => {
                this.state.update(s => ({ ...s, isLoading: false, error: 'Erreur lors du chargement des projets' }));
            }
        });
    }

    loadShootingTechniques(): void {
        if (this.state().shootingTechniques.length > 0) return;

        this.projectApiService.getShootingTechniques().subscribe({
            next: (techniques) => {
                this.state.update(s => ({ ...s, shootingTechniques: techniques }));
            },
            error: (err) => {
                console.error('Erreur lors du chargement des techniques', err);
            }
        });
    }

    createProject(input: CreateProjectInput): void {
        this.state.update(s => ({ ...s, isLoading: true, error: null }));
        this.projectApiService.createProject(input).subscribe({
            next: (project) => {
                // On recharge tout pour avoir les rôles corrects et l'ordre
                this.loadProjects();
                this.router.navigate(['/dashboard/workspace']);
            },
            error: (err) => {
                this.state.update(s => ({ ...s, isLoading: false, error: 'Erreur lors de la création du projet' }));
            }
        });
    }
}
