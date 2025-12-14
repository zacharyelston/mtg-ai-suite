import { render, screen, fireEvent } from '@testing-library/react';
import { CardItem } from '@/components/CardItem';
import { Card } from '@/types/card';

jest.mock('next/image', () => ({
  __esModule: true,
  default: (props: { alt: string; src: string }) => {
    return <img alt={props.alt} src={props.src} />;
  },
}));

const mockCard: Card = {
  id: 'test-id',
  name: 'Lightning Bolt',
  cmc: 1,
  type_line: 'Instant',
  oracle_text: 'Lightning Bolt deals 3 damage to any target.',
  colors: ['R'],
  color_identity: ['R'],
  set: 'lea',
  set_name: 'Limited Edition Alpha',
  rarity: 'common',
  prices: { usd: '1.00' },
  legalities: { standard: 'not_legal', modern: 'legal' },
  scryfall_uri: 'https://scryfall.com/card/lea/161',
  image_uris: {
    small: 'https://example.com/small.jpg',
    normal: 'https://example.com/normal.jpg',
    large: 'https://example.com/large.jpg',
    art_crop: 'https://example.com/art.jpg',
  },
};

describe('CardItem', () => {
  it('renders card image', () => {
    render(<CardItem card={mockCard} />);
    
    const image = screen.getByAltText('Lightning Bolt');
    expect(image).toBeInTheDocument();
    expect(image).toHaveAttribute('src', 'https://example.com/normal.jpg');
  });

  it('calls onClick when clicked', () => {
    const handleClick = jest.fn();
    render(<CardItem card={mockCard} onClick={handleClick} />);
    
    const cardElement = screen.getByTestId('card-item');
    fireEvent.click(cardElement);
    
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('renders fallback for cards without image', () => {
    const cardWithoutImage: Card = {
      ...mockCard,
      image_uris: undefined,
    };

    render(<CardItem card={cardWithoutImage} />);
    
    expect(screen.getByText('Lightning Bolt')).toBeInTheDocument();
  });
});
