# Game Implementation Template

This template shows how to add a new game to the Card Game AI Framework.

## Quick Start

1. Copy this directory: `cp -r template my-game`
2. Update `Cargo.toml` with your game name
3. Implement the core traits in `src/lib.rs`
4. Add your game to the workspace in root `Cargo.toml`
5. Register your game in the game registry

## Required Trait Implementations

| Trait | Purpose | Difficulty |
|-------|---------|------------|
| `GamePiece` | Define your card/piece type | Easy |
| `GameCollection` | Define deck/hand structure | Easy |
| `GameState` | Track game in progress | Medium |
| `GameRules` | Implement game rules | Hard |
| `GameDataSource` | Connect to card database API | Medium |
| `RecognitionProvider` | Card image recognition | Medium |
| `PlayAdvisor` | AI play suggestions | Hard |

## Example: Pokemon TCG

```rust
// Card type
impl GamePiece for PokemonCard {
    type Id = String;
    fn name(&self) -> &str { &self.name }
    fn category(&self) -> &str {
        match self.supertype {
            Supertype::Pokemon => "pokemon",
            Supertype::Trainer => "trainer", 
            Supertype::Energy => "energy",
        }
    }
}

// Data source
impl GameDataSource<PokemonCard> for PokemonTcgApi {
    async fn search(&self, query: &str, options: SearchOptions) -> Result<SearchResult<PokemonCard>> {
        // Call pokemontcg.io API
    }
}
```

## Testing

```bash
cargo test -p my-game
```
