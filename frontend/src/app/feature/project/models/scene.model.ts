import { ShootingTechnique } from './project.model';

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
    shooting_technique_id?: string;
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
    shooting_technique_id?: string;
}

export interface UpdateSceneInput {
    act_id?: string;
    position?: number;
    setting?: SceneSetting;
    location?: string;
    time_of_day?: SceneTimeOfDay;
    content?: string;
    note?: string;
    shooting_technique_id?: string;
}
