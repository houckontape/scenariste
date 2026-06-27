import { ApplicationConfig, provideZoneChangeDetection } from '@angular/core';
import { provideRouter, withComponentInputBinding } from '@angular/router';
import { provideHttpClient } from '@angular/common/http';
import { routes } from './app.routes';

export const appConfig: ApplicationConfig = {
  providers: [
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes, withComponentInputBinding()), // Active le routage et passe optionnellement les paramètres d'URL aux inputs
    provideHttpClient() // CRUCIAL : Active l'utilisation du HttpClient injecté dans notre service API
  ]
};