import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { 
    BrainstormingNote, CreateBrainstormingNoteInput, 
    CharacterBible, CreateCharacterInput,
    ProjectAct, CreateProjectActInput,
    Scene, CreateSceneInput
} from '../models/project-features.model';

@Injectable({
    providedIn: 'root'
})
export class ProjectFeatureService {
    private readonly http = inject(HttpClient);
    private readonly apiUrl = 'http://localhost:3000/api';

    // Brainstorming Notes
    getNotes(projectId: string): Observable<BrainstormingNote[]> {
        return this.http.get<BrainstormingNote[]>(`${this.apiUrl}/projects/${projectId}/notes`);
    }

    createNote(input: CreateBrainstormingNoteInput): Observable<BrainstormingNote> {
        return this.http.post<BrainstormingNote>(`${this.apiUrl}/notes`, input);
    }

    updateNote(id: string, input: Partial<CreateBrainstormingNoteInput>): Observable<BrainstormingNote> {
        return this.http.patch<BrainstormingNote>(`${this.apiUrl}/notes/${id}`, input);
    }

    deleteNote(id: string): Observable<void> {
        return this.http.delete<void>(`${this.apiUrl}/notes/${id}`);
    }

    // Character Bible
    getCharacters(projectId: string): Observable<CharacterBible[]> {
        return this.http.get<CharacterBible[]>(`${this.apiUrl}/projects/${projectId}/characters`);
    }

    createCharacter(input: CreateCharacterInput): Observable<CharacterBible> {
        return this.http.post<CharacterBible>(`${this.apiUrl}/characters`, input);
    }

    updateCharacter(id: string, input: Partial<CreateCharacterInput>): Observable<CharacterBible> {
        return this.http.patch<CharacterBible>(`${this.apiUrl}/characters/${id}`, input);
    }

    deleteCharacter(id: string): Observable<void> {
        return this.http.delete<void>(`${this.apiUrl}/characters/${id}`);
    }

    // Acts
    getActs(projectId: string): Observable<ProjectAct[]> {
        return this.http.get<ProjectAct[]>(`${this.apiUrl}/projects/${projectId}/acts`);
    }

    createAct(input: CreateProjectActInput): Observable<ProjectAct> {
        return this.http.post<ProjectAct>(`${this.apiUrl}/acts`, input);
    }

    updateAct(id: string, input: Partial<CreateProjectActInput>): Observable<ProjectAct> {
        return this.http.patch<ProjectAct>(`${this.apiUrl}/acts/${id}`, input);
    }

    deleteAct(id: string): Observable<void> {
        return this.http.delete<void>(`${this.apiUrl}/acts/${id}`);
    }

    // Scenes
    getScenes(projectId: string): Observable<Scene[]> {
        return this.http.get<Scene[]>(`${this.apiUrl}/projects/${projectId}/scenes`);
    }

    createScene(input: CreateSceneInput): Observable<Scene> {
        return this.http.post<Scene>(`${this.apiUrl}/scenes`, input);
    }

    updateScene(id: string, input: Partial<CreateSceneInput>): Observable<Scene> {
        return this.http.patch<Scene>(`${this.apiUrl}/scenes/${id}`, input);
    }

    deleteScene(id: string): Observable<void> {
        return this.http.delete<void>(`${this.apiUrl}/scenes/${id}`);
    }
}
