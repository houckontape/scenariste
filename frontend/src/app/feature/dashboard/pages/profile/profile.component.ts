import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { AuthStore } from '../../../auth/store/auth.store';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-profile',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="max-w-3xl mx-auto space-y-8">
      <header>
        <h1 class="text-3xl font-bold text-slate-900">Mon Profil</h1>
        <p class="text-slate-500 mt-2">Gérez vos informations personnelles et votre compte.</p>
      </header>

      <section class="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
        <div class="p-6 border-b border-slate-100">
          <h2 class="text-lg font-semibold text-slate-800">Informations générales</h2>
        </div>
        
        <div class="p-8 space-y-6">
          <div class="flex flex-col md-flex-row gap-8 items-start">
             <div class="flex flex-col items-center gap-4">
                <div class="relative">
                  @if (user()?.avatar_url) {
                    <img [src]="user()?.avatar_url" alt="Avatar" class="w-32 h-32 rounded-full border-4 border-slate-50 object-cover shadow-md">
                  } @else {
                    <div class="w-32 h-32 rounded-full border-4 border-slate-50 bg-slate-100 flex items-center justify-center shadow-md text-slate-400">
                      <i class="fa-solid fa-user text-5xl"></i>
                    </div>
                  }
                  <button class="absolute bottom-1 right-1 bg-cinema-dark text-white p-2 rounded-full shadow-lg hover:bg-cinema-deep transition-colors">
                    <i class="fa-solid fa-camera"></i>
                  </button>
                </div>
                <p class="text-xs text-slate-400 text-center">Format JPG ou PNG conseillé.</p>
             </div>

             <div class="flex-grow grid grid-cols-1 md-grid-cols-2 gap-4">
                <div class="space-y-2">
                  <label class="text-sm font-medium text-slate-700">Prénom</label>
                  <input type="text" [(ngModel)]="profileData.first_name" placeholder="Ex: Jean" class="form-input">
                </div>
                <div class="space-y-2">
                  <label class="text-sm font-medium text-slate-700">Nom</label>
                  <input type="text" [(ngModel)]="profileData.last_name" placeholder="Ex: Dupont" class="form-input">
                </div>
                <div class="md-col-span-2 space-y-2">
                  <label class="text-sm font-medium text-slate-700">Biographie</label>
                  <textarea [(ngModel)]="profileData.bio" rows="4" placeholder="Parlez-nous de votre parcours de scénariste..." class="form-input resize-none"></textarea>
                </div>
             </div>
          </div>
        </div>

        <div class="p-6 bg-slate-50 border-t border-slate-100 flex justify-end gap-3">
          <button class="px-4 py-2 text-slate-600 hover:text-slate-800 font-medium">Annuler</button>
          <button class="px-6 py-2 bg-cinema-dark text-white rounded-lg hover:bg-cinema-deep transition-colors font-medium">Sauvegarder</button>
        </div>
      </section>

      <section class="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
        <div class="p-6 border-b border-slate-100">
          <h2 class="text-lg font-semibold text-slate-800">Compte</h2>
        </div>
        <div class="p-6 space-y-4">
          <div class="flex justify-between items-center py-2">
            <div>
              <p class="text-sm font-medium text-slate-700">Adresse email</p>
              <p class="text-slate-500">{{ user()?.email }}</p>
            </div>
            <button class="text-cinema-dark hover-underline text-sm font-medium">Modifier</button>
          </div>
          <div class="h-px bg-slate-100"></div>
          <div class="flex justify-between items-center py-2">
            <div>
              <p class="text-sm font-medium text-slate-700">Type de compte</p>
              <p class="text-slate-500 uppercase tracking-tight text-xs bg-amber-100 text-amber-700 px-2 py-0.5 rounded inline-block font-bold mt-1">
                {{ user()?.role }}
              </p>
            </div>
            <button class="text-cinema-dark hover-underline text-sm font-medium">Gérer l'abonnement</button>
          </div>
        </div>
      </section>
    </div>
  `,
  styles: [`
    .form-input {
      width: 100%;
      padding: 0.5rem 1rem;
      border: 1px solid #e2e8f0;
      border-radius: 0.5rem;
      outline: none;
      transition: all 150ms ease;
      font-family: inherit;
      font-size: 0.875rem;
    }
    .form-input:focus {
      border-color: var(--color-cinema-accent);
      box-shadow: 0 0 0 2px rgba(147, 181, 198, 0.2);
    }
    .resize-none { resize: none; }
    .md-col-span-2 { grid-column: span 2 / span 2; }
    .md-flex-row { flex-direction: row; }
    @media (min-width: 768px) {
      .md-flex-row { flex-direction: row; }
      .md-grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
      .md-col-span-2 { grid-column: span 2 / span 2; }
    }
    .md-flex-row.items-start { align-items: flex-start; }
    .text-slate-600 { color: #475569; }
    .text-amber-100 { background-color: #fef3c7; }
    .px-2 { padding-left: 0.5rem; padding-right: 0.5rem; }
    .py-0.5 { padding-top: 0.125rem; padding-bottom: 0.125rem; }
    .tracking-tight { letter-spacing: -0.025em; }
    .text-cinema-dark { color: var(--color-cinema-dark); }
    .hover-underline {
      background: none;
      border: none;
      padding: 0;
      cursor: pointer;
    }
    .hover-underline:hover { text-decoration: underline; }
    .md-col-span-2 { grid-column: span 2 / span 2; }
    .items-start { align-items: flex-start; }
    .gap-8 { gap: 2rem; }
    .gap-4 { gap: 1rem; }
    .w-32 { width: 8rem; }
    .h-32 { height: 8rem; }
    .border-4 { border-width: 4px; border-style: solid; }
    .border-slate-50 { border-color: #f8fafc; }
    .bg-slate-50 { background-color: #f8fafc; }
    .bg-slate-100 { background-color: #f1f5f9; }
    .text-5xl { font-size: 3rem; }
    .text-amber-700 { color: #b45309; }
    .bottom-1 { bottom: 0.25rem; }
    .right-1 { right: 0.25rem; }
    .p-2 { padding: 0.5rem; }
  `]
})
export class ProfileComponent {
  private readonly authStore = inject(AuthStore);
  readonly user = this.authStore.currentUser;

  profileData = {
    first_name: this.user()?.first_name ?? '',
    last_name: this.user()?.last_name ?? '',
    bio: '', // Ajouté pour le test, pas encore dans le modèle User
    avatar_url: this.user()?.avatar_url ?? ''
  };
}
