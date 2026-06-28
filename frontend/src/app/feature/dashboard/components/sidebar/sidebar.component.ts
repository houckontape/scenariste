import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { AuthStore } from '../../../auth/store/auth.store';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.scss'
})
export class SidebarComponent {
  private readonly authStore = inject(AuthStore);
  
  readonly currentUser = this.authStore.currentUser;

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
