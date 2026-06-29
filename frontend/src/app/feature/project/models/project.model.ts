export type ProjectType = 'movie' | 'series' | 'animation' | 'novel';
export type MemberRole = 'owner' | 'write' | 'read';

export interface Project {
    id: string;
    title: string;
    slug: string;
    description?: string;
    p_type: ProjectType;
    is_private: boolean;
    shooting_technique_id?: string;
    created_at: string;
    updated_at: string;
}

export interface ProjectWithRole extends Project {
    user_role: MemberRole;
}

export interface CreateProjectInput {
    title: string;
    description?: string;
    p_type: ProjectType;
    is_private: boolean;
    shooting_technique_id?: string;
}

export interface ShootingTechnique {
    id: string;
    name: string;
    slug: string;
    description?: string;
    created_at: string;
    updated_at: string;
}
