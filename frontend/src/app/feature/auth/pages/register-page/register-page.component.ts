import { Component, inject } from '@angular/core';
import { AuthStore } from '../../store/auth.store';
import { RegisterFormComponent } from '../../components/register-form/register-form.component';
import { RegisterInput } from '../../models/auth.model';

@Component({
    selector: 'app-register-page',
    standalone: true,
    imports: [RegisterFormComponent],
    templateUrl: './register-page.html',
})
export class RegisterPageComponent {
    // Injection de notre tout nouveau Store
    protected readonly authStore = inject(AuthStore);

    onRegisterSubmit(input: RegisterInput): void {
        // On propage l'action au store
        this.authStore.register(input);
    }
}