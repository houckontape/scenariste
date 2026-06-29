import { Component, inject, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute } from '@angular/router';
import { ProjectFeatureService } from '../../services/project-feature.service';
import { BrainstormingNote } from '../../models/project-features.model';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-project-notes',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="space-y-6">
      <header class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-slate-900">Notes & Brainstorming</h1>
          <p class="text-slate-500">Capturez vos idées, thèmes et recherches.</p>
        </div>
        <button (click)="showCreateForm = true" class="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors">
          <i class="fa-solid fa-plus mr-2"></i> Nouvelle Note
        </button>
      </header>

      @if (showCreateForm) {
        <div class="bg-white p-6 rounded-xl border border-blue-200 shadow-sm animate-in fade-in slide-in-from-top-4 duration-300">
          <h2 class="text-lg font-bold mb-4">Créer une note</h2>
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-slate-700 mb-1">Titre</label>
              <input [(ngModel)]="newNote.title" type="text" class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none" placeholder="Ex: Idée pour l'acte 2">
            </div>
            <div>
              <label class="block text-sm font-medium text-slate-700 mb-1">Contenu (Markdown)</label>
              <textarea [(ngModel)]="newNote.content" rows="4" class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none" placeholder="Décrivez votre idée..."></textarea>
            </div>
            <div>
              <label class="block text-sm font-medium text-slate-700 mb-1">Tags (séparés par des virgules)</label>
              <input [(ngModel)]="tagsInput" type="text" class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-blue-500 outline-none" placeholder="Ex: idée, personnage, dialogue">
            </div>
            <div class="flex justify-end space-x-3">
              <button (click)="showCreateForm = false" class="px-4 py-2 text-slate-600 hover:bg-slate-100 rounded-lg transition-colors">Annuler</button>
              <button (click)="createNote()" [disabled]="!newNote.title || !newNote.content" class="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 disabled:opacity-50 transition-colors">Enregistrer</button>
            </div>
          </div>
        </div>
      }

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        @for (note of notes(); track note.id) {
          <div class="bg-white p-5 rounded-xl border border-slate-200 shadow-sm hover:shadow-md transition-shadow relative group">
            <div class="flex justify-between items-start mb-3">
              <h3 class="font-bold text-slate-900 leading-tight">{{ note.title }}</h3>
              <button (click)="deleteNote(note.id)" class="text-slate-300 hover:text-red-500 transition-colors opacity-0 group-hover:opacity-100">
                <i class="fa-solid fa-trash-can"></i>
              </button>
            </div>
            <p class="text-slate-600 text-sm line-clamp-4 mb-4 whitespace-pre-wrap">{{ note.content }}</p>
            <div class="flex flex-wrap gap-2 mt-auto">
              @for (tag of note.tags; track tag) {
                <span class="px-2 py-0.5 bg-slate-100 text-slate-600 text-[10px] font-bold uppercase rounded-md border border-slate-200">#{{ tag }}</span>
              }
            </div>
            <div class="mt-4 pt-3 border-t border-slate-50 text-[10px] text-slate-400 flex justify-between items-center">
              <span>Le {{ note.created_at | date:'dd/MM/yyyy' }}</span>
              <span><i class="fa-solid fa-clock mr-1"></i> {{ note.created_at | date:'HH:mm' }}</span>
            </div>
          </div>
        } @empty {
          <div class="col-span-full py-12 text-center bg-slate-50 rounded-2xl border-2 border-dashed border-slate-200">
            <i class="fa-solid fa-lightbulb text-4xl text-slate-200 mb-4"></i>
            <h3 class="text-lg font-medium text-slate-600">Aucune note pour le moment</h3>
            <p class="text-slate-400 mt-1">Commencez à noter vos idées pour ce projet.</p>
          </div>
        }
      </div>
    </div>
  `
})
export class ProjectNotesComponent implements OnInit {
  private readonly route = inject(ActivatedRoute);
  private readonly projectFeatureService = inject(ProjectFeatureService);

  projectId: string = '';
  notes = signal<BrainstormingNote[]>([]);
  showCreateForm = false;
  
  newNote = {
    title: '',
    content: ''
  };
  tagsInput = '';

  ngOnInit(): void {
    this.projectId = this.route.snapshot.paramMap.get('id') || '';
    if (this.projectId) {
      this.loadNotes();
    }
  }

  loadNotes(): void {
    this.projectFeatureService.getNotes(this.projectId).subscribe(notes => {
      this.notes.set(notes);
    });
  }

  createNote(): void {
    const tags = this.tagsInput.split(',').map(t => t.trim()).filter(t => t.length > 0);
    this.projectFeatureService.createNote({
      project_id: this.projectId,
      title: this.newNote.title,
      content: this.newNote.content,
      tags: tags.length > 0 ? tags : undefined
    }).subscribe(note => {
      this.notes.update(current => [note, ...current]);
      this.resetForm();
    });
  }

  deleteNote(id: string): void {
    if (confirm('Voulez-vous vraiment supprimer cette note ?')) {
      this.projectFeatureService.deleteNote(id).subscribe(() => {
        this.notes.update(current => current.filter(n => n.id !== id));
      });
    }
  }

  private resetForm(): void {
    this.newNote = { title: '', content: '' };
    this.tagsInput = '';
    this.showCreateForm = false;
  }
}
