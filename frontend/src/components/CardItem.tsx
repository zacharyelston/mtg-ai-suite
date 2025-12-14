'use client';

import Image from 'next/image';
import { Card } from '@/types/card';

interface CardItemProps {
  card: Card;
  onClick?: () => void;
}

function getCardImage(card: Card): string | null {
  if (card.image_uris?.normal) {
    return card.image_uris.normal;
  }
  if (card.card_faces?.[0]?.image_uris?.normal) {
    return card.card_faces[0].image_uris.normal;
  }
  return null;
}

export function CardItem({ card, onClick }: CardItemProps) {
  const imageUrl = getCardImage(card);

  return (
    <div
      onClick={onClick}
      className="cursor-pointer rounded-lg overflow-hidden bg-gray-800 hover:ring-2 hover:ring-blue-500 transition-all transform hover:scale-105"
      data-testid="card-item"
    >
      {imageUrl ? (
        <Image
          src={imageUrl}
          alt={card.name}
          width={244}
          height={340}
          className="w-full h-auto"
          unoptimized
        />
      ) : (
        <div className="aspect-[63/88] bg-gray-700 flex items-center justify-center p-4">
          <span className="text-gray-400 text-center text-sm">{card.name}</span>
        </div>
      )}
    </div>
  );
}
