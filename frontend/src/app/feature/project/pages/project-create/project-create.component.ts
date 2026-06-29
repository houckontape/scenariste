import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { Router, RouterLink } from '@angular/router';
import { ProjectStore } from '../../store/project.store';
import { ProjectType } from '../../models/project.model';

@Component({
  selector: 'app-project-create',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule, RouterLink],
  template: `
    <div class="max-w-2xl mx-auto py-8">
      <header class="mb-8">
        <h1 class="text-3xl font-bold text-slate-900">Nouveau Projet</h1>
        <p class="text-slate-500 mt-2">Donnez vie à votre prochaine histoire.</p>
      </header>

      <form [formGroup]="projectForm" (ngSubmit)="onSubmit()" class="bg-white p-8 rounded-xl shadow-sm border border-slate-200 space-y-6">
        <div class="space-y-2">
          <label for="title" class="block text-sm font-medium text-slate-700">Titre du projet</label>
          <input
            id="title"
            type="text"
            formControlName="title"
            class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all"
            placeholder="Ex: Le Mystère de l'Ombre"
          />
          @if (projectForm.get('title')?.touched && projectForm.get('title')?.invalid) {
            <p class="text-red-500 text-xs mt-1">Le titre est requis (min 3 caractères).</p>
          }
        </div>

        <div class="space-y-2">
          <label for="p_type" class="block text-sm font-medium text-slate-700">Type de projet</label>
          <select
            id="p_type"
            formControlName="p_type"
            class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all"
          >
            <option value="movie">Film</option>
            <option value="series">Série</option>
            <option value="animation">Animation</option>
            <option value="novel">Roman</option>
          </select>
        </div>

        <div class="space-y-2">
          <label for="shooting_technique_id" class="block text-sm font-medium text-slate-700">Technique de tournage</label>
          <select
            id="shooting_technique_id"
            formControlName="shooting_technique_id"
            class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all"
          >
            <option value="" disabled>Sélectionnez une technique</option>
            @for (tech of shootingTechniques(); track tech.id) {
              <option [value]="tech.id">{{ tech.name }}</option>
            }
          </select>
          @if (projectForm.get('shooting_technique_id')?.touched && projectForm.get('shooting_technique_id')?.invalid) {
            <p class="text-red-500 text-xs mt-1">Veuillez choisir une technique.</p>
          }
        </div>

        <div class="space-y-2">
          <label for="description" class="block text-sm font-medium text-slate-700">Description (Optionnel)</label>
          <textarea
            id="description"
            formControlName="description"
            rows="4"
            class="w-full px-4 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all"
            placeholder="Bref résumé de votre projet..."
          ></textarea>
        </div>

        <div class="flex items-center space-x-3 p-4 bg-slate-50 rounded-lg">
          <input
            id="is_private"
            type="checkbox"
            formControlName="is_private"
            class="w-4 h-4 text-blue-600 border-slate-300 rounded focus:ring-blue-500"
          />
          <label for="is_private" class="text-sm text-slate-700 font-medium">Projet privé</label>
        </div>

        <div class="flex items-center justify-end space-x-4 pt-4">
          <a routerLink="/dashboard/workspace" class="px-6 py-2 text-slate-600 hover:text-slate-800 font-medium">Annuler</a>
          <button
            type="submit"
            [disabled]="projectForm.invalid || isLoading()"
            class="px-6 py-2 bg-slate-900 text-white rounded-lg font-medium hover:bg-slate-800 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            @if (isLoading()) {
              <i class="fa-solid fa-circle-notch animate-spin mr-2"></i> Création...
            } @else {
              Créer le projet
            }
          </button>
        </div>
      </form>
    </div>
  `,
  styles: []
})
export class ProjectCreateComponent {
  private readonly fb = inject(FormBuilder);
  private readonly projectStore = inject(ProjectStore);
  
  readonly isLoading = this.projectStore.isLoading;
  readonly shootingTechniques = this.projectStore.shootingTechniques;

  projectForm = this.fb.group({
    title: ['', [Validators.required, Validators.minLength(3)]],
    description: [''],
    p_type: ['movie' as ProjectType, [Validators.required]],
    shooting_technique_id: ['', [Validators.required]],
    is_private: [true]
  });

  constructor() {
    this.projectStore.loadShootingTechniques();
  }

  onSubmit(): void {
    if (this.projectForm.valid) {
      this.projectStore.createProject(this.projectForm.value as any);
    }
  }
}
