export interface Card {
  id: string;
  name: string;
  mana_cost?: string;
  cmc: number;
  type_line: string;
  oracle_text?: string;
  power?: string;
  toughness?: string;
  colors?: string[];
  color_identity: string[];
  set: string;
  set_name: string;
  rarity: string;
  image_uris?: {
    small: string;
    normal: string;
    large: string;
    art_crop: string;
  };
  card_faces?: Array<{
    name: string;
    mana_cost?: string;
    type_line: string;
    oracle_text?: string;
    image_uris?: {
      small: string;
      normal: string;
      large: string;
      art_crop: string;
    };
  }>;
  prices: {
    usd?: string;
    usd_foil?: string;
  };
  legalities: Record<string, string>;
  scryfall_uri: string;
}

export interface SearchResult {
  object: string;
  total_cards: number;
  has_more: boolean;
  next_page?: string;
  data: Card[];
}
