'use client';

import { Card } from '@/types/card';
import { CardItem } from './CardItem';

interface CardGridProps {
  cards: Card[];
  onCardClick?: (card: Card) => void;
}

export function CardGrid({ cards, onCardClick }: CardGridProps) {
  if (cards.length === 0) {
    return (
      <div className="text-center text-gray-400 py-12">
        <p>No cards found. Try a different search.</p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4" data-testid="card-grid">
      {cards.map((card) => (
        <CardItem key={card.id} card={card} onClick={() => onCardClick?.(card)} />
      ))}
    </div>
  );
}
