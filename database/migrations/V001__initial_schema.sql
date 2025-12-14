-- MTG AI Suite Initial Schema
-- Flyway migration V001

-- ============================================
-- Extensions
-- ============================================
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";  -- pgvector for embeddings

-- ============================================
-- Cards table (Scryfall data cache)
-- ============================================
CREATE TABLE cards (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scryfall_id VARCHAR(36) UNIQUE NOT NULL,
    oracle_id VARCHAR(36),
    name VARCHAR(255) NOT NULL,
    mana_cost VARCHAR(100),
    cmc DECIMAL(4,1) NOT NULL DEFAULT 0,
    type_line VARCHAR(255) NOT NULL,
    oracle_text TEXT,
    power VARCHAR(10),
    toughness VARCHAR(10),
    colors JSONB DEFAULT '[]',
    color_identity JSONB DEFAULT '[]',
    keywords JSONB DEFAULT '[]',
    set_code VARCHAR(10) NOT NULL,
    set_name VARCHAR(100) NOT NULL,
    rarity VARCHAR(20) NOT NULL,
    image_uris JSONB,
    legalities JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cards_name ON cards(name);
CREATE INDEX idx_cards_scryfall_id ON cards(scryfall_id);
CREATE INDEX idx_cards_oracle_id ON cards(oracle_id);
CREATE INDEX idx_cards_set_code ON cards(set_code);

-- ============================================
-- Card name embeddings (for semantic search)
-- ============================================
CREATE TABLE card_embeddings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    card_id UUID NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    embedding vector(1536),  -- OpenAI ada-002 dimension
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_card_embeddings_card_id ON card_embeddings(card_id);

-- ============================================
-- Captures (card recognition attempts)
-- ============================================
CREATE TABLE captures (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID,  -- NULL for anonymous
    image_hash VARCHAR(64) NOT NULL,
    image_path VARCHAR(500),
    ocr_raw TEXT,
    recognized_card_id UUID REFERENCES cards(id),
    confidence DECIMAL(5,4),
    final_card_id UUID REFERENCES cards(id),
    user_verified BOOLEAN DEFAULT FALSE,
    captured_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_captures_user_id ON captures(user_id);
CREATE INDEX idx_captures_recognized_card ON captures(recognized_card_id);
CREATE INDEX idx_captures_captured_at ON captures(captured_at);

-- ============================================
-- Users
-- ============================================
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255),
    display_name VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- API Keys
-- ============================================
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key_hash VARCHAR(255) NOT NULL,
    name VARCHAR(100) NOT NULL,
    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);

-- ============================================
-- Decks
-- ============================================
CREATE TABLE decks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    format VARCHAR(50),
    description TEXT,
    is_public BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_decks_user_id ON decks(user_id);
CREATE INDEX idx_decks_format ON decks(format);

-- ============================================
-- Deck Cards (many-to-many)
-- ============================================
CREATE TABLE deck_cards (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    deck_id UUID NOT NULL REFERENCES decks(id) ON DELETE CASCADE,
    card_id UUID NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_sideboard BOOLEAN DEFAULT FALSE,
    is_commander BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_deck_cards_deck_id ON deck_cards(deck_id);
CREATE INDEX idx_deck_cards_card_id ON deck_cards(card_id);
CREATE UNIQUE INDEX idx_deck_cards_unique ON deck_cards(deck_id, card_id, is_sideboard);

-- ============================================
-- Game Sessions (for tracking games)
-- ============================================
CREATE TABLE game_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    deck_id UUID REFERENCES decks(id) ON DELETE SET NULL,
    format VARCHAR(50),
    opponent_name VARCHAR(100),
    result VARCHAR(20),  -- 'win', 'loss', 'draw'
    notes TEXT,
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_game_sessions_user_id ON game_sessions(user_id);
CREATE INDEX idx_game_sessions_deck_id ON game_sessions(deck_id);

-- ============================================
-- Updated at trigger function
-- ============================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply trigger to tables with updated_at
CREATE TRIGGER update_cards_updated_at BEFORE UPDATE ON cards
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_decks_updated_at BEFORE UPDATE ON decks
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
