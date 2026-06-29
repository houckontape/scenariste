import { Component, inject, signal, effect } from '@angular/core';
import { CommonModule } from '@angular/common';
import { AuthStore } from '../../../auth/store/auth.store';
import { Router, NavigationEnd, RouterModule } from '@angular/router';
import { filter } from 'rxjs';

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.scss'
})
export class SidebarComponent {
  private readonly authStore = inject(AuthStore);
  private readonly router = inject(Router);
  
  readonly currentUser = this.authStore.currentUser;
  readonly currentProjectId = signal<string | null>(null);

  constructor() {
    this.router.events.pipe(
      filter(event => event instanceof NavigationEnd)
    ).subscribe(() => {
      this.updateCurrentProjectId();
    });
    // Initial check
    this.updateCurrentProjectId();
  }

  private updateCurrentProjectId(): void {
    const urlParts = this.router.url.split('/');
    const projectsIndex = urlParts.indexOf('projects');
    if (projectsIndex !== -1 && urlParts[projectsIndex + 1] && urlParts[projectsIndex + 1] !== 'create') {
      this.currentProjectId.set(urlParts[projectsIndex + 1]);
    } else {
      this.currentProjectId.set(null);
    }
  }

  get userRoleDisplay(): string {
    const role = this.currentUser()?.role;
    switch (role) {
      case 'free_user': return 'Scénariste Free';
      case 'premium_user': return 'Scénariste Premium';
      case 'admin': return 'Administrateur';
      case 'super_admin': return 'Super Admin';
      case 'support': return 'Support';
      default: return 'Scénariste';
    }
  }

  get userDisplayName(): string {
    const user = this.currentUser();
    if (user?.first_name || user?.last_name) {
      return `${user.first_name ?? ''} ${user.last_name ?? ''}`.trim();
    }
    return user?.email ?? 'Utilisateur';
  }

  logout(): void {
    this.authStore.logout();
  }
}
