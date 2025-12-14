import { Octokit } from '@octokit/rest'

let connectionSettings;

async function getAccessToken() {
  if (connectionSettings && connectionSettings.settings.expires_at && new Date(connectionSettings.settings.expires_at).getTime() > Date.now()) {
    return connectionSettings.settings.access_token;
  }
  
  const hostname = process.env.REPLIT_CONNECTORS_HOSTNAME
  const xReplitToken = process.env.REPL_IDENTITY 
    ? 'repl ' + process.env.REPL_IDENTITY 
    : process.env.WEB_REPL_RENEWAL 
    ? 'depl ' + process.env.WEB_REPL_RENEWAL 
    : null;

  if (!xReplitToken) {
    throw new Error('X_REPLIT_TOKEN not found for repl/depl');
  }

  connectionSettings = await fetch(
    'https://' + hostname + '/api/v2/connection?include_secrets=true&connector_names=github',
    {
      headers: {
        'Accept': 'application/json',
        'X_REPLIT_TOKEN': xReplitToken
      }
    }
  ).then(res => res.json()).then(data => data.items?.[0]);

  const accessToken = connectionSettings?.settings?.access_token || connectionSettings.settings?.oauth?.credentials?.access_token;

  if (!connectionSettings || !accessToken) {
    throw new Error('GitHub not connected');
  }
  return accessToken;
}

async function getUncachableGitHubClient() {
  const accessToken = await getAccessToken();
  return new Octokit({ auth: accessToken });
}

const issues = [
  {
    title: "[Backend] Mount Python card routes to FastAPI app",
    body: `## Description
The card routes in \`backend/api/cards.py\` are defined but never connected to the main FastAPI app.

## Current State
- \`backend/api/cards.py\` has a router with card endpoints
- \`backend/app/main.py\` only has \`/\` and \`/health\` endpoints

## Required Changes
Add to \`backend/app/main.py\`:
\`\`\`python
from api.cards import router as cards_router
app.include_router(cards_router)
\`\`\`

## Priority
High - Blocks backend functionality

## Labels
bug, backend, priority-high`,
    labels: ["bug", "backend"]
  },
  {
    title: "[Backend] Add timestamp to health endpoint",
    body: `## Description
Per REPLIT_WORKLIST.md, the health endpoint should return a timestamp.

## Current Implementation
\`\`\`rust
Json(HealthResponse {
    status: "healthy".to_string(),
    version: env!("CARGO_PKG_VERSION").to_string(),
})
\`\`\`

## Expected Implementation
\`\`\`rust
Json(json!({
    "status": "healthy",
    "version": env!("CARGO_PKG_VERSION"),
    "timestamp": chrono::Utc::now().to_rfc3339()
}))
\`\`\`

## Files
- \`crates/mtg-server/src/api/health.rs\`
- \`backend/app/main.py\` (Python version)

## Priority
Medium`,
    labels: ["enhancement", "backend"]
  },
  {
    title: "[Backend] Implement card search with database or Scryfall proxy",
    body: `## Description
All Rust card endpoints return placeholder data. Need to implement actual card search.

## Current State
- \`list_cards\` returns empty array
- \`get_card\` returns placeholder
- \`autocomplete\` returns empty array

## Options
1. **SQLite Database** - Load Scryfall bulk data into local database
2. **Scryfall Proxy** - Forward requests to Scryfall API (like Python backend does)

## Files
- \`crates/mtg-server/src/api/cards.rs\`
- \`crates/mtg-core/src/fuzzy.rs\` (for fuzzy matching)

## Priority
High - Core functionality

## Related Tasks
- Create \`scripts/load_scryfall.py\` to load bulk data`,
    labels: ["enhancement", "backend"]
  },
  {
    title: "[Scripts] Create Scryfall data loader script",
    body: `## Description
Per REPLIT_WORKLIST.md Task 2.1, create a script to load Scryfall bulk data.

## Required
\`\`\`bash
python scripts/load_scryfall.py --bulk-type oracle_cards
\`\`\`

## Functionality
1. Download Scryfall bulk data (oracle_cards)
2. Parse JSON
3. Insert into SQLite database
4. Create indexes for search

## Priority
High - Required for card database`,
    labels: ["enhancement", "scripts"]
  },
  {
    title: "[Frontend] Connect to backend API instead of direct Scryfall calls",
    body: `## Description
The frontend currently calls Scryfall API directly. Should route through backend for:
- Rate limiting
- Caching
- Analytics
- Future features

## Current State
\`frontend/src/lib/scryfall.ts\` calls \`https://api.scryfall.com\` directly

## Required Changes
1. Update frontend to call backend API
2. Ensure backend proxies to Scryfall
3. Add error handling for backend unavailability

## Priority
Medium - Architecture improvement`,
    labels: ["enhancement", "frontend"]
  },
  {
    title: "[Frontend] Implement camera capture for card recognition",
    body: `## Description
Per REPLIT_WORKLIST.md Task 4.3, implement camera capture functionality.

## Requirements
- WebRTC camera access
- Capture button
- Send to recognition API (\`POST /api/v1/captures\`)
- Display recognition results

## Priority
Low - Phase 3 feature`,
    labels: ["enhancement", "frontend"]
  },
  {
    title: "[Backend] Implement OCR pipeline for card recognition",
    body: `## Description
Per REPLIT_WORKLIST.md Task 3.2, implement card recognition from images.

## Requirements
1. Extract text from image (Tesseract OCR or similar)
2. Use fuzzy matcher to find card name
3. Return recognition result with confidence

## Files
- \`crates/mtg-core/src/recognition.rs\`
- \`crates/mtg-core/src/image_processing.rs\`

## Priority
Low - Phase 3 feature`,
    labels: ["enhancement", "backend"]
  }
];

async function createIssues() {
  const octokit = await getUncachableGitHubClient();
  
  const { data: user } = await octokit.users.getAuthenticated();
  console.log(`Authenticated as: ${user.login}`);
  
  const owner = 'zacharyelston';
  const repo = 'mtg-ai-suite';
  
  console.log(`Creating issues in ${owner}/${repo}...`);
  
  for (const issue of issues) {
    try {
      const { data } = await octokit.issues.create({
        owner,
        repo,
        title: issue.title,
        body: issue.body,
      });
      console.log(`Created: #${data.number} - ${issue.title}`);
    } catch (error) {
      console.error(`Failed to create: ${issue.title}`, error.message);
    }
  }
  
  console.log('Done!');
}

createIssues().catch(console.error);
