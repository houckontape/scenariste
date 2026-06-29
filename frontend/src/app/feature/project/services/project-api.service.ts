import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Project, ProjectWithRole, CreateProjectInput, ShootingTechnique } from '../models/project.model';

@Injectable({
    providedIn: 'root'
})
export class ProjectApiService {
    private readonly http = inject(HttpClient);
    private readonly apiUrl = 'http://localhost:3000/api/projects';
    private readonly metadataUrl = 'http://localhost:3000/api/metadata';

    getProjects(): Observable<ProjectWithRole[]> {
        return this.http.get<ProjectWithRole[]>(this.apiUrl);
    }

    getProject(id: string): Observable<ProjectWithRole> {
        return this.http.get<ProjectWithRole>(`${this.apiUrl}/${id}`);
    }

    createProject(input: CreateProjectInput): Observable<Project> {
        return this.http.post<Project>(this.apiUrl, input);
    }

    deleteProject(id: string): Observable<void> {
        return this.http.delete<void>(`${this.apiUrl}/${id}`);
    }

    getShootingTechniques(): Observable<ShootingTechnique[]> {
        return this.http.get<ShootingTechnique[]>(`${this.metadataUrl}/shooting-techniques`);
    }
}
