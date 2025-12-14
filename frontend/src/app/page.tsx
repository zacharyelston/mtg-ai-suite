'use client';

import { useState, useCallback } from 'react';
import { SearchBar, CardGrid, CardDetail, CameraCapture, CapturePreview } from '@/components';
import { searchCards, getRandomCard } from '@/lib/scryfall';
import { submitCapture } from '@/lib/captures';
import { Card } from '@/types/card';
import { CaptureMetadata, CaptureResponse } from '@/types/capture';

type TabType = 'search' | 'camera';

export default function Home() {
  const [activeTab, setActiveTab] = useState<TabType>('search');
  
  const [cards, setCards] = useState<Card[]>([]);
  const [selectedCard, setSelectedCard] = useState<Card | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [hasSearched, setHasSearched] = useState(false);
  const [totalCards, setTotalCards] = useState(0);

  const [capturedImage, setCapturedImage] = useState<string | null>(null);
  const [captureMetadata, setCaptureMetadata] = useState<CaptureMetadata | null>(null);
  const [captureResponse, setCaptureResponse] = useState<CaptureResponse | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSearch = async (query: string) => {
    setIsLoading(true);
    setHasSearched(true);
    try {
      const result = await searchCards(query);
      setCards(result.data);
      setTotalCards(result.total_cards);
    } catch (error) {
      console.error('Search failed:', error);
      setCards([]);
      setTotalCards(0);
    } finally {
      setIsLoading(false);
    }
  };

  const handleRandomCard = async () => {
    setIsLoading(true);
    try {
      const card = await getRandomCard();
      setSelectedCard(card);
    } catch (error) {
      console.error('Failed to get random card:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCapture = useCallback((imageData: string, metadata: CaptureMetadata) => {
    setCapturedImage(imageData);
    setCaptureMetadata(metadata);
    setCaptureResponse(null);
  }, []);

  const handleSubmitCapture = async () => {
    if (!capturedImage || !captureMetadata) return;

    setIsSubmitting(true);
    try {
      const response = await submitCapture(capturedImage, captureMetadata);
      setCaptureResponse(response);
    } catch (error) {
      console.error('Failed to submit capture:', error);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleDiscardCapture = () => {
    setCapturedImage(null);
    setCaptureMetadata(null);
    setCaptureResponse(null);
  };

  return (
    <main className="min-h-screen bg-gradient-to-b from-gray-900 to-gray-800">
      <div className="container mx-auto px-4 py-8">
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-white mb-2">MTG AI Suite</h1>
          <p className="text-gray-300 mb-6">
            Search cards or scan them with your camera
          </p>

          <div className="flex justify-center gap-2 mb-8">
            <button
              onClick={() => setActiveTab('search')}
              className={`px-6 py-2 rounded-lg font-medium transition-colors ${
                activeTab === 'search'
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
              }`}
              data-testid="search-tab"
            >
              Search Cards
            </button>
            <button
              onClick={() => setActiveTab('camera')}
              className={`px-6 py-2 rounded-lg font-medium transition-colors ${
                activeTab === 'camera'
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
              }`}
              data-testid="camera-tab"
            >
              Camera Scan
            </button>
          </div>
        </div>

        {activeTab === 'search' && (
          <>
            <div className="text-center mb-8">
              <SearchBar onSearch={handleSearch} isLoading={isLoading} />
              
              <button
                onClick={handleRandomCard}
                disabled={isLoading}
                className="mt-4 px-4 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-purple-800 rounded text-white font-medium transition-colors"
                data-testid="random-card-button"
              >
                Random Card
              </button>
            </div>

            {isLoading && (
              <div className="text-center text-gray-400 py-12">
                <div className="animate-spin inline-block w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full mb-4"></div>
                <p>Loading...</p>
              </div>
            )}

            {!isLoading && hasSearched && (
              <div className="mb-4">
                <p className="text-gray-400 text-sm">
                  Found {totalCards} card{totalCards !== 1 ? 's' : ''}
                </p>
              </div>
            )}

            {!isLoading && hasSearched && <CardGrid cards={cards} onCardClick={setSelectedCard} />}

            {!hasSearched && !isLoading && (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mt-8">
                <FeatureCard
                  title="Card Database"
                  description="Search and explore the complete MTG card database with semantic search"
                  icon="🃏"
                />
                <FeatureCard
                  title="Deck Builder"
                  description="AI-powered deck building with archetype analysis and suggestions"
                  icon="📚"
                />
                <FeatureCard
                  title="Game Tracker"
                  description="Real-time game state tracking and play history"
                  icon="📊"
                />
                <FeatureCard
                  title="Play Advisor"
                  description="Get intelligent suggestions for optimal plays"
                  icon="🧠"
                />
              </div>
            )}
          </>
        )}

        {activeTab === 'camera' && (
          <div className="max-w-2xl mx-auto">
            {capturedImage && captureMetadata ? (
              <CapturePreview
                imageData={capturedImage}
                metadata={captureMetadata}
                response={captureResponse || undefined}
                isSubmitting={isSubmitting}
                onSubmit={handleSubmitCapture}
                onDiscard={handleDiscardCapture}
              />
            ) : (
              <>
                <CameraCapture onCapture={handleCapture} />
                <div className="mt-6 text-center text-gray-400 text-sm">
                  <p>Point your camera at an MTG card and tap capture to identify it.</p>
                  <p className="mt-2">Works best with good lighting and a flat card.</p>
                </div>
              </>
            )}
          </div>
        )}

        {selectedCard && (
          <CardDetail card={selectedCard} onClose={() => setSelectedCard(null)} />
        )}
      </div>
    </main>
  );
}

function FeatureCard({
  title,
  description,
  icon,
}: {
  title: string;
  description: string;
  icon: string;
}) {
  return (
    <div className="bg-gray-800 rounded-lg p-6 border border-gray-700 hover:border-blue-500 transition-colors">
      <div className="text-4xl mb-4">{icon}</div>
      <h3 className="text-xl font-semibold text-white mb-2">{title}</h3>
      <p className="text-gray-400">{description}</p>
    </div>
  );
}
