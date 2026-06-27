export interface RegisterInput {
    email: string;
    company: string;
    password: string;
}

export interface AuthResponse {
    message: string;
    // Nous pourrons enrichir cette interface plus tard (ex: avec un User ou un token)
}