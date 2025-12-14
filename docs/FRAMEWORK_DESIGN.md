# Card Game AI Framework - Design Document

## Overview

This project is designed as a **reusable framework** for building AI-powered assistants for card and board games. Magic: The Gathering (MTG) is the first implementation, but the architecture supports swapping in any card-based or board game.

---

## Framework vs Game Separation

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         FRAMEWORK LAYER                                  │
│                    (Game-Agnostic Components)                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐ │
│  │   Capture    │  │ Recognition  │  │    Sync      │  │    Auth     │ │
│  │   Pipeline   │  │   Engine     │  │   Engine     │  │   System    │ │
│  └──────────────┘  └──────────────┘  └──────────────┘  └─────────────┘ │
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐ │
│  │   Storage    │  │     LLM      │  │    Push      │  │    CLI      │ │
│  │   Service    │  │   Gateway    │  │   Service    │  │  Framework  │ │
│  └──────────────┘  └──────────────┘  └──────────────┘  └─────────────┘ │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Implements Game Traits
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           GAME LAYER                                     │
│                    (Game-Specific Implementation)                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                    MTG Implementation                             │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────────────┐  │  │
│  │  │  Cards   │  │  Decks   │  │  Rules   │  │  Play Advisor   │  │  │
│  │  │  Module  │  │  Module  │  │  Engine  │  │     Module      │  │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └─────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                  Pokemon TCG Implementation                       │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────────────┐  │  │
│  │  │  Cards   │  │  Decks   │  │  Rules   │  │  Play Advisor   │  │  │
│  │  │  Module  │  │  Module  │  │  Engine  │  │     Module      │  │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └─────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                    Yu-Gi-Oh! Implementation                       │  │
│  │                          (Future)                                 │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Core Framework Traits

### 1. GamePiece Trait
Represents any identifiable game component (card, token, tile, etc.)

```rust
/// A game piece that can be recognized and tracked
pub trait GamePiece: Send + Sync {
    /// Unique identifier for this piece type
    type Id: Clone + Eq + Hash + Serialize + DeserializeOwned;
    
    /// Human-readable name
    fn name(&self) -> &str;
    
    /// Unique identifier
    fn id(&self) -> &Self::Id;
    
    /// Image URL for display
    fn image_url(&self) -> Option<&str>;
    
    /// Searchable text (name, description, etc.)
    fn searchable_text(&self) -> String;
    
    /// Category/type for filtering
    fn category(&self) -> &str;
}
```

### 2. GameCollection Trait
Represents a collection of pieces (deck, hand, board, etc.)

```rust
/// A collection of game pieces
pub trait GameCollection<P: GamePiece>: Send + Sync {
    /// Collection identifier
    fn id(&self) -> Uuid;
    
    /// Collection name
    fn name(&self) -> &str;
    
    /// Pieces in this collection
    fn pieces(&self) -> &[CollectionEntry<P>];
    
    /// Add a piece
    fn add(&mut self, piece_id: P::Id, quantity: u32);
    
    /// Remove a piece
    fn remove(&mut self, piece_id: P::Id, quantity: u32) -> bool;
    
    /// Validate collection against game rules
    fn validate(&self, rules: &dyn GameRules<P>) -> ValidationResult;
}

pub struct CollectionEntry<P: GamePiece> {
    pub piece_id: P::Id,
    pub quantity: u32,
    pub metadata: HashMap<String, Value>,
}
```

### 3. GameState Trait
Represents the current state of a game in progress

```rust
/// Current state of a game
pub trait GameState: Send + Sync + Clone {
    type Piece: GamePiece;
    type Player: GamePlayer;
    type Zone: GameZone;
    type Action: GameAction;
    
    /// Current turn number
    fn turn(&self) -> u32;
    
    /// Current phase/step
    fn phase(&self) -> &str;
    
    /// Active player
    fn active_player(&self) -> &Self::Player;
    
    /// All players
    fn players(&self) -> &[Self::Player];
    
    /// Pieces in a specific zone
    fn pieces_in_zone(&self, zone: &Self::Zone) -> Vec<&Self::Piece>;
    
    /// Legal actions from current state
    fn legal_actions(&self) -> Vec<Self::Action>;
    
    /// Apply an action to get new state
    fn apply_action(&self, action: &Self::Action) -> Result<Self, GameError>;
    
    /// Serialize state for LLM context
    fn to_llm_context(&self) -> String;
}
```

### 4. GameRules Trait
Encapsulates game-specific rules

```rust
/// Game rules engine
pub trait GameRules<P: GamePiece>: Send + Sync {
    type State: GameState<Piece = P>;
    type Format: GameFormat;
    
    /// Validate a collection for a format
    fn validate_collection(
        &self,
        collection: &dyn GameCollection<P>,
        format: &Self::Format,
    ) -> ValidationResult;
    
    /// Check if an action is legal
    fn is_legal_action(
        &self,
        state: &Self::State,
        action: &<Self::State as GameState>::Action,
    ) -> bool;
    
    /// Get all legal actions
    fn legal_actions(&self, state: &Self::State) -> Vec<<Self::State as GameState>::Action>;
    
    /// Determine winner (if game is over)
    fn winner(&self, state: &Self::State) -> Option<&<Self::State as GameState>::Player>;
}
```

### 5. GameDataSource Trait
Provides game piece data from external sources

```rust
/// External data source for game pieces
#[async_trait]
pub trait GameDataSource<P: GamePiece>: Send + Sync {
    /// Fetch piece by ID
    async fn get_piece(&self, id: &P::Id) -> Result<P, DataSourceError>;
    
    /// Search pieces
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<P>, DataSourceError>;
    
    /// Autocomplete piece names
    async fn autocomplete(&self, prefix: &str) -> Result<Vec<String>, DataSourceError>;
    
    /// Get bulk data for offline use
    async fn bulk_download(&self) -> Result<Vec<P>, DataSourceError>;
    
    /// Check for updates since timestamp
    async fn updates_since(&self, since: DateTime<Utc>) -> Result<Vec<P>, DataSourceError>;
}
```

### 6. PlayAdvisor Trait
AI-powered play suggestions

```rust
/// AI play advisor
#[async_trait]
pub trait PlayAdvisor<S: GameState>: Send + Sync {
    /// Suggest best actions from current state
    async fn suggest_actions(
        &self,
        state: &S,
        context: &AdvisorContext,
    ) -> Result<Vec<ActionSuggestion<S::Action>>, AdvisorError>;
    
    /// Evaluate a potential action
    async fn evaluate_action(
        &self,
        state: &S,
        action: &S::Action,
    ) -> Result<ActionEvaluation, AdvisorError>;
    
    /// Explain reasoning for a suggestion
    async fn explain(
        &self,
        state: &S,
        suggestion: &ActionSuggestion<S::Action>,
    ) -> Result<String, AdvisorError>;
}

pub struct ActionSuggestion<A> {
    pub action: A,
    pub confidence: f64,
    pub reasoning: String,
    pub priority: u32,
}
```

### 7. RecognitionProvider Trait
Game-specific piece recognition

```rust
/// Recognizes game pieces from images
#[async_trait]
pub trait RecognitionProvider<P: GamePiece>: Send + Sync {
    /// Recognize piece from image
    async fn recognize(
        &self,
        image: &ProcessedImage,
    ) -> Result<RecognitionResult<P::Id>, RecognitionError>;
    
    /// Recognize multiple pieces (board state)
    async fn recognize_board(
        &self,
        image: &ProcessedImage,
    ) -> Result<Vec<BoardPieceRecognition<P::Id>>, RecognitionError>;
    
    /// Get piece name dictionary for fuzzy matching
    fn piece_dictionary(&self) -> &[(String, P::Id)];
}

pub struct BoardPieceRecognition<Id> {
    pub piece_id: Id,
    pub confidence: f64,
    pub bounding_box: BoundingBox,
    pub state: HashMap<String, Value>,  // tapped, counters, etc.
}
```

---

## Crate Structure

```
cardgame-ai/                        # Renamed from mtg-ai-suite
├── crates/
│   │
│   │   # ═══════════════════════════════════════════════════════
│   │   # FRAMEWORK CRATES (Game-Agnostic)
│   │   # ═══════════════════════════════════════════════════════
│   │
│   ├── cardgame-core/              # Core traits and types
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── traits/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── piece.rs        # GamePiece trait
│   │   │   │   ├── collection.rs   # GameCollection trait
│   │   │   │   ├── state.rs        # GameState trait
│   │   │   │   ├── rules.rs        # GameRules trait
│   │   │   │   ├── datasource.rs   # GameDataSource trait
│   │   │   │   ├── advisor.rs      # PlayAdvisor trait
│   │   │   │   └── recognition.rs  # RecognitionProvider trait
│   │   │   ├── types/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── image.rs        # ProcessedImage, BoundingBox
│   │   │   │   ├── recognition.rs  # RecognitionResult
│   │   │   │   └── validation.rs   # ValidationResult
│   │   │   └── error.rs
│   │   └── Cargo.toml
│   │
│   ├── cardgame-capture/           # Image capture pipeline
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── camera.rs           # Camera access
│   │   │   ├── preprocessing.rs    # Image preprocessing
│   │   │   └── ocr.rs              # OCR integration
│   │   └── Cargo.toml
│   │
│   ├── cardgame-recognition/       # Recognition engine
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── fuzzy.rs            # Fuzzy matching
│   │   │   ├── embedding.rs        # Vector embeddings
│   │   │   └── pipeline.rs         # Recognition pipeline
│   │   └── Cargo.toml
│   │
│   ├── cardgame-llm/               # LLM integration
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── providers/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── openai.rs
│   │   │   │   ├── anthropic.rs
│   │   │   │   └── ollama.rs
│   │   │   ├── prompt.rs           # Prompt templates
│   │   │   └── context.rs          # Context building
│   │   └── Cargo.toml
│   │
│   ├── cardgame-server/            # API server framework
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── api/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── captures.rs     # Generic capture endpoints
│   │   │   │   ├── collections.rs  # Generic collection endpoints
│   │   │   │   ├── games.rs        # Generic game state endpoints
│   │   │   │   └── auth.rs         # Authentication
│   │   │   ├── services/
│   │   │   └── middleware/
│   │   └── Cargo.toml
│   │
│   ├── cardgame-cli/               # CLI framework
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── commands/
│   │   └── Cargo.toml
│   │
│   ├── cardgame-wasm/              # WASM framework
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   │   # ═══════════════════════════════════════════════════════
│   │   # GAME IMPLEMENTATIONS
│   │   # ═══════════════════════════════════════════════════════
│   │
│   ├── games/
│   │   │
│   │   ├── mtg/                    # Magic: The Gathering
│   │   │   ├── src/
│   │   │   │   ├── lib.rs
│   │   │   │   ├── card.rs         # impl GamePiece for MtgCard
│   │   │   │   ├── deck.rs         # impl GameCollection for MtgDeck
│   │   │   │   ├── state.rs        # impl GameState for MtgGameState
│   │   │   │   ├── rules.rs        # impl GameRules for MtgRules
│   │   │   │   ├── scryfall.rs     # impl GameDataSource for Scryfall
│   │   │   │   ├── advisor.rs      # impl PlayAdvisor for MtgAdvisor
│   │   │   │   └── recognition.rs  # impl RecognitionProvider
│   │   │   ├── Cargo.toml
│   │   │   └── tests/
│   │   │
│   │   ├── pokemon/                # Pokemon TCG (example)
│   │   │   ├── src/
│   │   │   │   ├── lib.rs
│   │   │   │   ├── card.rs         # impl GamePiece for PokemonCard
│   │   │   │   ├── deck.rs
│   │   │   │   ├── state.rs
│   │   │   │   ├── rules.rs
│   │   │   │   ├── pokemontcg.rs   # impl GameDataSource for PokemonTCG API
│   │   │   │   └── advisor.rs
│   │   │   └── Cargo.toml
│   │   │
│   │   └── template/               # Template for new games
│   │       ├── src/
│   │       │   ├── lib.rs
│   │       │   └── ...
│   │       ├── Cargo.toml
│   │       └── README.md           # How to implement a new game
│   │
│   │   # ═══════════════════════════════════════════════════════
│   │   # BINARIES (Use framework + game implementation)
│   │   # ═══════════════════════════════════════════════════════
│   │
│   ├── mtg-server/                 # MTG-specific server binary
│   │   ├── src/main.rs
│   │   └── Cargo.toml              # Depends on cardgame-server + games/mtg
│   │
│   ├── mtg-cli/                    # MTG-specific CLI binary
│   │   ├── src/main.rs
│   │   └── Cargo.toml
│   │
│   └── mtg-wasm/                   # MTG-specific WASM binary
│       ├── src/lib.rs
│       └── Cargo.toml
│
├── client/                         # Frontend (game-selectable)
│   ├── src/
│   │   ├── games/
│   │   │   ├── mtg/               # MTG-specific UI components
│   │   │   ├── pokemon/           # Pokemon-specific UI components
│   │   │   └── index.ts           # Game registry
│   │   ├── framework/             # Shared UI components
│   │   └── ...
│   └── ...
│
└── ...
```

---

## Game Configuration

Each game implementation provides a configuration:

```rust
/// Game configuration
pub struct GameConfig {
    /// Unique game identifier
    pub id: &'static str,
    
    /// Display name
    pub name: &'static str,
    
    /// Game description
    pub description: &'static str,
    
    /// Icon/logo URL
    pub icon_url: Option<&'static str>,
    
    /// Supported formats
    pub formats: Vec<FormatConfig>,
    
    /// Data source configuration
    pub data_source: DataSourceConfig,
    
    /// Recognition configuration
    pub recognition: RecognitionConfig,
    
    /// LLM prompt templates
    pub prompts: PromptTemplates,
}

// Example: MTG configuration
pub const MTG_CONFIG: GameConfig = GameConfig {
    id: "mtg",
    name: "Magic: The Gathering",
    description: "The world's greatest trading card game",
    icon_url: Some("https://..."),
    formats: vec![
        FormatConfig { id: "standard", name: "Standard", min_deck: 60, max_deck: None },
        FormatConfig { id: "modern", name: "Modern", min_deck: 60, max_deck: None },
        FormatConfig { id: "commander", name: "Commander", min_deck: 100, max_deck: Some(100) },
    ],
    data_source: DataSourceConfig {
        provider: "scryfall",
        base_url: "https://api.scryfall.com",
        bulk_endpoint: "/bulk-data",
    },
    recognition: RecognitionConfig {
        name_region: (0.0, 0.0, 1.0, 0.15),  // Top 15% of card
        min_confidence: 0.7,
    },
    prompts: PromptTemplates {
        play_suggestion: include_str!("prompts/play_suggestion.txt"),
        deck_analysis: include_str!("prompts/deck_analysis.txt"),
        rules_question: include_str!("prompts/rules_question.txt"),
    },
};
```

---

## Adding a New Game

### Step 1: Create Game Crate

```bash
cargo new crates/games/my-game --lib
```

### Step 2: Implement Core Traits

```rust
// crates/games/my-game/src/lib.rs

use cardgame_core::prelude::*;

pub struct MyGameCard { ... }
impl GamePiece for MyGameCard { ... }

pub struct MyGameDeck { ... }
impl GameCollection<MyGameCard> for MyGameDeck { ... }

pub struct MyGameState { ... }
impl GameState for MyGameState { ... }

pub struct MyGameRules { ... }
impl GameRules<MyGameCard> for MyGameRules { ... }

pub struct MyGameDataSource { ... }
impl GameDataSource<MyGameCard> for MyGameDataSource { ... }

pub struct MyGameAdvisor { ... }
impl PlayAdvisor<MyGameState> for MyGameAdvisor { ... }
```

### Step 3: Register Game

```rust
// In game registry
pub fn register_games(registry: &mut GameRegistry) {
    registry.register::<mtg::MtgGame>("mtg");
    registry.register::<pokemon::PokemonGame>("pokemon");
    registry.register::<my_game::MyGame>("my-game");  // Add new game
}
```

### Step 4: Add UI Components (Optional)

```typescript
// client/src/games/my-game/index.ts
export const MyGameComponents = {
  CardView: MyGameCardView,
  DeckEditor: MyGameDeckEditor,
  GameTracker: MyGameTracker,
};

// Register in client/src/games/index.ts
export const GAMES = {
  mtg: MtgComponents,
  pokemon: PokemonComponents,
  'my-game': MyGameComponents,
};
```

---

## Example: Pokemon TCG Implementation

```rust
// crates/games/pokemon/src/card.rs

use cardgame_core::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonCard {
    pub id: String,
    pub name: String,
    pub supertype: PokemonSupertype,  // Pokemon, Trainer, Energy
    pub subtypes: Vec<String>,
    pub hp: Option<u32>,
    pub types: Vec<PokemonType>,
    pub attacks: Vec<Attack>,
    pub weaknesses: Vec<Weakness>,
    pub retreat_cost: Vec<PokemonType>,
    pub image_url: String,
}

impl GamePiece for PokemonCard {
    type Id = String;
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn id(&self) -> &Self::Id {
        &self.id
    }
    
    fn image_url(&self) -> Option<&str> {
        Some(&self.image_url)
    }
    
    fn searchable_text(&self) -> String {
        format!("{} {} {:?}", self.name, self.supertype, self.subtypes)
    }
    
    fn category(&self) -> &str {
        match self.supertype {
            PokemonSupertype::Pokemon => "Pokemon",
            PokemonSupertype::Trainer => "Trainer",
            PokemonSupertype::Energy => "Energy",
        }
    }
}
```

---

## Potential Games to Support

### Trading Card Games

| Game | Complexity | Data Source | Notes |
|------|------------|-------------|-------|
| **Magic: The Gathering** | High | Scryfall API | First implementation |
| **Pokemon TCG** | Medium | pokemontcg.io | Good second target |
| **Yu-Gi-Oh!** | High | YGOProDeck API | Complex rules |
| **Flesh and Blood** | Medium | fabdb.net | Growing community |
| **Lorcana** | Low | Community APIs | New game |
| **KeyForge** | Medium | keyforgegame.com | Unique deck system |
| **Hearthstone** | Medium | HearthstoneJSON | Digital-first |
| **Marvel Snap** | Low | snap.fan | Simple rules |
| **Dominion** | Medium | - | Deck builder |

### Classic Board Games

| Game | Complexity | Data Source | Notes |
|------|------------|-------------|-------|
| **Chess** | Low | - | Perfect information, well-studied AI |
| **Go** | Medium | - | Simple rules, deep strategy |
| **Checkers** | Low | - | Good starter project |
| **Backgammon** | Low | - | Dice + strategy |

### Word & Puzzle Games

| Game | Complexity | Piece Type | AI Challenge |
|------|------------|------------|--------------|
| **Scrabble** | Medium | Letter tiles | Vocabulary + board optimization |
| **Crossword** | Medium | Clue/Answer pairs | NLP for clue interpretation |
| **Wordle** | Low | Letters | Information theory, elimination |
| **Sudoku** | Low | Numbers 1-9 | Constraint satisfaction |
| **Boggle** | Low | Letter grid | Word search + pathfinding |

### Casino & Betting Games

| Game | Complexity | Piece Type | AI Challenge |
|------|------------|------------|--------------|
| **Poker (Texas Hold'em)** | High | Cards + chips | Incomplete info, bluffing, GTO |
| **Blackjack** | Low | Cards | Card counting, basic strategy |
| **Bridge** | High | Cards | Partnership, bidding conventions |

### How These Fit the Framework

#### Scrabble Example
```rust
struct ScrabbleTile {
    letter: char,
    point_value: u8,
}

impl GamePiece for ScrabbleTile {
    type Id = char;
    fn name(&self) -> &str { /* letter as string */ }
    fn category(&self) -> &str { "tile" }
}

// GameState tracks: board, racks, bag, scores
// PlayAdvisor suggests: highest-scoring valid words
// Recognition: OCR tile letters from board photo
```

#### Poker Example
```rust
struct PlayingCard {
    rank: Rank,    // 2-10, J, Q, K, A
    suit: Suit,    // ♠♥♦♣
}

impl GamePiece for PlayingCard {
    type Id = (Rank, Suit);
    fn category(&self) -> &str { "card" }
}

// GameState tracks: community cards, hole cards, pot, positions, stack sizes
// PlayAdvisor suggests: fold/call/raise with EV calculations
// Recognition: Card detection from table photo
```

#### Sudoku Example
```rust
struct SudokuCell {
    value: Option<u8>,  // 1-9 or empty
    position: (u8, u8), // row, col
    is_given: bool,     // original clue vs solved
}

impl GamePiece for SudokuCell {
    type Id = (u8, u8);  // position
    fn category(&self) -> &str { 
        if self.is_given { "given" } else { "solved" }
    }
}

// GameState tracks: 9x9 grid, candidates per cell
// PlayAdvisor suggests: next cell to fill, technique to use
// Recognition: OCR numbers from puzzle photo
```

#### Crossword Example
```rust
struct CrosswordClue {
    number: u16,
    direction: Direction,  // Across or Down
    clue_text: String,
    answer: String,
    cells: Vec<(u8, u8)>,
}

impl GamePiece for CrosswordClue {
    type Id = (u16, Direction);
    fn category(&self) -> &str { 
        match self.direction {
            Direction::Across => "across",
            Direction::Down => "down",
        }
    }
}

// GameState tracks: grid, filled letters, solved clues
// PlayAdvisor suggests: answers using NLP + crossword databases
// Recognition: OCR grid and clue list from photo
```

### Framework Applicability Matrix

| Feature | TCGs | Board Games | Word Games | Card Games |
|---------|------|-------------|------------|------------|
| **GamePiece** | Cards | Pieces/Tiles | Letters/Clues | Cards |
| **GameCollection** | Decks | - | Rack/Hand | Hand |
| **GameState** | Complex | Medium | Simple | Medium |
| **Recognition** | Card OCR | Board vision | Grid OCR | Card detection |
| **PlayAdvisor** | LLM + rules | Search/MCTS | Dictionary + NLP | GTO + EV |
| **DataSource** | Card APIs | - | Dictionaries | - |

### Design Considerations by Game Type

| Game Type | Key Challenge | Framework Strength |
|-----------|---------------|-------------------|
| **TCGs** | Complex rules, large card pools | Recognition, data sync |
| **Board Games** | State representation, move generation | State traits, rules engine |
| **Word Games** | NLP, dictionary lookup | LLM integration, fuzzy matching |
| **Poker/Casino** | Probability, opponent modeling | Advisor traits, state tracking |
| **Puzzles** | Constraint solving | State representation, solver integration |

---

## Benefits of Framework Approach

1. **Code Reuse**: 70%+ of code is game-agnostic
2. **Faster Development**: New games only need trait implementations
3. **Consistent UX**: Same patterns across all games
4. **Shared Improvements**: Bug fixes benefit all games
5. **Community Contributions**: Easy to add new games
6. **Testing**: Framework tests apply to all games
7. **Documentation**: One set of docs for framework

---

## Migration Path

### Current State → Framework

1. **Phase 1**: Extract traits from MTG implementation
2. **Phase 2**: Refactor MTG code to implement traits
3. **Phase 3**: Create `cardgame-*` framework crates
4. **Phase 4**: Move MTG code to `games/mtg`
5. **Phase 5**: Add second game (Pokemon) to validate design
6. **Phase 6**: Create game template and documentation
