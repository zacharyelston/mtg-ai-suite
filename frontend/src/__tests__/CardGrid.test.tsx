import { render, screen } from '@testing-library/react';
import { CardGrid } from '@/components/CardGrid';
import { Card } from '@/types/card';

jest.mock('next/image', () => ({
  __esModule: true,
  default: (props: { alt: string; src: string }) => {
    return <img alt={props.alt} src={props.src} />;
  },
}));

const mockCards: Card[] = [
  {
    id: '1',
    name: 'Lightning Bolt',
    cmc: 1,
    type_line: 'Instant',
    color_identity: ['R'],
    set: 'lea',
    set_name: 'Alpha',
    rarity: 'common',
    prices: {},
    legalities: {},
    scryfall_uri: 'https://scryfall.com/card/1',
    image_uris: {
      small: 'https://example.com/small1.jpg',
      normal: 'https://example.com/normal1.jpg',
      large: 'https://example.com/large1.jpg',
      art_crop: 'https://example.com/art1.jpg',
    },
  },
  {
    id: '2',
    name: 'Counterspell',
    cmc: 2,
    type_line: 'Instant',
    color_identity: ['U'],
    set: 'lea',
    set_name: 'Alpha',
    rarity: 'uncommon',
    prices: {},
    legalities: {},
    scryfall_uri: 'https://scryfall.com/card/2',
    image_uris: {
      small: 'https://example.com/small2.jpg',
      normal: 'https://example.com/normal2.jpg',
      large: 'https://example.com/large2.jpg',
      art_crop: 'https://example.com/art2.jpg',
    },
  },
];

describe('CardGrid', () => {
  it('renders all cards in the grid', () => {
    render(<CardGrid cards={mockCards} />);
    
    expect(screen.getByTestId('card-grid')).toBeInTheDocument();
    expect(screen.getByAltText('Lightning Bolt')).toBeInTheDocument();
    expect(screen.getByAltText('Counterspell')).toBeInTheDocument();
  });

  it('renders empty message when no cards', () => {
    render(<CardGrid cards={[]} />);
    
    expect(screen.getByText('No cards found. Try a different search.')).toBeInTheDocument();
  });
});
