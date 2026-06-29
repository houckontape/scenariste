export interface BrainstormingNote {
    id: string;
    project_id: string;
    author_id?: string;
    title: string;
    content: string;
    tags?: string[];
    created_at: string;
    updated_at: string;
}

export interface CreateBrainstormingNoteInput {
    project_id: string;
    title: string;
    content: string;
    tags?: string[];
}

export interface CharacterBible {
    id: string;
    project_id: string;
    full_name: string;
    character_role: string;
    avatar_url?: string;
    description?: string;
    attributes: any;
    created_at: string;
    updated_at: string;
}

export interface CreateCharacterInput {
    project_id: string;
    full_name: string;
    character_role?: string;
    avatar_url?: string;
    description?: string;
    attributes?: any;
}

export interface ProjectAct {
    id: string;
    project_id: string;
    title: string;
    position: number;
    description?: string;
    created_at: string;
    updated_at: string;
}

export interface CreateProjectActInput {
    project_id: string;
    title: string;
    position: number;
    description?: string;
}

export type SceneSetting = 'INT' | 'EXT' | 'INT_EXT';
export type SceneTimeOfDay = 'DAY' | 'NIGHT' | 'MORNING' | 'EVENING' | 'DAWN' | 'DUSK';

export interface Scene {
    id: string;
    act_id: string;
    project_id: string;
    position: number;
    setting: SceneSetting;
    location: string;
    time_of_day: SceneTimeOfDay;
    content: string;
    note?: string;
    created_at: string;
    updated_at: string;
}

export interface CreateSceneInput {
    act_id: string;
    project_id: string;
    position: number;
    setting: SceneSetting;
    location: string;
    time_of_day: SceneTimeOfDay;
    content?: string;
    note?: string;
}
