import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { AuthStore } from '../../../auth/store/auth.store';

@Component({
  selector: 'app-workspace-home',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="space-y-6">
      <header>
        <h1 class="text-3xl font-bold text-slate-900">Bienvenue, {{ userName }} !</h1>
        <p class="text-slate-500 mt-2">Prêt à écrire votre prochain chef-d'œuvre ? Voici un aperçu de votre activité.</p>
      </header>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-8">
        <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
          <div class="w-12 h-12 bg-blue-100 text-blue-600 rounded-lg flex items-center justify-center mb-4">
            <i class="fa-solid fa-clapperboard text-xl"></i>
          </div>
          <h3 class="text-lg font-semibold text-slate-800">Projets actifs</h3>
          <p class="text-3xl font-bold text-slate-900 mt-2">0</p>
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
            <i class="fa-solid fa-star text-xl"></i>
          </div>
          <h3 class="text-lg font-semibold text-slate-800">Favoris</h3>
          <p class="text-3xl font-bold text-slate-900 mt-2">0</p>
        </div>
      </div>
      
      <div class="mt-12 p-8 border-2 border-dashed border-slate-200 rounded-2xl flex flex-col items-center justify-center text-center">
        <div class="w-16 h-16 bg-slate-100 text-slate-400 rounded-full flex items-center justify-center mb-4">
          <i class="fa-solid fa-plus text-2xl"></i>
        </div>
        <h2 class="text-xl font-semibold text-slate-700">Aucun projet récent</h2>
        <p class="text-slate-500 mt-2 max-w-sm">Commencez par créer un nouveau projet pour voir vos statistiques apparaître ici.</p>
        <button class="create-btn">
          Créer mon premier projet
        </button>
      </div>
    </div>
  `,
  styles: [`
    .create-btn {
      margin-top: 1.5rem;
      padding: 0.75rem 1.5rem;
      background-color: var(--color-cinema-dark);
      color: white;
      border-radius: 0.5rem;
      border: none;
      cursor: pointer;
      font-weight: 500;
      transition: background-color 150ms ease;
    }
    .create-btn:hover {
      background-color: var(--color-cinema-deep);
    }
  `]
})
export class WorkspaceHomeComponent {
  private readonly authStore = inject(AuthStore);
  
  get userName(): string {
    const user = this.authStore.currentUser();
    return user?.first_name || user?.email?.split('@')[0] || 'Scénariste';
  }
}
