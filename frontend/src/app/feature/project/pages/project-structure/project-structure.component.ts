import { Component, inject, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute } from '@angular/router';
import { ProjectFeatureService } from '../../services/project-feature.service';
import { ProjectAct, Scene, CharacterBible, SceneSetting, SceneTimeOfDay, CreateSceneInput } from '../../models/project-features.model';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-project-structure',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <div class="space-y-6">
      <header class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-slate-900">Structure Narrative</h1>
          <p class="text-slate-500">Organisez votre récit en actes et scènes.</p>
        </div>
        <button (click)="createAct()" class="bg-amber-600 text-white px-4 py-2 rounded-lg hover:bg-amber-700 transition-colors">
          <i class="fa-solid fa-plus mr-2"></i> Ajouter un Acte
        </button>
      </header>

      <div class="space-y-8">
        @for (act of acts(); track act.id) {
          <div class="bg-white rounded-xl border border-slate-200 shadow-sm overflow-hidden">
            <div class="bg-slate-50 px-6 py-4 border-b border-slate-200 flex justify-between items-center">
              <div class="flex items-center space-x-4">
                <span class="bg-amber-100 text-amber-700 w-8 h-8 rounded-full flex items-center justify-center font-bold text-sm">{{ act.position }}</span>
                <h2 class="text-lg font-bold text-slate-800">{{ act.title }}</h2>
              </div>
              <div class="flex items-center space-x-2">
                 <button (click)="openAddSceneModal(act)" class="text-xs bg-white border border-slate-200 px-3 py-1.5 rounded-md hover:bg-slate-50 transition-colors font-medium">
                    <i class="fa-solid fa-plus mr-1"></i> Scène
                 </button>
                 <button (click)="deleteAct(act.id)" class="text-slate-400 hover:text-red-500 transition-colors p-1.5">
                    <i class="fa-solid fa-trash-can text-sm"></i>
                 </button>
              </div>
            </div>
            
            <div class="p-6">
              <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                @for (scene of getScenesForAct(act.id); track scene.id) {
                  <div class="p-4 rounded-lg border border-slate-100 bg-slate-50/50 hover:border-amber-200 hover:bg-amber-50/30 transition-all cursor-pointer group">
                    <div class="flex justify-between items-start mb-2">
                       <span class="text-[10px] font-bold text-slate-400 uppercase tracking-tighter">Scène {{ scene.position }}</span>
                       <button (click)="deleteScene(scene.id); $event.stopPropagation()" class="text-slate-300 hover:text-red-500 opacity-0 group-hover:opacity-100 transition-opacity">
                         <i class="fa-solid fa-xmark"></i>
                       </button>
                    </div>
                    <div class="flex items-baseline space-x-1 mb-1">
                      <span class="text-[10px] font-black text-amber-600">{{ scene.setting }}.</span>
                      <h4 class="font-bold text-slate-800 text-sm uppercase truncate">{{ scene.location }}</h4>
                    </div>
                    <div class="text-[9px] font-bold text-slate-400 uppercase mb-3">{{ scene.time_of_day }}</div>
                    <p class="text-slate-500 text-xs line-clamp-2 italic">{{ scene.content || scene.note || 'Pas de contenu.' }}</p>
                  </div>
                } @empty {
                  <div class="col-span-full py-8 text-center text-slate-400 text-sm italic">
                    Aucune scène dans cet acte.
                  </div>
                }
              </div>
            </div>
          </div>
        } @empty {
          <div class="py-20 text-center bg-slate-50 rounded-2xl border-2 border-dashed border-slate-200">
            <i class="fa-solid fa-layer-group text-4xl text-slate-200 mb-4"></i>
            <h3 class="text-lg font-medium text-slate-600">Votre structure est vide</h3>
            <p class="text-slate-400 mt-1">Commencez par ajouter l'Acte I pour structurer votre récit.</p>
            <button (click)="createAct()" class="mt-6 bg-amber-600 text-white px-6 py-2 rounded-lg hover:bg-amber-700 transition-colors">
              Créer l'Acte I
            </button>
          </div>
        }
      </div>
    </div>

    <!-- Modal Ajout Scène -->
    @if (showSceneModal()) {
      <div class="fixed inset-0 bg-slate-900/50 backdrop-blur-sm flex items-center justify-center z-50 p-4">
        <div class="bg-white rounded-2xl shadow-xl w-full max-w-2xl overflow-hidden">
          <div class="px-6 py-4 border-b border-slate-100 flex justify-between items-center bg-slate-50">
            <h3 class="font-bold text-slate-800">Ajouter une nouvelle scène</h3>
            <button (click)="showSceneModal.set(false)" class="text-slate-400 hover:text-slate-600">
              <i class="fa-solid fa-xmark"></i>
            </button>
          </div>
          
          <div class="p-6 space-y-6 max-h-[80vh] overflow-y-auto">
            <!-- Slug Line Section -->
            <div class="space-y-4">
              <h4 class="text-xs font-bold text-slate-400 uppercase tracking-widest">Entête de scène (Slug Line)</h4>
              <div class="grid grid-cols-12 gap-4">
                <div class="col-span-3">
                  <label class="block text-xs font-medium text-slate-500 mb-1">Setting</label>
                  <select [(ngModel)]="sceneForm.setting" class="w-full bg-slate-100 border-none rounded-lg text-sm focus:ring-2 focus:ring-amber-500">
                    <option value="INT">INT.</option>
                    <option value="EXT">EXT.</option>
                    <option value="INT_EXT">INT/EXT.</option>
                  </select>
                </div>
                <div class="col-span-5">
                  <label class="block text-xs font-medium text-slate-500 mb-1">Lieu</label>
                  <input type="text" [(ngModel)]="sceneForm.location" placeholder="SALON, FORÊT..." class="w-full bg-slate-100 border-none rounded-lg text-sm focus:ring-2 focus:ring-amber-500 uppercase">
                </div>
                <div class="col-span-4">
                  <label class="block text-xs font-medium text-slate-500 mb-1">Moment</label>
                  <select [(ngModel)]="sceneForm.time_of_day" class="w-full bg-slate-100 border-none rounded-lg text-sm focus:ring-2 focus:ring-amber-500">
                    <option value="DAY">JOUR</option>
                    <option value="NIGHT">NUIT</option>
                    <option value="MORNING">MATIN</option>
                    <option value="EVENING">SOIR</option>
                    <option value="DAWN">AUBE</option>
                    <option value="DUSK">CRÉPUSCULE</option>
                  </select>
                </div>
              </div>
            </div>

            <!-- Action Section -->
            <div class="space-y-4">
              <h4 class="text-xs font-bold text-slate-400 uppercase tracking-widest">Description de l'action</h4>
              <textarea [(ngModel)]="sceneForm.action" rows="3" placeholder="Décrivez l'action de la scène..." class="w-full bg-slate-100 border-none rounded-lg text-sm focus:ring-2 focus:ring-amber-500"></textarea>
            </div>

            <!-- Dialogue Section -->
            <div class="space-y-4">
              <h4 class="text-xs font-bold text-slate-400 uppercase tracking-widest">Premier Dialogue (Optionnel)</h4>
              <div class="grid grid-cols-1 gap-4">
                <div>
                  <label class="block text-xs font-medium text-slate-500 mb-1">Personnage</label>
                  <select [(ngModel)]="sceneForm.character" class="w-full bg-slate-100 border-none rounded-lg text-sm focus:ring-2 focus:ring-amber-500">
                    <option value="">-- Sélectionner un personnage --</option>
                    @for (char of characters(); track char.id) {
                      <option [value]="char.full_name">{{ char.full_name | uppercase }}</option>
                    }
                  </select>
                </div>
                <div>
                  <label class="block text-xs font-medium text-slate-500 mb-1">Réplique</label>
                  <textarea [(ngModel)]="sceneForm.dialogue" rows="2" placeholder="Saisissez la réplique..." class="w-full bg-slate-100 border-none rounded-lg text-sm focus:ring-2 focus:ring-amber-500"></textarea>
                </div>
              </div>
            </div>
          </div>

          <div class="px-6 py-4 bg-slate-50 border-t border-slate-100 flex justify-end space-x-3">
            <button (click)="showSceneModal.set(false)" class="px-4 py-2 text-sm font-medium text-slate-600 hover:text-slate-800 transition-colors">
              Annuler
            </button>
            <button (click)="submitScene()" [disabled]="!sceneForm.location" class="bg-amber-600 text-white px-6 py-2 rounded-lg hover:bg-amber-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
              Enregistrer la scène
            </button>
          </div>
        </div>
      </div>
    }
  `
})
export class ProjectStructureComponent implements OnInit {
  private readonly route = inject(ActivatedRoute);
  private readonly projectFeatureService = inject(ProjectFeatureService);

  projectId: string = '';
  acts = signal<ProjectAct[]>([]);
  scenes = signal<Scene[]>([]);
  characters = signal<CharacterBible[]>([]);
  
  showSceneModal = signal(false);
  currentActForScene: ProjectAct | null = null;
  
  sceneForm = {
    setting: 'INT' as SceneSetting,
    location: '',
    time_of_day: 'DAY' as SceneTimeOfDay,
    action: '',
    character: '',
    dialogue: ''
  };

  ngOnInit(): void {
    this.projectId = this.route.snapshot.paramMap.get('id') || '';
    if (this.projectId) {
      this.loadData();
    }
  }

  loadData(): void {
    this.projectFeatureService.getActs(this.projectId).subscribe(acts => {
      this.acts.set(acts.sort((a, b) => a.position - b.position));
    });
    this.projectFeatureService.getScenes(this.projectId).subscribe(scenes => {
      this.scenes.set(scenes.sort((a, b) => a.position - b.position));
    });
    this.projectFeatureService.getCharacters(this.projectId).subscribe(chars => {
      this.characters.set(chars);
    });
  }

  getScenesForAct(actId: string): Scene[] {
    return this.scenes().filter(s => s.act_id === actId);
  }

  createAct(): void {
    const nextPosition = this.acts().length + 1;
    const title = nextPosition === 1 ? 'Acte I' : nextPosition === 2 ? 'Acte II' : nextPosition === 3 ? 'Acte III' : `Acte ${nextPosition}`;
    
    this.projectFeatureService.createAct({
      project_id: this.projectId,
      title: title,
      position: nextPosition
    }).subscribe(act => {
      this.acts.update(current => [...current, act].sort((a, b) => a.position - b.position));
    });
  }

  openAddSceneModal(act: ProjectAct): void {
    this.currentActForScene = act;
    this.sceneForm = {
      setting: 'INT',
      location: '',
      time_of_day: 'DAY',
      action: '',
      character: '',
      dialogue: ''
    };
    this.showSceneModal.set(true);
  }

  submitScene(): void {
    if (!this.currentActForScene || !this.sceneForm.location) return;

    const actScenes = this.getScenesForAct(this.currentActForScene.id);
    const nextPosition = actScenes.length + 1;

    // Construction du contenu au format Fountain-ish
    let content = '';
    if (this.sceneForm.action) {
      content += this.sceneForm.action + '\n\n';
    }
    if (this.sceneForm.character && this.sceneForm.dialogue) {
      content += this.sceneForm.character.toUpperCase() + '\n';
      content += this.sceneForm.dialogue;
    }

    this.projectFeatureService.createScene({
      act_id: this.currentActForScene.id,
      project_id: this.projectId,
      position: nextPosition,
      setting: this.sceneForm.setting,
      location: this.sceneForm.location.toUpperCase(),
      time_of_day: this.sceneForm.time_of_day,
      content: content.trim()
    }).subscribe(scene => {
      this.scenes.update(current => [...current, scene].sort((a, b) => a.position - b.position));
      this.showSceneModal.set(false);
    });
  }

  deleteAct(id: string): void {
    if (confirm('Supprimer cet acte supprimera aussi toutes ses scènes. Continuer ?')) {
      this.projectFeatureService.deleteAct(id).subscribe(() => {
        this.acts.update(current => current.filter(a => a.id !== id));
        this.scenes.update(current => current.filter(s => s.act_id !== id));
      });
    }
  }

  deleteScene(id: string): void {
    if (confirm('Supprimer cette scène ?')) {
      this.projectFeatureService.deleteScene(id).subscribe(() => {
        this.scenes.update(current => current.filter(s => s.id !== id));
      });
    }
  }
}
