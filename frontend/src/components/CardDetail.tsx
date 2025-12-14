'use client';

import Image from 'next/image';
import { Card } from '@/types/card';

interface CardDetailProps {
  card: Card;
  onClose: () => void;
}

function getCardImage(card: Card): string | null {
  if (card.image_uris?.large) {
    return card.image_uris.large;
  }
  if (card.card_faces?.[0]?.image_uris?.large) {
    return card.card_faces[0].image_uris.large;
  }
  return null;
}

export function CardDetail({ card, onClose }: CardDetailProps) {
  const imageUrl = getCardImage(card);

  return (
    <div
      className="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-4"
      onClick={onClose}
      data-testid="card-detail-modal"
    >
      <div
        className="bg-gray-800 rounded-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex flex-col md:flex-row gap-6 p-6">
          <div className="flex-shrink-0">
            {imageUrl ? (
              <Image
                src={imageUrl}
                alt={card.name}
                width={336}
                height={468}
                className="rounded-lg mx-auto"
                unoptimized
              />
            ) : (
              <div className="w-[336px] aspect-[63/88] bg-gray-700 rounded-lg flex items-center justify-center">
                <span className="text-gray-400">{card.name}</span>
              </div>
            )}
          </div>
          
          <div className="flex-grow text-white">
            <div className="flex justify-between items-start mb-4">
              <h2 className="text-2xl font-bold">{card.name}</h2>
              <button
                onClick={onClose}
                className="text-gray-400 hover:text-white text-2xl leading-none"
                aria-label="Close"
              >
                &times;
              </button>
            </div>
            
            {card.mana_cost && (
              <p className="text-gray-300 mb-2">
                <span className="text-gray-500">Mana Cost:</span> {card.mana_cost}
              </p>
            )}
            
            <p className="text-gray-300 mb-2">
              <span className="text-gray-500">Type:</span> {card.type_line}
            </p>
            
            {card.oracle_text && (
              <div className="mb-4">
                <p className="text-gray-500 mb-1">Oracle Text:</p>
                <p className="text-gray-200 whitespace-pre-wrap">{card.oracle_text}</p>
              </div>
            )}
            
            {(card.power || card.toughness) && (
              <p className="text-gray-300 mb-2">
                <span className="text-gray-500">P/T:</span> {card.power}/{card.toughness}
              </p>
            )}
            
            <div className="flex gap-4 mb-4">
              <p className="text-gray-300">
                <span className="text-gray-500">Set:</span> {card.set_name}
              </p>
              <p className="text-gray-300">
                <span className="text-gray-500">Rarity:</span> {card.rarity}
              </p>
            </div>
            
            {(card.prices.usd || card.prices.usd_foil) && (
              <div className="flex gap-4 mb-4">
                {card.prices.usd && (
                  <p className="text-green-400">
                    <span className="text-gray-500">Price:</span> ${card.prices.usd}
                  </p>
                )}
                {card.prices.usd_foil && (
                  <p className="text-purple-400">
                    <span className="text-gray-500">Foil:</span> ${card.prices.usd_foil}
                  </p>
                )}
              </div>
            )}
            
            <a
              href={card.scryfall_uri}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-block mt-4 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded text-white transition-colors"
            >
              View on Scryfall
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
