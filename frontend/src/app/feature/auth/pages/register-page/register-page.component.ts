import { Component, inject } from '@angular/core';
import { AuthStore } from '../../store/auth.store';
import { RegisterInput } from '../../models/auth.model';
import { RouterLink } from '@angular/router';
import { TopBarComponent } from '../../../../shared/components/top-bar/top-bar.component';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { CommonModule } from '@angular/common';

@Component({
    selector: 'app-register-page',
    standalone: true,
    imports: [RouterLink, TopBarComponent, ReactiveFormsModule, CommonModule],
    templateUrl: './register-page.html',
    styleUrl: './register-page.scss'
})
export class RegisterPageComponent {
    protected readonly authStore = inject(AuthStore);
    private readonly fb = inject(FormBuilder);

    registerForm = this.fb.nonNullable.group({
        email: ['', [Validators.required, Validators.email]],
        password: ['', [Validators.required, Validators.minLength(8)]]
    });

    onRegisterSubmit(): void {
        if (this.registerForm.valid) {
            this.authStore.register(this.registerForm.getRawValue());
        }
    }
}