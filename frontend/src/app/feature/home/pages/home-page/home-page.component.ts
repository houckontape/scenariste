import { Component } from '@angular/core';
import { TopBarComponent } from '../../../../core/components/top-bar/top-bar.component';
import { RouterLink } from '@angular/router';

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [TopBarComponent, RouterLink],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.scss'
})
export class HomePageComponent {
  features = [
    {
      title: 'Gestion de Projets',
      description: 'Organisez vos histoires, personnages et lieux dans un espace dédié et structuré.',
      icon: '📁'
    },
    {
      title: 'Brainstorming Markdown',
      description: 'Prenez des notes rapides et structurez vos idées avec la puissance du Markdown.',
      icon: '📝'
    },
    {
      title: 'Canevas Excalidraw',
      description: 'Visualisez vos intrigues et relations entre personnages avec des outils de dessin intuitifs.',
      icon: '🎨'
    },
    {
      title: 'Écriture de Scènes',
      description: 'Rédigez vos scènes avec un éditeur optimisé pour le format Fountain et standard de l\'industrie.',
      icon: '🎬'
    }
  ];
}
