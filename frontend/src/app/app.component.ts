import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
    selector: 'app-root',
    standalone: true,
    imports: [RouterOutlet], // On importe le RouterOutlet pour activer l'affichage des routes
    template: `
        <main class="app-container">
            <router-outlet></router-outlet>
        </main>
    `,
    styles: [`
    .app-container {
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      min-height: 100vh;
      background-color: #f9fafb;
      display: flex;
      justify-content: center;
      align-items: center;
    }
  `]
})
export class AppComponent {}