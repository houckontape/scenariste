import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SidebarComponent } from '../../components/sidebar/sidebar.component';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-dashboard-layout',
  standalone: true,
  imports: [CommonModule, SidebarComponent, RouterModule],
  template: `
    <div class="flex h-screen w-screen overflow-hidden bg-slate-50">
      <!-- Sidebar fixe -->
      <app-sidebar class="w-64 flex-shrink-0 h-full"></app-sidebar>

      <!-- Zone de contenu principal -->
      <main class="flex-grow h-full overflow-y-auto p-8">
        <div class="max-w-7xl mx-auto">
          <router-outlet></router-outlet>
        </div>
      </main>
    </div>
  `,
  styles: [`
    :host {
      display: block;
      height: 100vh;
      width: 100vw;
    }
  `]
})
export class DashboardLayoutComponent {}
