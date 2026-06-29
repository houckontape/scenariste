import { Component, inject, OnInit } from '@angular/core';
import { CommonModule, DatePipe } from '@angular/common';
import { AuthStore } from '../../../auth/store/auth.store';
import { ProjectStore } from '../../../project/store/project.store';
import { RouterLink } from '@angular/router';

@Component({
  selector: 'app-workspace-home',
  standalone: true,
  imports: [CommonModule, RouterLink],
  providers: [DatePipe],
  template: `
    <div class="space-y-6">
      <header class="flex justify-between items-start">
        <div>
          <h1 class="text-3xl font-bold text-slate-900">Bienvenue, {{ userName }} !</h1>
          <p class="text-slate-500 mt-2">Prêt à écrire votre prochain chef-d'œuvre ? Voici un aperçu de votre activité.</p>
        </div>
        <a routerLink="/dashboard/projects/create" class="bg-slate-900 text-white px-4 py-2 rounded-lg font-medium hover:bg-slate-800 transition-colors">
          <i class="fa-solid fa-plus mr-2"></i> Nouveau Projet
        </a>
      </header>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-8">
        <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
          <div class="w-12 h-12 bg-blue-100 text-blue-600 rounded-lg flex items-center justify-center mb-4">
            <i class="fa-solid fa-clapperboard text-xl"></i>
          </div>
          <h3 class="text-lg font-semibold text-slate-800">Projets actifs</h3>
          <p class="text-3xl font-bold text-slate-900 mt-2">{{ projectCount() }}</p>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
          <div class="w-12 h-12 bg-purple-100 text-purple-600 rounded-lg flex items-center justify-center mb-4">
            <i class="fa-solid fa-note-sticky text-xl"></i>
          </div>
          <h3 class="text-lg font-semibold text-slate-800">Notes</h3>
          <p class="text-3xl font-bold text-slate-900 mt-2">0</p>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
          <div class="w-12 h-12 bg-amber-100 text-amber-600 rounded-lg flex items-center justify-center mb-4">
            <i class="fa-solid fa-clock-rotate-left text-xl"></i>
          </div>
          <h3 class="text-lg font-semibold text-slate-800">Dernière modif.</h3>
          <p class="text-sm font-medium text-slate-600 mt-2">
            @if (lastModifiedProject()) {
              {{ lastModifiedProject()?.updated_at | date:'dd/MM/yyyy HH:mm' }}
            } @else {
              Aucune activité
            }
          </p>
        </div>
      </div>

      @if (isLoading()) {
        <div class="flex justify-center py-12">
          <i class="fa-solid fa-circle-notch animate-spin text-3xl text-slate-300"></i>
        </div>
      } @else if (projects().length > 0) {
        <div class="mt-8">
          <h2 class="text-xl font-bold text-slate-800 mb-4">Vos Projets Récents</h2>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            @for (project of projects(); track project.id) {
              <div class="bg-white border border-slate-200 rounded-xl overflow-hidden hover:shadow-md transition-shadow group">
                <div class="h-32 bg-slate-100 flex items-center justify-center border-b border-slate-100">
                   <i class="fa-solid fa-film text-4xl text-slate-300 group-hover:scale-110 transition-transform"></i>
                </div>
                <div class="p-4">
                  <div class="flex justify-between items-start mb-2">
                    <div class="flex flex-col">
                      <span class="text-xs font-semibold uppercase tracking-wider text-slate-400">{{ project.p_type }}</span>
                      @if (project.shooting_technique_id) {
                        <span class="text-[10px] text-blue-500 font-medium">{{ getTechniqueName(project.shooting_technique_id) }}</span>
                      }
                    </div>
                    @if (project.is_private) {
                      <i class="fa-solid fa-lock text-slate-400 text-xs" title="Privé"></i>
                    }
                  </div>
                  <h4 class="font-bold text-slate-900 truncate">{{ project.title }}</h4>
                  <p class="text-slate-500 text-sm mt-1 line-clamp-2">{{ project.description || 'Aucune description' }}</p>
                  <div class="mt-4 flex justify-between items-center">
                    <span class="text-xs text-slate-400">Màj le {{ project.updated_at | date:'dd/MM/yy' }}</span>
                    <a [routerLink]="['/dashboard/projects', project.id]" class="text-blue-600 hover:text-blue-800 text-sm font-semibold">
                      Détails <i class="fa-solid fa-arrow-right ml-1"></i>
                    </a>
                  </div>
                </div>
              </div>
            }
          </div>
        </div>
      } @else {
        <div class="mt-12 p-8 border-2 border-dashed border-slate-200 rounded-2xl flex flex-col items-center justify-center text-center">
          <div class="w-16 h-16 bg-slate-100 text-slate-400 rounded-full flex items-center justify-center mb-4">
            <i class="fa-solid fa-plus text-2xl"></i>
          </div>
          <h2 class="text-xl font-semibold text-slate-700">Aucun projet récent</h2>
          <p class="text-slate-500 mt-2 max-w-sm">Commencez par créer un nouveau projet pour voir vos statistiques apparaître ici.</p>
          <a routerLink="/dashboard/projects/create" class="create-btn">
            Créer mon premier projet
          </a>
        </div>
      }
    </div>
  `,
  styles: [`
    .create-btn {
      display: inline-block;
      margin-top: 1.5rem;
      padding: 0.75rem 1.5rem;
      background-color: var(--color-cinema-dark);
      color: white;
      border-radius: 0.5rem;
      border: none;
      cursor: pointer;
      font-weight: 500;
      text-decoration: none;
      transition: background-color 150ms ease;
    }
    .create-btn:hover {
      background-color: var(--color-cinema-deep);
    }
  `]
})
export class WorkspaceHomeComponent implements OnInit {
  private readonly authStore = inject(AuthStore);
  private readonly projectStore = inject(ProjectStore);
  
  readonly projects = this.projectStore.projects;
  readonly shootingTechniquesMap = this.projectStore.shootingTechniquesMap;
  readonly projectCount = this.projectStore.projectCount;
  readonly lastModifiedProject = this.projectStore.lastModifiedProject;
  readonly isLoading = this.projectStore.isLoading;

  ngOnInit(): void {
    this.projectStore.loadProjects();
    this.projectStore.loadShootingTechniques();
  }

  getTechniqueName(id?: string): string {
    if (!id) return '';
    return this.shootingTechniquesMap().get(id)?.name || '';
  }

  get userName(): string {
    const user = this.authStore.currentUser();
    return user?.first_name || user?.email?.split('@')[0] || 'Scénariste';
  }
}
