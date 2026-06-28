import { Component, output } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { inject } from '@angular/core';
import { LoginInput } from '../../models/auth.model';

@Component({
    selector: 'app-login-form',
    standalone: true,
    imports: [ReactiveFormsModule],
    templateUrl: './login-form.html',
})
export class LoginFormComponent {
    private readonly fb = inject(FormBuilder);

    submitLogin = output<LoginInput>();

    loginForm = this.fb.nonNullable.group({
        email: ['', [Validators.required, Validators.email]],
        password: ['', [Validators.required]]
    });

    onSubmit(): void {
        if (this.loginForm.valid) {
            this.submitLogin.emit(this.loginForm.getRawValue());
        }
    }
}
