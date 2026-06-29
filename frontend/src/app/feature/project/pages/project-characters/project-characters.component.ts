import { Component, inject, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute } from '@angular/router';
import { ProjectFeatureService } from '../../services/project-feature.service';
import { CharacterBible } from '../../models/project-features.model';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-project-characters',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="space-y-6">
      <header class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-slate-900">Bible des Personnages</h1>
          <p class="text-slate-500">Définissez vos protagonistes, antagonistes et personnages secondaires.</p>
        </div>
        <button (click)="showCreateForm = true" class="bg-indigo-600 text-white px-4 py-2 rounded-lg hover:bg-indigo-700 transition-colors">
          <i class="fa-solid fa-plus mr-2"></i> Nouveau Personnage
        </button>
      </header>

      @if (showCreateForm) {
        <div class="bg-white p-6 rounded-xl border border-indigo-200 shadow-sm animate-in fade-in slide-in-from-top-4 duration-300">
          <h2 class="text-lg font-bold mb-4">Ajouter un personnage</h2>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="md:col-span-2">
              <label class="block text-sm font-medium text-slate-700 mb-1">Nom complet</label>
              <input [(ngModel)]="newCharacter.full_name" type="text" class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none">
            </div>
            <div>
              <label class="block text-sm font-medium text-slate-700 mb-1">Rôle</label>
              <select [(ngModel)]="newCharacter.character_role" class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none">
                <option value="protagonist">Protagoniste</option>
                <option value="antagonist">Antagoniste</option>
                <option value="mentor">Mentor</option>
                <option value="sidekick">Compagnon</option>
                <option value="love_interest">Intérêt amoureux</option>
                <option value="secondary">Secondaire</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-slate-700 mb-1">URL Avatar (Optionnel)</label>
              <input [(ngModel)]="newCharacter.avatar_url" type="text" class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none" placeholder="https://...">
            </div>
            <div class="md:col-span-2">
              <label class="block text-sm font-medium text-slate-700 mb-1">Description / Bio</label>
              <textarea [(ngModel)]="newCharacter.description" rows="3" class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-indigo-500 outline-none"></textarea>
            </div>
          </div>
          <div class="flex justify-end space-x-3 mt-6">
            <button (click)="showCreateForm = false" class="px-4 py-2 text-slate-600 hover:bg-slate-100 rounded-lg transition-colors">Annuler</button>
            <button (click)="createCharacter()" [disabled]="!newCharacter.full_name" class="bg-indigo-600 text-white px-4 py-2 rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition-colors">Enregistrer</button>
          </div>
        </div>
      }

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        @for (char of characters(); track char.id) {
          <div class="bg-white rounded-xl border border-slate-200 shadow-sm overflow-hidden group hover:shadow-md transition-shadow">
            <div class="h-40 bg-slate-100 relative overflow-hidden">
              @if (char.avatar_url) {
                <img [src]="char.avatar_url" alt="Avatar" class="w-full h-full object-cover">
              } @else {
                <div class="w-full h-full flex items-center justify-center text-slate-300 bg-indigo-50">
                  <i class="fa-solid fa-user-ninja text-5xl"></i>
                </div>
              }
              <div class="absolute top-2 right-2 flex space-x-1 opacity-0 group-hover:opacity-100 transition-opacity">
                 <button (click)="deleteCharacter(char.id)" class="bg-white/90 p-1.5 rounded-md text-red-500 hover:bg-red-50">
                    <i class="fa-solid fa-trash-can text-xs"></i>
                 </button>
              </div>
              <div class="absolute bottom-0 inset-x-0 p-3 bg-gradient-to-t from-black/60 to-transparent">
                <span class="text-[10px] font-bold uppercase tracking-wider text-white/80 bg-indigo-600/80 px-2 py-0.5 rounded">{{ char.character_role }}</span>
              </div>
            </div>
            <div class="p-4">
              <h3 class="font-bold text-slate-900 truncate">{{ char.full_name }}</h3>
              <p class="text-slate-500 text-xs mt-2 line-clamp-3 h-12">{{ char.description || 'Pas de description.' }}</p>
              <div class="mt-4 pt-3 border-t border-slate-50 flex justify-between items-center">
                <button class="text-indigo-600 hover:text-indigo-800 text-xs font-bold uppercase tracking-tight">Voir profil complet</button>
              </div>
            </div>
          </div>
        } @empty {
          <div class="col-span-full py-12 text-center bg-slate-50 rounded-2xl border-2 border-dashed border-slate-200">
            <i class="fa-solid fa-users-rectangle text-4xl text-slate-200 mb-4"></i>
            <h3 class="text-lg font-medium text-slate-600">Aucun personnage répertorié</h3>
            <p class="text-slate-400 mt-1">Donnez vie à vos histoires en créant vos personnages.</p>
          </div>
        }
      </div>
    </div>
  `
})
export class ProjectCharactersComponent implements OnInit {
  private readonly route = inject(ActivatedRoute);
  private readonly projectFeatureService = inject(ProjectFeatureService);

  projectId: string = '';
  characters = signal<CharacterBible[]>([]);
  showCreateForm = false;
  
  newCharacter = {
    full_name: '',
    character_role: 'protagonist',
    avatar_url: '',
    description: ''
  };

  ngOnInit(): void {
    this.projectId = this.route.snapshot.paramMap.get('id') || '';
    if (this.projectId) {
      this.loadCharacters();
    }
  }

  loadCharacters(): void {
    this.projectFeatureService.getCharacters(this.projectId).subscribe(chars => {
      this.characters.set(chars);
    });
  }

  createCharacter(): void {
    this.projectFeatureService.createCharacter({
      project_id: this.projectId,
      full_name: this.newCharacter.full_name,
      character_role: this.newCharacter.character_role,
      avatar_url: this.newCharacter.avatar_url || undefined,
      description: this.newCharacter.description || undefined
    }).subscribe(char => {
      this.characters.update(current => [...current, char]);
      this.resetForm();
    });
  }

  deleteCharacter(id: string): void {
    if (confirm('Voulez-vous vraiment supprimer ce personnage ?')) {
      this.projectFeatureService.deleteCharacter(id).subscribe(() => {
        this.characters.update(current => current.filter(c => c.id !== id));
      });
    }
  }

  private resetForm(): void {
    this.newCharacter = { 
      full_name: '', 
      character_role: 'protagonist', 
      avatar_url: '', 
      description: '' 
    };
    this.showCreateForm = false;
  }
}
