export interface RegisterInput {
    email: string;
    password: string;
}

export interface LoginInput {
    email: string;
    password: string;
}

export interface User {
    id: string;
    email: string;
    role: string;
    first_name?: string;
    last_name?: string;
    avatar_url?: string;
}

export interface AuthResponse {
    token: string;
    user: User;
}

export interface RegisterResponse {
    message: string;
    user_id: string;
}