import { Component, output } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { inject } from '@angular/core';
import { RegisterInput } from '../../models/auth.model';

@Component({
    selector: 'app-register-form',
    standalone: true,
    imports: [ReactiveFormsModule], // Plus besoin de modules globaux, on importe directement ce dont on a besoin
    templateUrl: './register-form.html',
})
export class RegisterFormComponent {
    private readonly fb = inject(FormBuilder);

    // Déclaration de l'événement de sortie avec la nouvelle API output() (Angular v17.3+)
    submitRegister = output<RegisterInput>();

    // Définition du formulaire réactif avec validations de base
    registerForm = this.fb.nonNullable.group({
        email: ['', [Validators.required, Validators.email]],
        password: ['', [Validators.required, Validators.minLength(8)]] // Rappel : Notre back exige min 8 caractères !
    });

    onSubmit(): void {
        if (this.registerForm.valid) {
            // getRawValue() garantit qu'on récupère les types stricts définis par nonNullable
            const formValue = this.registerForm.getRawValue();
            this.submitRegister.emit(formValue);
        }
    }
}